// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use tempfile::Builder;

use flate2::read::GzDecoder;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder, ErrorType};

pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
struct MovingUploader {
    out_path: String,
    recv_cnt: AtomicUsize,
}

impl MovingUploader {
    fn new(out_path: String) -> Self {
        let mut cnt = 0;

        let mut dir = PathBuf::from(&out_path);
        dir.push("sent_pings");
        if let Ok(entries) = dir.read_dir() {
            cnt = entries.filter_map(|entry| entry.ok()).count();
        }

        Self {
            out_path,
            recv_cnt: AtomicUsize::new(cnt),
        }
    }
}

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        let cnt = self.recv_cnt.fetch_add(1, Ordering::Relaxed) + 1;
        let net::PingUploadRequest {
            body, url, headers, ..
        } = upload_request;
        let mut gzip_decoder = GzDecoder::new(&body[..]);
        let mut s = String::with_capacity(body.len());

        let data = gzip_decoder
            .read_to_string(&mut s)
            .ok()
            .map(|_| &s[..])
            .or_else(|| std::str::from_utf8(&body).ok())
            .unwrap();

        let mut out_path = PathBuf::from(&self.out_path);
        out_path.push("sent_pings");
        std::fs::create_dir_all(&out_path).unwrap();

        let mut components = url.rsplit('/');
        let docid = components.next().unwrap();
        let _doc_version = components.next().unwrap();
        let doctype = components.next().unwrap();
        out_path.push(format!("{cnt:0>3}-{doctype}-{docid}.json"));
        let mut fp = File::create(out_path).unwrap();

        // pseudo-JSON, let's hope this works.
        writeln!(fp, "{{").unwrap();
        writeln!(fp, "  \"url\": {url:?},").unwrap();
        for (key, val) in headers {
            writeln!(fp, "  \"{key}\": \"{val}\",").unwrap();
        }
        writeln!(fp, "}}").unwrap();

        let data: serde_json::Value = serde_json::from_str(data).unwrap();
        let json = serde_json::to_string_pretty(&data).unwrap();
        writeln!(fp, "{json}").unwrap();

        net::UploadResult::http_status(200)
    }
}

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    };

    let uploader = MovingUploader::new(data_path.display().to_string());
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .with_uploader(uploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    glean_metrics::test_metrics::sample_boolean.set(true);

    use glean_metrics::party::{BalloonsObject, BalloonsObjectItem};
    let balloons = BalloonsObject::from([
        BalloonsObjectItem {
            colour: Some("red".to_string()),
            diameter: Some(5),
        },
        BalloonsObjectItem {
            colour: Some("blue".to_string()),
            diameter: None,
        },
    ]);
    glean_metrics::party::balloons.set(balloons);

    // Testing with empty and null values.
    let drinks = serde_json::json!([
        { "name": "lemonade", "ingredients": ["lemon", "water", "sugar"] },
        { "name": "sparkling-water", "ingredients": [] },
        { "name": "still-water", "ingredients": null },
    ]);
    glean_metrics::party::drinks.set_string(drinks.to_string());

    assert_eq!(
        0,
        glean_metrics::party::drinks.test_get_num_recorded_errors(ErrorType::InvalidValue)
    );

    {
        let mut buffer = glean_metrics::test_metrics::timings.start_buffer();

        let mock_duration = Duration::from_millis(10);
        for _ in 0..100 {
            buffer.accumulate(mock_duration.as_millis() as u64);
        }
    }

    glean_metrics::prototype.submit(None);
    // Need to wait a short time for Glean to actually act.
    thread::sleep(Duration::from_millis(100));

    glean::shutdown();
}
