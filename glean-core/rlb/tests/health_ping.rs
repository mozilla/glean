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
fn test_pre_post_init_health_pings_exist() {
    common::enable_test_logging();

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    let cfg = ConfigurationBuilder::new(true, tmpname.clone(), "health-ping-test")
        .with_server_endpoint("invalid-test-host")
        .with_uploader(FakeUploader)
        .build();
    common::initialize(cfg);

    glean::shutdown();

    // Check for the initialization pings.
    let pings = get_pings(&tmpname.join("pending_pings"));
    pings.iter().for_each(|(url, _, _)| {
        println!("Ping URL: {}", url);
    });
    assert!(!pings.is_empty());
    assert_eq!(
        2,
        pings
            .iter()
            .filter(|(url, _, _)| url.contains("health"))
            .count()
    );
    assert_eq!(
        1,
        pings
            .iter()
            .filter(|(url, body, _)| url.contains("health")
                && body.get("ping_info").unwrap().get("reason").unwrap() == "pre_init")
            .count()
    );
    assert_eq!(
        1,
        pings
            .iter()
            .filter(|(url, body, _)| url.contains("health")
                && body.get("ping_info").unwrap().get("reason").unwrap() == "post_init")
            .count()
    );
}
