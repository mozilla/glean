// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use glean_core::StoredSubmittedPingHandler;
mod common;

use crate::common::*;
use chrono::Utc;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Lifetime};

#[test]
fn snapshot_returns_none_if_nothing_is_recorded_in_the_store() {
    let (glean, _t) = new_glean(None);
    assert!(StorageManager
        .snapshot(glean.storage(), "unknown_store", true)
        .is_none())
}

#[test]
fn can_snapshot() {
    let (glean, _t) = new_glean(None);

    let local_metric = StringMetric::new(CommonMetricData {
        name: "can_snapshot_local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["store1".into()],
        ..Default::default()
    });

    local_metric.set_sync(&glean, "snapshot 42");

    assert!(StorageManager
        .snapshot(glean.storage(), "store1", true)
        .is_some())
}

#[test]
fn snapshot_correctly_clears_the_stores() {
    let (glean, _t) = new_glean(None);
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = CounterMetric::new(CommonMetricData {
        name: "metric".into(),
        category: "telemetry".into(),
        send_in_pings: store_names,
        disabled: false,
        lifetime: Lifetime::Ping,
        ..Default::default()
    });

    metric.add_sync(&glean, 1);

    // Get the snapshot from "store1" and clear it.
    let snapshot = StorageManager.snapshot(glean.storage(), "store1", true);
    assert!(snapshot.is_some());
    // Check that getting a new snapshot for "store1" returns an empty store.
    assert!(StorageManager
        .snapshot(glean.storage(), "store1", false)
        .is_none());
    // Check that we get the right data from both the stores. Clearing "store1" must
    // not clear "store2" as well.
    let snapshot2 = StorageManager.snapshot(glean.storage(), "store2", true);
    assert!(snapshot2.is_some());
}

#[test]
fn storage_is_thread_safe() {
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    let (glean, _t) = new_glean(None);
    let glean = Arc::new(Mutex::new(glean));

    let threadsafe_metric = CounterMetric::new(CommonMetricData {
        name: "threadsafe".into(),
        category: "global".into(),
        send_in_pings: vec!["store1".into(), "metrics".into()],
        ..Default::default()
    });
    let threadsafe_metric = Arc::new(threadsafe_metric);

    let barrier = Arc::new(Barrier::new(2));
    let c = barrier.clone();
    let threadsafe_metric_clone = threadsafe_metric.clone();
    let glean_clone = glean.clone();
    let child = thread::spawn(move || {
        threadsafe_metric_clone.add_sync(&glean_clone.lock().unwrap(), 1);
        c.wait();
        threadsafe_metric_clone.add_sync(&glean_clone.lock().unwrap(), 1);
    });

    threadsafe_metric.add_sync(&glean.lock().unwrap(), 1);
    barrier.wait();
    threadsafe_metric.add_sync(&glean.lock().unwrap(), 1);

    child.join().unwrap();

    let snapshot = StorageManager
        .snapshot_as_json(glean.lock().unwrap().storage(), "store1", true)
        .unwrap();
    assert_eq!(json!({"counter": { "global.threadsafe": 4 }}), snapshot);
}

#[test]
fn test_storing_and_fetching_submitted_pings() {
    let (glean, _temp) = new_glean(None);

    let utc_time_one = chrono::DateTime::parse_from_rfc3339("2026-08-05T12:30:00.50Z")
        .unwrap()
        .to_utc();
    let utc_time_two = chrono::DateTime::parse_from_rfc3339("2026-08-05T12:30:00.51Z")
        .unwrap()
        .to_utc();
    let utc_time_three = chrono::DateTime::parse_from_rfc3339("2026-08-05T12:30:00.52Z")
        .unwrap()
        .to_utc();

    // First ping, no upload date
    glean
        .storage()
        .store_submitted_ping(
            "id",
            "ping",
            utc_time_one,
            None,
            None,
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    // Second ping, no upload date
    glean
        .storage()
        .store_submitted_ping(
            "id-one",
            "ping-two",
            utc_time_two,
            None,
            None,
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    // Second ping again, with upload date .01s after submitted date
    glean
        .storage()
        .store_submitted_ping(
            "id-one",
            "ping-two",
            utc_time_two,
            Some(utc_time_two),
            None,
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    // Third ping, upload failed
    glean
        .storage()
        .store_submitted_ping(
            "id-two",
            "ping-three",
            utc_time_three,
            None,
            Some(utc_time_three),
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    let all_pings = glean.storage().get_all_submitted_pings();
    assert_eq!(all_pings.len(), 3);
    assert_eq!(all_pings.last().unwrap().document_id, "id".to_string());
    assert_eq!(all_pings.get(1).unwrap().document_id, "id-one".to_string());
    assert_eq!(all_pings.get(1).unwrap().submitted_date(), utc_time_two);
    assert_eq!(
        all_pings.get(1).unwrap().uploaded_date().unwrap(),
        utc_time_two
    );
    assert_eq!(
        all_pings.get(1).unwrap().payload.clone().unwrap(),
        serde_json::json!({ "test": "a value" })
    );
    assert_eq!(all_pings.first().unwrap().document_id, "id-two");
    assert!(all_pings.first().unwrap().upload_failed.is_some());

    let count = glean.storage().mark_ping_as_uploaded("id", utc_time_one);
    assert_eq!(count, 1);

    let some_pings = glean.storage().get_submitted_pings_by_name("ping");
    assert_eq!(some_pings.len(), 1);
    assert_eq!(some_pings.first().unwrap().document_id, "id".to_string());
    assert_eq!(
        some_pings.first().unwrap().uploaded_date().unwrap(),
        utc_time_one
    );
}

#[test]
fn test_cleanup_of_submitted_pings() {
    let (glean, _temp) = new_glean(None);

    let utc_time_more_than_30_days_ago =
        chrono::DateTime::parse_from_rfc3339("2026-06-05T12:30:00.50Z")
            .unwrap()
            .to_utc();

    // Submitted ping from more than 30 days ago
    glean
        .storage()
        .store_submitted_ping(
            "id-one",
            "ping",
            utc_time_more_than_30_days_ago,
            None,
            None,
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    // Submitted ping from now
    glean
        .storage()
        .store_submitted_ping(
            "id-two",
            "ping",
            Utc::now(),
            None,
            None,
            serde_json::json!({ "test": "a value" }),
        )
        .unwrap();

    // Both pings should have been stored
    let all_pings = glean.storage().get_all_submitted_pings();
    assert_eq!(all_pings.len(), 2);

    // Run regular maintenance (happens on shutdown)
    // This should only remove the ping from >30 days ago
    glean
        .storage()
        .cleanup_submitted_pings(None)
        .expect("Error running cleanup_submitted_pings");

    let all_pings = glean.storage().get_all_submitted_pings();
    assert_eq!(all_pings.len(), 1);
    assert_eq!(all_pings.first().unwrap().document_id, "id-two".to_string());

    // Run `cleanup_submitted_pings` with now as the `before_time`
    // This should clear out all pings
    glean
        .storage()
        .cleanup_submitted_pings(Some(Utc::now()))
        .expect("Error running cleanup_submitted_pings");

    let all_pings = glean.storage().get_all_submitted_pings();
    assert_eq!(all_pings.len(), 0);
}
