// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::path::PathBuf;

use tempfile::Builder;
use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

use glean::{net, ClientInfoMetrics, ConfigurationBuilder};

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
struct MockUploader;

impl MockUploader {
    fn new() -> Self {
        Self
    }
}

impl net::PingUploader for MockUploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
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

    let maxn: usize = args
        .next()
        .unwrap_or_else(|| String::from("100"))
        .parse()
        .unwrap();
    let seed: Option<u64> = args.next().and_then(|a| a.parse().ok());

    log::info!("Iterations: {maxn}");
    log::info!("Seed: {seed:?}");

    let mut rng = match seed {
        Some(seed) => {
            log::info!("Setting seed: {seed}");
            ChaCha20Rng::seed_from_u64(seed)
        }
        _ => ChaCha20Rng::from_os_rng(),
    };

    let uploader = MockUploader::new();
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.rapid.metrics")
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

    _ = &*glean_metrics::prototype;
    _ = &*glean_metrics::usage_reporting;
    glean::initialize(cfg, client_info);

    for _ in 0..maxn {
        let coin = rng.random_range(0..4);

        match coin {
            0 => {
                let val = rng.random_range(0..2);
                log::info!("Setting bool={val}");
                glean_metrics::test_metrics::sample_boolean.set(val != 0);
            },
            1 => {
                let val = rng.random_range(0..2);
                log::info!("Adding counter={val}");
                glean_metrics::test_metrics::sample_counter.add(val);
            },
            2 => {
                let val = rng.random_range(1..101);
                log::info!("Setting dual-labeld val={val}");
                glean_metrics::test_metrics::static_static
                    .get("key1", "category1")
                    .add(val);
            },
            3 => {
                let val = format!("cola-{}", rng.random_range(0..100));
                log::info!("Setting string val={val}");
                glean_metrics::test_metrics::sample_string.set(val);
            },
            _ => unreachable!(),
        }
    }

    glean::shutdown();
}
