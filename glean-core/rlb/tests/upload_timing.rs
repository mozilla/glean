// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

mod common;

use std::io::Read;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time;

use crossbeam_channel::{bounded, Sender};
use flate2::read::GzDecoder;
use serde_json::Value as JsonValue;

use glean::net;
use glean::ConfigurationBuilder;

pub mod metrics {
    use glean::{private::BooleanMetric, CommonMetricData, Lifetime};

    #[allow(non_upper_case_globals)]
    pub static sample_boolean: once_cell::sync::Lazy<BooleanMetric> =
        once_cell::sync::Lazy::new(|| {
            BooleanMetric::new(CommonMetricData {
                name: "sample_boolean".into(),
                category: "test.metrics".into(),
                send_in_pings: vec!["validation".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            })
        });
}

mod pings {
    use glean::private::PingType;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static validation: Lazy<PingType> =
        Lazy::new(|| glean::private::PingType::new("validation", true, true, vec![]));

    // metrics ping replica
    #[allow(non_upper_case_globals)]
    pub static metrics: Lazy<PingType> = Lazy::new(|| {
        glean::private::PingType::new(
            "metrics",
            true,
            false,
            vec![
                "overdue".to_string(),
                "reschedule".to_string(),
                "today".to_string(),
                "tomorrow".to_string(),
                "upgrade".to_string(),
            ],
        )
    });
}

// Define a fake uploader that sleeps.
#[derive(Debug)]
struct FakeUploader {
    calls: AtomicUsize,
    sender: Sender<JsonValue>,
}

impl net::PingUploader for FakeUploader {
    fn upload(
        &self,
        _url: String,
        body: Vec<u8>,
        _headers: Vec<(String, String)>,
    ) -> net::UploadResult {
        let calls = self.calls.fetch_add(1, Ordering::SeqCst);
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

        match calls {
            // First goes through as is.
            0 => net::UploadResult::http_status(200),
            // Second briefly sleeps
            1 => {
                thread::sleep(time::Duration::from_millis(100));
                net::UploadResult::http_status(200)
            }
            // Third one fails
            2 => net::UploadResult::http_status(404),
            // Fourth one fast again
            3 => {
                self.sender.send(decode(body)).unwrap();
                net::UploadResult::http_status(200)
            }
            // Last one is the metrics ping, a-ok.
            _ => {
                self.sender.send(decode(body)).unwrap();
                net::UploadResult::http_status(200)
            }
        }
    }
}

/// Test scenario: Different timings for upload on success and failure.
///
/// The app is initialized, in turn Glean gets initialized without problems.
/// A custom ping is submitted multiple times to trigger upload.
/// A metrics ping is submitted to get the upload timing data.
///
/// And later the whole process is shutdown.
#[test]
fn upload_timings() {
    common::enable_test_logging();

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();
    let (tx, rx) = bounded(1);

    let cfg = ConfigurationBuilder::new(true, tmpname, "glean-upload-timing")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(FakeUploader {
            calls: AtomicUsize::new(0),
            sender: tx,
        })
        .build();
    common::initialize(cfg);

    // Wait for init to finish,
    // otherwise we might be to quick with calling `shutdown`.
    let _ = metrics::sample_boolean.test_get_value(None);

    // fast
    pings::validation.submit(None);
    // slow
    pings::validation.submit(None);
    // failed
    pings::validation.submit(None);
    // fast
    pings::validation.submit(None);

    // wait for the last ping
    let _body = rx.recv().unwrap();

    pings::metrics.submit(None);

    let body = rx.recv().unwrap();

    let count = |dist: &JsonValue| {
        dist["values"]
            .as_object()
            .expect("an array of values")
            .iter()
            .map(|(_key, val)| val.as_i64().unwrap_or(0))
            .filter(|&val| val > 0)
            .sum::<i64>()
    };

    assert_eq!(
        3,
        count(&body["metrics"]["timing_distribution"]["glean.upload.sending_success"]),
        "Successful pings: two fast, one slow"
    );
    assert_eq!(
        1,
        count(&body["metrics"]["timing_distribution"]["glean.upload.sending_failure"]),
        "One failed pings"
    );

    glean::shutdown();
}
