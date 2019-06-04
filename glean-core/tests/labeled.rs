// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::Glean;
use glean_core::{CommonMetricData, Lifetime};

#[test]
fn can_create_labeled_counter_metric() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        Some(vec!["label1".into()]),
    );

    let metric = labeled.get(&glean, "label1");
    metric.add(&glean, 1);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": { "label1": 1 }
            }
        }),
        snapshot
    );
}

#[test]
fn can_create_labeled_string_metric() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        StringMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        Some(vec!["label1".into()]),
    );

    let metric = labeled.get(&glean, "label1");
    metric.set(&glean, "text");

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_string": {
                "telemetry.labeled_metric": { "label1": "text" }
            }
        }),
        snapshot
    );
}

#[test]
fn can_create_labeled_bool_metric() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        BooleanMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        Some(vec!["label1".into()]),
    );

    let metric = labeled.get(&glean, "label1");
    metric.set(&glean, true);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_boolean": {
                "telemetry.labeled_metric": { "label1": true }
            }
        }),
        snapshot
    );
}

#[test]
fn can_use_multiple_labels() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        None,
    );

    let metric = labeled.get(&glean, "label1");
    metric.add(&glean, 1);

    let metric = labeled.get(&glean, "label2");
    metric.add(&glean, 2);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": {
                    "label1": 1,
                    "label2": 2,
                }
            }
        }),
        snapshot
    );
}

#[test]
fn labels_are_checked_against_static_list() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        Some(vec!["label1".into(), "label2".into()]),
    );

    let metric = labeled.get(&glean, "label1");
    metric.add(&glean, 1);

    let metric = labeled.get(&glean, "label2");
    metric.add(&glean, 2);

    // All non-registed labels get mapped to the `other` label
    let metric = labeled.get(&glean, "label3");
    metric.add(&glean, 3);
    let metric = labeled.get(&glean, "label4");
    metric.add(&glean, 4);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": {
                    "label1": 1,
                    "label2": 2,
                    "__other__": 7,
                }
            }
        }),
        snapshot
    );
}

#[test]
fn dynamic_labels_too_long() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        None,
    );

    let metric = labeled.get(&glean, "this_string_has_more_than_thirty_characters");
    metric.add(&glean, 1);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": {
                    "__other__": 1,
                }
            }
        }),
        snapshot
    );
}

#[test]
fn dynamic_labels_regex_mimsatch() {
    let (glean, _t) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        None,
    );

    labeled.get(&glean, "notSnakeCase").add(&glean, 1);
    labeled.get(&glean, "").add(&glean, 1);
    labeled.get(&glean, "with/slash").add(&glean, 1);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": {
                    "__other__": 3,
                }
            }
        }),
        snapshot
    );
}

#[test]
fn seen_labels_get_reloaded_from_disk() {
    let (glean, dir) = new_glean();
    let mut labeled = LabeledMetric::new(
        CounterMetric::new(CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        }),
        None,
    );

    // Store some data into labeled metrics
    {
        // Set the maximum number of labels
        for i in 1..=16 {
            let label = format!("label{}", i);
            labeled.get(&glean, &label).add(&glean, i);
        }

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", false)
            .unwrap();

        // Check that the data is there
        for i in 1..=16 {
            let label = format!("label{}", i);
            assert_eq!(
                i,
                snapshot["labeled_counter"]["telemetry.labeled_metric"][&label]
            );
        }

        drop(glean);
    }

    // Force a reload
    {
        let tmpname = dir.path().display().to_string();
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

        // Try to store another label
        labeled.get(&glean, "new_label").add(&glean, 40);

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", false)
            .unwrap();

        // Check that the old data is still there
        for i in 1..=16 {
            let label = format!("label{}", i);
            assert_eq!(
                i,
                snapshot["labeled_counter"]["telemetry.labeled_metric"][&label]
            );
        }

        // The new label lands in the __other__ bucket, due to too many labels
        assert_eq!(
            40,
            snapshot["labeled_counter"]["telemetry.labeled_metric"]["__other__"]
        );
    }
}
