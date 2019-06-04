// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Glean, Lifetime};

// SKIPPED from glean-ac: string deserializer should correctly parse integers
// This test doesn't really apply to rkv

#[test]
fn string_serializer_should_correctly_serialize_strings() {
    let (_t, tmpname) = tempdir();
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

        let metric = StringMetric::new(CommonMetricData {
            name: "string_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::User,
        });

        metric.set(&glean, "test_string_value");

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"string": {"telemetry.string_metric": "test_string_value"}}),
            snapshot
        );
    }

    // Make a new glean instance here, which should force reloading of the data from disk
    // so we can ensure it persisted, because it has User lifetime
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"string": {"telemetry.string_metric": "test_string_value"}}),
            snapshot
        );
    }
}

#[test]
fn set_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean();
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = StringMetric::new(CommonMetricData {
        name: "string_metric".into(),
        category: "telemetry".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    metric.set(&glean, "test_string_value");

    // Check that the data was correctly set in each store.
    for store_name in store_names {
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), &store_name, true)
            .unwrap();

        assert_eq!(
            json!({"string": {"telemetry.string_metric": "test_string_value"}}),
            snapshot
        );
    }
}

// SKIPPED from glean-ac: strings are serialized in the correct JSON format
// Completely redundant with other tests.

#[test]
fn long_string_values_are_truncated() {
    let (glean, _t) = new_glean();

    let metric = StringMetric::new(CommonMetricData {
        name: "string_metric".into(),
        category: "telemetry".into(),
        send_in_pings: vec!["store1".into()],
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    let test_sting = "01234567890".repeat(20);
    metric.set(&glean, test_sting.clone());

    // Check that data was truncated
    assert_eq!(
        test_sting[..100],
        metric.test_get_value(&glean, "store1").unwrap()
    );

    // TODO: Requires error reporting (bug 1551975)
    //assertEquals(1, testGetNumRecordedErrors(stringMetric, ErrorType.InvalidValue))
}
