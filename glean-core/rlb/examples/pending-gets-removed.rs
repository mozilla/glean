// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Test that pings can be enabled/disabled at runtime.

use std::env;
use std::fs::{read_dir, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use glean::{net, ClientInfoMetrics, Configuration, ConfigurationBuilder, TestGetValue};
use serde_json::Value as JsonValue;

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
            send_in_pings: vec!["validation".into()],
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
    pub static validation: Lazy<PingType> = Lazy::new(|| {
        glean::private::PingType::new(
            "validation",
            true,
            true,
            true,
            true,
            true,
            vec![],
            vec![],
            true,
            vec![],
        )
    });

    #[allow(non_upper_case_globals)]
    pub static nofollows: Lazy<PingType> = Lazy::new(|| {
        glean::private::PingType::new(
            "nofollows",
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

// Define a fake uploader that sleeps.
#[derive(Debug)]
struct FakeUploader;

impl net::PingUploader for FakeUploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        // Recoverable upload failure, will be retried 3 times,
        // but then keeps the pending ping around.
        net::UploadResult::http_status(500)
    }
}

fn get_pings(pings_dir: &Path) -> Vec<(String, JsonValue, Option<JsonValue>)> {
    let Ok(entries) = read_dir(pings_dir) else {
        return vec![];
    };
    entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| match entry.file_type() {
            Ok(file_type) => file_type.is_file(),
            Err(_) => false,
        })
        .filter_map(|entry| File::open(entry.path()).ok())
        .filter_map(|file| {
            let mut lines = BufReader::new(file).lines();
            if let (Some(Ok(url)), Some(Ok(body)), Ok(metadata)) =
                (lines.next(), lines.next(), lines.next().transpose())
            {
                let parsed_metadata = metadata.map(|m| {
                    serde_json::from_str::<JsonValue>(&m).expect("metadata should be valid JSON")
                });
                if let Ok(parsed_body) = serde_json::from_str::<JsonValue>(&body) {
                    Some((url, parsed_body, parsed_metadata))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect()
}

fn get_queued_pings(data_path: &Path) -> Vec<(String, JsonValue, Option<JsonValue>)> {
    get_pings(&data_path.join("pending_pings"))
}

fn get_deletion_pings(data_path: &Path) -> Vec<(String, JsonValue, Option<JsonValue>)> {
    get_pings(&data_path.join("deletion_request"))
}

fn get_config(data_path: &Path, upload_enabled: bool) -> Configuration {
    ConfigurationBuilder::new(upload_enabled, data_path, "glean.pending-removed")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(FakeUploader)
        .build()
}

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = PathBuf::from(args.next().expect("need data path"));
    let state = args.next().unwrap_or_default();
    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    // Ensure this ping is always registered early.
    _ = &*pings::validation;
    pings::nofollows.set_enabled(true);

    match &state[..] {
        "1" => {
            assert_eq!(
                0,
                get_queued_pings(&data_path).len(),
                "no pending ping should exist before init"
            );

            let cfg = get_config(&data_path, true);
            glean::initialize(cfg, client_info);

            // Wait for init to finish.
            let _ = metrics::boo.test_get_value(None);

            pings::validation.submit(None);
            pings::nofollows.submit(None);
            glean::shutdown();

            assert_eq!(2, get_queued_pings(&data_path).len());
        }
        "2" => {
            assert_eq!(
                2,
                get_queued_pings(&data_path).len(),
                "two pending pings should exist before init"
            );

            let cfg = get_config(&data_path, false);
            glean::initialize(cfg, client_info);

            // Wait for init to finish.
            let _ = metrics::boo.test_get_value(None);

            assert_eq!(
                1,
                get_queued_pings(&data_path).len(),
                "one pending ping should exist after init"
            );
            assert_eq!(
                1,
                get_deletion_pings(&data_path).len(),
                "one deletion-request ping should exist after init"
            );
        }
        "3" => {
            assert_eq!(
                1,
                get_queued_pings(&data_path).len(),
                "one pending ping should exist before init"
            );
            assert_eq!(
                1,
                get_deletion_pings(&data_path).len(),
                "one deletion-request ping should exist before init (leftover from previous run)"
            );

            let cfg = get_config(&data_path, false);
            glean::initialize(cfg, client_info);

            pings::nofollows.set_enabled(false);

            // Wait for init to finish.
            let _ = metrics::boo.test_get_value(None);

            assert_eq!(
                0,
                get_queued_pings(&data_path).len(),
                "no pending ping should exist after ping is disabled"
            );
            assert_eq!(
                1,
                get_deletion_pings(&data_path).len(),
                "one deletion-request ping should exist after init (leftover from previous run)"
            );
        }
        _ => panic!("unknown state: {state:?}"),
    };
}
