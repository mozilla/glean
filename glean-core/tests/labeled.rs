// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Lifetime};

#[test]
fn can_create_labeled_metric() {
    let (glean, _t) = new_glean();
    let mut labeled: LabeledMetric<CounterMetric> = LabeledMetric::new(
        CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        Some(vec!["label1".into(), "label2".into()]),
    );

    let metric = labeled.get(&glean, "label1");
    metric.add(&glean, 1);

    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "store1", true)
        .unwrap();

    assert_eq!(
        json!({
            "labeled_counter": {
                "telemetry.labeled_metric": {
                    "label1": 1
                }
            }
        }),
        snapshot
    );
}

#[test]
fn can_use_multiple_labels() {
    let (glean, _t) = new_glean();
    let mut labeled: LabeledMetric<CounterMetric> = LabeledMetric::new(
        CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
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
    let mut labeled: LabeledMetric<CounterMetric> = LabeledMetric::new(
        CommonMetricData {
            name: "labeled_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
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
