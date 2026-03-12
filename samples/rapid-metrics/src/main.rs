// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::path::PathBuf;
use std::{env, thread};

use fastrand::Rng;
use tempfile::Builder;

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

    let flags = xflags::parse_or_exit! {
        /// Number of operations (default: 100)
        optional -n,--maxn maxn: usize
        /// Random seed (default: none)
        optional -s,--seed seed: u64
        /// Number of threads (default: 4)
        optional -t,--threads threads: usize
        /// Data directory (default: simple-db)
        optional path: PathBuf
    };

    let data_path = flags.path.unwrap_or_else(|| {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    });

    let maxn: usize = flags.maxn.unwrap_or(100);
    let seed: Option<u64> = flags.seed;
    let nthreads = flags.threads.unwrap_or(4);

    log::info!("Data path: {}", data_path.display());
    log::info!("Iterations: {maxn}");
    log::info!("Seed: {seed:?}");
    log::info!("Threads: {nthreads}");

    let mut rng = match seed {
        Some(seed) => {
            log::info!("Setting seed: {seed}");
            Rng::with_seed(seed)
        }
        _ => Rng::new(),
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

    thread::scope(|s| {
        for tid in 0..nthreads {
            let thread_seed = rng.u64(..);
            thread::Builder::new()
                .name(format!("t-{tid}"))
                .spawn_scoped(s, move || {
                    log::info!("thread={tid}, thread seed={thread_seed}");
                    let mut rng = Rng::with_seed(thread_seed);

                    let mut timer_id = None;
                    let mut timer_id_start = 0;

                    for i in 0..maxn {
                        let dice = rng.u32(0..6);

                        if let Some(timer_id) = timer_id.take() {
                            if (i - timer_id_start) == 10 {
                                log::info!("Got timer. Stopping timings after 10 rounds.");
                                glean_metrics::test_metrics::timings.stop_and_accumulate(timer_id);
                            }
                        }

                        match dice {
                            0 => {
                                let val = rng.bool();
                                log::info!("Setting bool={val}");
                                glean_metrics::test_metrics::sample_boolean.set(val);
                            }
                            1 => {
                                let val = rng.i32(0..2);
                                log::info!("Adding counter={val}");
                                glean_metrics::test_metrics::sample_counter.add(val);
                            }
                            2 => {
                                let val = rng.i32(1..101);
                                log::info!("Setting dual-labeld val={val}");
                                glean_metrics::test_metrics::static_static
                                    .get("key1", "category1")
                                    .add(val);
                            }
                            3 => {
                                let val = format!("cola-{}", rng.u8(0..100));
                                log::info!("Setting string val={val}");
                                glean_metrics::test_metrics::sample_string.set(val);
                            }
                            4 => {
                                if let Some(timer_id) = timer_id.take() {
                                    log::info!("Got timer. Stopping timings.");
                                    glean_metrics::test_metrics::timings
                                        .stop_and_accumulate(timer_id);
                                }
                                log::info!("Starting timings.");
                                timer_id = Some(glean_metrics::test_metrics::timings.start());
                                timer_id_start = i;
                            }
                            5 => {
                                let key = format!("key-{}", rng.u8(0..16));
                                let cat = format!("cat-{}", rng.u8(0..16));
                                let val = rng.i32(1..101);

                                log::info!("Setting dual-labeld key={key} cat={cat} val={val}");
                                glean_metrics::test_metrics::dynamic_dynamic
                                    .get(key, cat)
                                    .add(val);
                            }
                            _ => unreachable!(),
                        }
                    }
                })
                .unwrap();
        }
    });

    glean::shutdown();
}
