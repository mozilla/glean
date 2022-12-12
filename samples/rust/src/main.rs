// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use tempfile::Builder;

use glean::{ClientInfoMetrics, Configuration};

pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
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

    let cfg = Configuration {
        data_path,
        application_id: "org.mozilla.glean_core.example".into(),
        upload_enabled: true,
        max_events: None,
        delay_ping_lifetime_io: false,
        server_endpoint: Some("invalid-test-host".into()),
        uploader: None,
        use_core_mps: true,
        trim_data_to_registered_pings: false,
    };

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
    };

    glean::initialize(cfg, client_info);

    glean_metrics::test_metrics::sample_boolean.set(true);

    glean_metrics::prototype.submit(None);
    // Need to wait a short time for Glean to actually act.
    thread::sleep(Duration::from_millis(100));

    glean::shutdown();
}
