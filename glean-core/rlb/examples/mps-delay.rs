// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test that the metrics ping is correctly scheduled and not repeated.
//! Driven by `test-mps-delay.sh` which sets a timezone and fakes the system time.
//!
//! Note: `libfaketime` behavior on macOS seems to be incorrect for the condvar wakeups.

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::Duration;
use std::{env, thread};

use flate2::read::GzDecoder;
use glean::net;
use glean::{ClientInfoMetrics, ConfigurationBuilder};

/// A timing_distribution
mod metrics {
    use glean::private::*;
    use glean::Lifetime;
    use glean_core::CommonMetricData;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static boo: Lazy<CounterMetric> = Lazy::new(|| {
        CounterMetric::new(CommonMetricData {
            name: "boo".into(),
            category: "sample".into(),
            send_in_pings: vec!["metrics".into()],
            lifetime: Lifetime::Ping,
            disabled: false,
            ..Default::default()
        })
    });
}

#[derive(Debug)]
struct MovingUploader(PathBuf);

impl MovingUploader {
    fn new(mut path: PathBuf) -> Self {
        path.push("sent_pings");
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        let upload_request = upload_request.capable(|_| true).unwrap();
        // Filter out uninteristing pings.
        if upload_request.ping_name != "metrics" {
            return net::UploadResult::http_status(200);
        }
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

        let docid = url.rsplit('/').next().unwrap();
        let out_path = self.0.join(format!("{docid}.json"));
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

#[derive(Debug, Clone, Copy)]
enum State {
    Init,
    Second,
    Third,
}

impl From<&str> for State {
    fn from(state: &str) -> Self {
        match state {
            "init" => State::Init,
            "second" => State::Second,
            "third" => State::Third,
            _ => {
                panic!("unknown argument: {state}");
            }
        }
    }
}

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = PathBuf::from(args.next().expect("need data path"));
    let state = args.next().unwrap_or_default();
    let state = State::from(&*state);
    log::info!("Runing state {state:?}");

    let uploader = MovingUploader::new(data_path.clone());
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.mps-delay")
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

    metrics::boo.add(1);

    // Wait for init to finish, otherwise the metrics increment above might be lost
    // before we call `shutdown`.
    thread::sleep(Duration::from_millis(100));

    glean::shutdown(); // Cleanly shut down at the end of the test.
}
