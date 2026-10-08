// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
#![cfg(feature = "sqlite")]

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{test_get_num_recorded_errors, ErrorType};
use glean_core::{CommonMetricData, Glean, LabeledMetricData, Lifetime};

fn new_glean_delayed_io() -> (Glean, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();

    let cfg = glean_core::InternalConfiguration {
        data_path: dir.path().display().to_string(),
        application_id: GLOBAL_APPLICATION_ID.into(),
        language_binding_name: "Rust".into(),
        upload_enabled: true,
        max_events: None,
        delay_ping_lifetime_io: true, // Differs from `new_glean`
        app_build: "Unknown".into(),
        use_core_mps: false,
        trim_data_to_registered_pings: false,
        log_level: None,
        rate_limit: None,
        enable_event_timestamps: false,
        experimentation_id: None,
        enable_internal_pings: true,
        ping_schedule: Default::default(),
        ping_lifetime_threshold: 0,
        ping_lifetime_max_time: 0,
        max_pending_pings_count: None,
        max_pending_pings_directory_size: None,
        session_mode: glean_core::SessionMode::Auto,
        session_sample_rate: 1.0,
        session_inactivity_timeout_ms: 1_800_000,
        events_ping_acceleration_factor: None,
        enable_store_submitted_pings: false,
    };
    let mut glean = Glean::new(cfg).unwrap();
    _ = new_test_ping(&mut glean, "store1");

    (glean, dir)
}

fn labeled_counter(lifetime: Lifetime) -> LabeledCounter {
    LabeledCounter::new(
        LabeledMetricData::Common {
            cmd: CommonMetricData {
                name: "labeled_metric".into(),
                category: "telemetry".into(),
                send_in_pings: vec!["store1".into()],
                disabled: false,
                lifetime,
                ..Default::default()
            },
        },
        None,
    )
}

/// Errors for non-ping-lifetime metrics are recorded through `record_error_sqlite`,
/// which writes the (ping-lifetime) error counter via `record_with_transaction`.
/// With delayed ping-lifetime IO, that counter must land in the in-memory cache,
/// otherwise it never shows up in the `metrics` ping.
#[test]
fn errors_for_user_lifetime_metrics_are_visible_with_delayed_io() {
    let (glean, _t) = new_glean_delayed_io();

    let labeled = labeled_counter(Lifetime::User);
    let metric = labeled.get("1".repeat(112));
    metric.add_sync(&glean, 1);

    assert_eq!(
        Ok(1),
        test_get_num_recorded_errors(&glean, metric.meta(), ErrorType::InvalidLabel)
    );

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "metrics", false)
        .expect("metrics ping should not be empty");
    assert_eq!(
        1,
        snapshot["labeled_counter"]["glean.error.invalid_label"]["telemetry.labeled_metric"],
        "snapshot: {snapshot}"
    );
}

/// Dynamic labels may contain `|`, which the in-memory cache uses as its key separator.
#[test]
fn dynamic_label_with_pipe_does_not_panic_with_delayed_io() {
    let (glean, _t) = new_glean_delayed_io();

    let labeled = labeled_counter(Lifetime::Ping);
    labeled.get("a|b").add_sync(&glean, 1);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", false)
        .unwrap();
    assert_eq!(
        1,
        snapshot["labeled_counter"]["telemetry.labeled_metric"]["a|b"],
        "snapshot: {snapshot}"
    );
}
