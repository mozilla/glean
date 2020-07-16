// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Lifetime};

const SAMPLE_JWE_VALUE: &str = "eyJhbGciOiJFQ0RILUVTIiwia2lkIjoiMFZFRTdmT0txbFdHVGZrY0taRUJ2WWl3dkpMYTRUUGlJVGxXMGJOcDdqVSIsImVwayI6eyJrdHkiOiJFQyIsImNydiI6IlAtMjU2IiwieCI6InY3Q1FlRWtVQjMwUGwxV0tPMUZUZ25OQlNQdlFyNlh0UnZxT2kzSWdzNHciLCJ5IjoiNDBKVEpaQlMwOXpWNHpxb0hHZDI5NGFDeHRqcGU5a09reGhELVctUEZsSSJ9LCJlbmMiOiJBMjU2R0NNIn0..A_wzJya943vlHKFH.yq0JhkGZiZd6UiZK6goTcEf6i4gbbBeXxvq8QV5_nC4.Knl_sYSBrrP-aa54z6B6gA";

#[test]
fn jwe_metric_is_generated_and_stored() {
    let (glean, _t) = new_glean(None);

    let metric = JweMetric::new(CommonMetricData {
        name: "jwe_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    metric.set_with_compact_repr(&glean, SAMPLE_JWE_VALUE);
    let snapshot = StorageManager
        .snapshot_as_json(glean.storage(), "core", true)
        .unwrap();

    assert_eq!(
        json!({"jwe": {"local.jwe_metric": SAMPLE_JWE_VALUE }}),
        snapshot
    );
}

#[test]
fn set_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean(None);
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = JweMetric::new(CommonMetricData {
        name: "jwe_metric".into(),
        category: "local".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
        ..Default::default()
    });

    metric.set_with_compact_repr(&glean, SAMPLE_JWE_VALUE);

    // Check that the data was correctly set in each store.
    for store_name in store_names {
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), &store_name, true)
            .unwrap();

        assert_eq!(
            json!({"jwe": {"local.jwe_metric": SAMPLE_JWE_VALUE}}),
            snapshot
        );
    }
}
