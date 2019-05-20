// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Glean, Lifetime};

// SKIPPED from glean-ac: counter deserializer should correctly parse integers
// This test doesn't really apply to rkv

#[test]
fn counter_serializer_should_correctly_serialize_counters() {
    let (_t, tmpname) = tempdir();
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID).unwrap();

        let metric = CounterMetric::new(CommonMetricData {
            name: "counter_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::User,
        });

        metric.add(&glean, 1);

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"counter": {"telemetry.counter_metric": 1}}),
            snapshot
        );
    }

    // Make a new glean instance here, which should force reloading of the data from disk
    // so we can ensure it persisted, because it has User lifetime
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID).unwrap();
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"counter": {"telemetry.counter_metric": 1}}),
            snapshot
        );
    }
}

#[test]
fn set_value_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean();
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = CounterMetric::new(CommonMetricData {
        name: "counter_metric".into(),
        category: "telemetry".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    metric.add(&glean, 1);

    for store_name in store_names {
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), &store_name, true)
            .unwrap();

        assert_eq!(
            json!({"counter": {"telemetry.counter_metric": 1}}),
            snapshot
        );
    }
}

#[test]
fn snapshot_returns_none_if_nothing_is_recorded_in_the_store() {
    let (glean, _t) = new_glean();
    assert!(StorageManager
        .snapshot(glean.storage(), "unknown_store", true)
        .is_none())
}

#[test]
fn snapshot_correctly_clears_the_stores() {
    let (glean, _t) = new_glean();
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = CounterMetric::new(CommonMetricData {
        name: "counter_metric".into(),
        category: "telemetry".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    metric.add(&glean, 1);

    // Get the snapshot from "store1" and clear it.
    let snapshot = StorageManager.snapshot(glean.storage(), "store1", true);
    assert!(!snapshot.unwrap().is_empty());
    // Check that getting a new snapshot for "store1" returns an empty store.
    assert!(StorageManager
        .snapshot(glean.storage(), "store1", false)
        .is_none());
    // Check that we get the right data from both the stores. Clearing "store1" must
    // not clear "store2" as well.
    let snapshot2 = StorageManager.snapshot(glean.storage(), "store2", true);
    assert!(!snapshot2.unwrap().is_empty());
}

// SKIPPED from glean-ac: counters are serialized in the correct JSON format
// Completely redundant with other tests.

#[test]
fn counters_must_not_increment_when_passed_zero_or_negative() {
    let (glean, _t) = new_glean();

    let metric = CounterMetric::new(CommonMetricData {
        name: "counter_metric".into(),
        category: "telemetry".into(),
        send_in_pings: vec!["store1".into()],
        disabled: false,
        lifetime: Lifetime::Application,
    });

    // Attempt to increment the counter with zero
    metric.add(&glean, 0);
    // Check that nothing was recorded
    assert!(metric.test_get_value(&glean, "store1").is_none());

    // Attempt to increment the counter with negative
    metric.add(&glean, -1);
    // Check that nothing was recorded
    assert!(metric.test_get_value(&glean, "store1").is_none());

    // Attempt increment counter properly
    metric.add(&glean, 1);
    // Check that nothing was recorded
    assert_eq!(1, metric.test_get_value(&glean, "store1").unwrap());

    // Attempt increment counter properly
    metric.add(&glean, -1);
    // Check that nothing was recorded
    assert_eq!(1, metric.test_get_value(&glean, "store1").unwrap());

    // TODO: 1551975 Implement error reporting
    // assert_eq!(2, test_get_num_recorded_errors(metric, ...))
}
