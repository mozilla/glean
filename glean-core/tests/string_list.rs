// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{test_get_num_recorded_errors, CommonMetricData, ErrorType, Glean, Lifetime};

#[test]
fn list_can_store_multiple_items() {
    let (mut glean, _t) = new_glean();

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    list.add(&glean, "first");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));
    assert!(!snapshot.contains(r#""second""#));
    assert!(!snapshot.contains(r#""third""#));
    assert!(!snapshot.contains(r#""fourth""#));

    list.add(&glean, "second");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));
    assert!(snapshot.contains(r#""second""#));
    assert!(!snapshot.contains(r#""third""#));
    assert!(!snapshot.contains(r#""fourth""#));

    list.set(&glean, vec!["third".into()]);
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(!snapshot.contains(r#""first""#));
    assert!(!snapshot.contains(r#""second""#));
    assert!(snapshot.contains(r#""third""#));
    assert!(!snapshot.contains(r#""fourth""#));

    list.add(&glean, "fourth");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(!snapshot.contains(r#""first""#));
    assert!(!snapshot.contains(r#""second""#));
    assert!(snapshot.contains(r#""third""#));
    assert!(snapshot.contains(r#""fourth""#));
}

#[test]
fn stringlist_serializer_should_correctly_serialize_stringlists() {
    let (_t, tmpname) = tempdir();
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();
        let metric = StringListMetric::new(CommonMetricData {
            name: "string_list_metric".into(),
            category: "telemetry.test".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::User,
        });
        metric.set(&glean, vec!["test_string_1".into(), "test_string_2".into()]);
    }

    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"string_list": {"telemetry.test.string_list_metric": ["test_string_1", "test_string_2"]}}),
            snapshot
        );
    }
}

#[test]
fn set_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean();
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = StringListMetric::new(CommonMetricData {
        name: "string_list_metric".into(),
        category: "telemetry.test".into(),
        send_in_pings: store_names.clone(),
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    metric.set(&glean, vec!["test_string_1".into(), "test_string_2".into()]);

    for store_name in store_names {
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), &store_name, true)
            .unwrap();

        assert_eq!(
            json!({"string_list": {"telemetry.test.string_list_metric": ["test_string_1", "test_string_2"]}}),
            snapshot
        );
    }
}

#[test]
fn long_string_values_are_truncated() {
    let (glean, _t) = new_glean();

    let metric = StringListMetric::new(CommonMetricData {
        name: "string_list_metric".into(),
        category: "telemetry.test".into(),
        send_in_pings: vec!["store1".into()],
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    let test_string = "0123456789".repeat(20);
    metric.add(&glean, test_string.clone());

    // Ensure the string was truncated to the proper length.
    assert_eq!(
        vec![test_string[..50].to_string()],
        metric.test_get_value(&glean, "store1").unwrap()
    );

    // Ensure the error has been recorded.
    assert_eq!(
        Ok(1),
        test_get_num_recorded_errors(&glean, metric.meta(), ErrorType::InvalidValue, None)
    );
}

#[test]
fn disabled_string_lists_dont_record() {
    let (glean, _t) = new_glean();

    let metric = StringListMetric::new(CommonMetricData {
        name: "string_list_metric".into(),
        category: "telemetry.test".into(),
        send_in_pings: vec!["store1".into()],
        disabled: true,
        lifetime: Lifetime::Ping,
    });

    metric.add(&glean, "test_string".repeat(20));

    // Ensure the string was not added.
    assert_eq!(None, metric.test_get_value(&glean, "store1"));

    metric.set(&glean, vec!["test_string_2".repeat(20)]);

    // Ensure the stringlist was not set.
    assert_eq!(None, metric.test_get_value(&glean, "store1"));

    // Ensure no error was recorded.
    assert!(
        test_get_num_recorded_errors(&glean, metric.meta(), ErrorType::InvalidValue, None).is_err()
    );
}

#[test]
fn string_lists_dont_exceed_max_items() {
    let (glean, _t) = new_glean();

    let metric = StringListMetric::new(CommonMetricData {
        name: "string_list_metric".into(),
        category: "telemetry.test".into(),
        send_in_pings: vec!["store1".into()],
        disabled: false,
        lifetime: Lifetime::Ping,
    });

    for _n in 1..21 {
        metric.add(&glean, "test_string");
    }

    let expected: Vec<String> = "test_string "
        .repeat(20)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    assert_eq!(expected, metric.test_get_value(&glean, "store1").unwrap());

    // Ensure the 21st string wasn't added.
    metric.add(&glean, "test_string");
    assert_eq!(expected, metric.test_get_value(&glean, "store1").unwrap());

    // TODO (bug 1557828) - Uncomment when the error starts being recorded again.
    // Ensure we recorded the error.
    // assert_eq!(Ok(1), test_get_num_recorded_errors(&glean, metric.meta(), ErrorType::InvalidValue, None));

    // Clear the metric.
    metric.set(&glean, vec![]);

    // Try to set it to a list that's too long. Ensure it cuts off at 20 elements.
    let too_many: Vec<String> = "test_string "
        .repeat(21)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    metric.set(&glean, too_many);
    assert_eq!(expected, metric.test_get_value(&glean, "store1").unwrap());

    // TODO (bug 1557828) - Increment by 1 when the previous error starts being recorded again.
    assert_eq!(
        Ok(1),
        test_get_num_recorded_errors(&glean, metric.meta(), ErrorType::InvalidValue, None)
    );
}
