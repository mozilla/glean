// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

mod common;

use std::{
    fs::{read_dir, File},
    io::{BufRead, BufReader},
    path::Path,
};

use glean::{net, ConfigurationBuilder};
use serde_json::Value as JsonValue;

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

/// Test scenario: After initialization, check that both pre-init and post-init health pings exist
///
/// The app is initialized, in turn Glean gets initialized without problems.
/// Some data is recorded (before and after initialization).
/// And later the whole process is shutdown.
#[test]
fn test_pre_init_health_pings_exist() {
    common::enable_test_logging();

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    let cfg = ConfigurationBuilder::new(true, tmpname.clone(), "health-ping-test")
        .with_server_endpoint("invalid-test-host")
        .with_uploader(FakeUploader)
        .build();
    common::initialize(cfg);

    glean_core::glean_test_destroy_glean(false, Some(tmpname.display().to_string()));

    // Check for the initialization pings.
    let pings = get_pings(&tmpname.join("pending_pings"));
    let health_pings: Vec<_> = pings
        .iter()
        .filter(|(url, _body, _)| url.contains("health"))
        .collect();
    assert_eq!(1, health_pings.len());

    let preinit = &health_pings[0];
    // Ensure both "health" pings have the same init count.
    assert_eq!(
        1,
        preinit.1["metrics"]["counter"]["glean.health.init_count"]
    );

    let exception_state = &preinit.1["metrics"]["string"]["glean.health_exception_state"];
    assert_eq!(&JsonValue::Null, exception_state);
    let exception_uuid = &preinit.1["metrics"]["uuid"]["glean.health_recovered_client_id"];
    assert_eq!(&JsonValue::Null, exception_uuid);

    let cfg = ConfigurationBuilder::new(true, tmpname.clone(), "health-ping-test")
        .with_server_endpoint("invalid-test-host")
        .with_uploader(FakeUploader)
        .build();
    common::initialize(cfg);

    glean::shutdown();

    let pings = get_pings(&tmpname.join("pending_pings"));
    let second_preinit: Vec<_> = pings
        .iter()
        .filter(|(url, body, _)| url.contains("health") && body["ping_info"]["seq"] == 1)
        .collect();

    // We should have a second "pre_init"-reason "health" ping now.
    assert_eq!(1, second_preinit.len());
}
