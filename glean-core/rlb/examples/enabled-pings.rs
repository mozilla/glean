// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test that pings can be enabled/disabled at runtime.

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use flate2::read::GzDecoder;
use glean::net;
use glean::{ClientInfoMetrics, ConfigurationBuilder, TestGetValue};

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
            send_in_pings: vec!["one".into(), "two".into()],
            lifetime: Lifetime::Ping,
            disabled: false,
            ..Default::default()
        })
    });
}

mod pings {
    use glean::private::PingType;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static one: Lazy<PingType> = Lazy::new(|| {
        glean::private::PingType::new(
            "one",
            true,
            true,
            true,
            true,
            false,
            vec![],
            vec![],
            false,
            vec![],
        )
    });

    #[allow(non_upper_case_globals)]
    pub static two: Lazy<PingType> = Lazy::new(|| {
        glean::private::PingType::new(
            "two",
            true,
            true,
            true,
            true,
            false,
            vec![],
            vec![],
            false,
            vec![],
        )
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
        if upload_request.ping_name != "one" && upload_request.ping_name != "two" {
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
    One,
    Two,
    Both,
}

impl From<&str> for State {
    fn from(state: &str) -> Self {
        match state {
            "default" => State::One,
            "enable-both" => State::Both,
            "enable-only-two" => State::Two,
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

    _ = &*pings::one;
    _ = &*pings::two;

    let uploader = MovingUploader::new(data_path.clone());
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.enabled-pings")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(uploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    // Wait for init to finish,
    // otherwise we might be to quick with calling `shutdown`.
    let _ = metrics::boo.test_get_value(None);

    let state = State::from(&*state);
    match state {
        State::One => {
            pings::one.set_enabled(true);
            pings::two.set_enabled(false);
        }
        State::Two => {
            pings::one.set_enabled(false);
            pings::two.set_enabled(true);
        }
        State::Both => {
            pings::one.set_enabled(true);
            pings::two.set_enabled(true);
        }
    }
    metrics::boo.add(1);
    if matches!(state, State::One | State::Both) {
        assert_eq!(
            Some(1),
            metrics::boo.test_get_value(Some("one".to_string()))
        );
    }

    if matches!(state, State::Two | State::Both) {
        assert_eq!(
            Some(1),
            metrics::boo.test_get_value(Some("two".to_string()))
        );
    }

    pings::one.submit(None);
    assert!(metrics::boo
        .test_get_value(Some("one".to_string()))
        .is_none());

    if matches!(state, State::Two | State::Both) {
        assert_eq!(
            Some(1),
            metrics::boo.test_get_value(Some("two".to_string()))
        );
    }

    pings::two.submit(None);
    if matches!(state, State::One | State::Both) {
        assert!(metrics::boo
            .test_get_value(Some("one".to_string()))
            .is_none());
    }
    if matches!(state, State::Two | State::Both) {
        assert!(metrics::boo
            .test_get_value(Some("two".to_string()))
            .is_none());
    }

    std::thread::sleep(std::time::Duration::from_millis(100));
    glean::shutdown(); // Cleanly shut down at the end of the test.
}
