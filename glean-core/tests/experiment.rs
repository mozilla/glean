// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{test_get_num_recorded_errors, ErrorType};
use glean_core::{CommonMetricData, Glean, Lifetime};

// Tests ported from glean-ac

// SKIPPED from glean-ac: counter deserializer should correctly parse integers
// This test doesn't really apply to rkv

#[test]
fn experiment_serializer_should_correctly_serialize_experiments() {
    let (_t, tmpname) = tempdir();
    let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

    let metric = ExperimentMetric::new("some-experiment".to_string());

    metric.set_active(&glean, "test-branch".to_string(), None);

    let snapshot = StorageManager
        .snapshot_experiments_as_json(glean.storage())
        .unwrap();
    assert_eq!(
        json!({"experiments": {"some-experiment": {"branch": "test-branch"}}}),
        snapshot
    );

    metric.set_inactive(&glean);

    let empty_snapshot = StorageManager
        .snapshot_experiments_as_json(glean.storage());
    assert!(empty_snapshot.is_none());
}

