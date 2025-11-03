// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

mod common;

use std::{fs, io::Read};

use crossbeam_channel::{bounded, Sender};
use flate2::read::GzDecoder;
use glean::{net, ConfigurationBuilder};
use serde_json::Value as JsonValue;

// Define a fake uploader that reports when and what it uploads.
#[derive(Debug)]
struct ReportingUploader {
    sender: Sender<JsonValue>,
}

impl net::PingUploader for ReportingUploader {
    fn upload(&self, upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        let upload_request = upload_request.capable(|_| true).unwrap();
        let body = upload_request.body;
        let decode = |body: Vec<u8>| {
            let mut gzip_decoder = GzDecoder::new(&body[..]);
            let mut s = String::with_capacity(body.len());

            gzip_decoder
                .read_to_string(&mut s)
                .ok()
                .map(|_| &s[..])
                .or_else(|| std::str::from_utf8(&body).ok())
                .and_then(|payload| serde_json::from_str(payload).ok())
                .unwrap()
        };

        self.sender.send(decode(body)).unwrap();
        net::UploadResult::http_status(200)
    }
}

/// Test scenario: Write a client ID to the backup file and check that it's used after initialization.
#[test]
fn test_pre_post_init_health_pings_exist() {
    common::enable_test_logging();

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    let client_id = "e03cc2de-bc8b-4f9c-862f-b474d910899e";

    // We write a random but fixed client ID, without there being a Glean database.
    let clientid_txt = tmpname.join("client_id.txt");
    fs::write(&clientid_txt, client_id.as_bytes()).unwrap();

    let (tx, rx) = bounded(1);
    let cfg = ConfigurationBuilder::new(true, tmpname.clone(), "health-ping-test")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(ReportingUploader { sender: tx })
        .build();
    common::initialize(cfg);

    glean_core::glean_test_destroy_glean(false, Some(tmpname.display().to_string()));

    // Check for the initialization pings.
    // Wait for the ping to arrive.
    let payload = rx.recv().unwrap();

    let exception_state = &payload["metrics"]["string"]["glean.health.exception_state"];
    assert_eq!("empty-db", exception_state);
    let exception_uuid = &payload["metrics"]["uuid"]["glean.health_recovered_client_id"];
    assert_eq!(&JsonValue::Null, exception_uuid);

    // TODO(bug 1996862): We don't run the mitigation yet.
    //let ping_client_id = &payload["client_info"]["client_id"];
    //assert_eq!(client_id, ping_client_id);
}
