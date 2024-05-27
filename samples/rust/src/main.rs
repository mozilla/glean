// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use tempfile::Builder;

use flate2::read::GzDecoder;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder};

pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
struct MovingUploader(String);

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
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

        let mut out_path = PathBuf::from(&self.0);
        out_path.push("sent_pings");
        std::fs::create_dir_all(&out_path).unwrap();

        let docid = url.rsplit('/').next().unwrap();
        out_path.push(format!("{docid}.json"));
        let mut fp = File::create(out_path).unwrap();

        // pseudo-JSON, let's hope this works.
        writeln!(fp, "{{").unwrap();
        writeln!(fp, "  \"url\": {url},").unwrap();
        for (key, val) in headers {
            writeln!(fp, "  \"{key}\": \"{val}\",").unwrap();
        }
        writeln!(fp, "}}").unwrap();
        writeln!(fp, "{data}").unwrap();

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

    let uploader = MovingUploader(data_path.display().to_string());
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

    fn rand() -> usize {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};

        let random_value = RandomState::new().build_hasher().finish() as usize;
        random_value % u32::max_value() as usize
    }

    _ = glean_metrics::test_metrics::sample_boolean.test_get_value(None);

    for _ in 0..1000 {
        glean_metrics::test::time.accumulate_single_sample(rand() as i64);
    }

    _ = glean_metrics::test_metrics::sample_boolean.test_get_value(None);

    glean_metrics::prototype.submit(None);
    // Need to wait a short time for Glean to actually act.
    thread::sleep(Duration::from_millis(100));

    glean::shutdown();
}
