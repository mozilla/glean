// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Lifetime};

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
        name: "metric".into(),
        category: "telemetry".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    metric.add(&glean, 1);

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
