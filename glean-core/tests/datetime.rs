// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use chrono::prelude::*;
use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Lifetime};

// SKIPPED from glean-ac: datetime deserializer should correctly parse integers
// This test doesn't really apply to rkv

#[test]
fn datetime_serializer_should_correctly_serialize_datetime() {
    let expected_value = "1983-04-13T12:09+00:00";
    let (mut tempdir, _) = tempdir();

    {
        // We give tempdir to the `new_glean` function...
        let (glean, dir) = new_glean(Some(tempdir));
        // And then we get it back once that function returns.
        tempdir = dir;

        let metric = DatetimeMetric::new(
            CommonMetricData {
                name: "datetime_metric".into(),
                category: "telemetry".into(),
                send_in_pings: vec!["store1".into()],
                disabled: false,
                lifetime: Lifetime::User,
                ..Default::default()
            },
            TimeUnit::Minute,
        );

        // `1983-04-13T12:09:14.274+00:00` will be truncated to Minute resolution.
        let dt = FixedOffset::east_opt(0)
            .unwrap()
            .with_ymd_and_hms(1983, 4, 13, 12, 9, 14)
            .unwrap()
            .with_nanosecond(274 * 1_000_000)
            .unwrap();
        metric.set_sync(&glean, Some(dt.into()));

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"datetime": {"telemetry.datetime_metric": expected_value}}),
            snapshot
        );
    }

    // Make a new Glean instance here, which should force reloading of the data from disk
    // so we can ensure it persisted, because it has User lifetime
    {
        let (glean, _t) = new_glean(Some(tempdir));
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"datetime": {"telemetry.datetime_metric": expected_value}}),
            snapshot
        );
    }
}

#[test]
fn set_value_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean(None);
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let metric = DatetimeMetric::new(
        CommonMetricData {
            name: "datetime_metric".into(),
            category: "telemetry".into(),
            send_in_pings: store_names.clone(),
            disabled: false,
            lifetime: Lifetime::Ping,
            ..Default::default()
        },
        TimeUnit::Nanosecond,
    );

    // `1983-04-13T12:09:14.274+00:00` will be truncated to Minute resolution.
    let dt = FixedOffset::east_opt(0)
        .unwrap()
        .with_ymd_and_hms(1983, 4, 13, 12, 9, 14)
        .unwrap()
        .with_nanosecond(1_560_274)
        .unwrap();
    metric.set_sync(&glean, Some(dt.into()));

    for store_name in store_names {
        assert_eq!(
            "1983-04-13T12:09:14.001560274+00:00",
            metric
                .get_value_as_string(&glean, Some(store_name))
                .unwrap()
        );
    }
}

// SKIPPED from glean-ac: getSnapshot() returns null if nothing is recorded in the store
// This test doesn't really apply to rkv

// SKIPPED from glean-ac: getSnapshot() correctly clears the stores
// This test doesn't really apply to rkv

#[test]
fn test_that_truncation_works() {
    let (glean, _t) = new_glean(None);

    // `1985-07-03T12:09:14.000560274+01:00`
    let high_res_datetime = FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(1985, 7, 3, 12, 9, 14)
        .unwrap()
        .with_nanosecond(1_560_274)
        .unwrap();
    let store_name = "store1";

    // Create an helper struct for defining the truncation cases.
    struct TestCase {
        case_name: &'static str,
        desired_resolution: TimeUnit,
        expected_result: &'static str,
    }

    // Define the single test cases.
    let test_cases = vec![
        TestCase {
            case_name: "nano",
            desired_resolution: TimeUnit::Nanosecond,
            expected_result: "1985-07-03T12:09:14.001560274+01:00",
        },
        TestCase {
            case_name: "micro",
            desired_resolution: TimeUnit::Microsecond,
            expected_result: "1985-07-03T12:09:14.001560+01:00",
        },
        TestCase {
            case_name: "milli",
            desired_resolution: TimeUnit::Millisecond,
            expected_result: "1985-07-03T12:09:14.001+01:00",
        },
        TestCase {
            case_name: "second",
            desired_resolution: TimeUnit::Second,
            expected_result: "1985-07-03T12:09:14+01:00",
        },
        TestCase {
            case_name: "minute",
            desired_resolution: TimeUnit::Minute,
            expected_result: "1985-07-03T12:09+01:00",
        },
        TestCase {
            case_name: "hour",
            desired_resolution: TimeUnit::Hour,
            expected_result: "1985-07-03T12+01:00",
        },
        TestCase {
            case_name: "day",
            desired_resolution: TimeUnit::Day,
            expected_result: "1985-07-03+01:00",
        },
    ];

    // Execute them all.
    for t in test_cases {
        let metric = DatetimeMetric::new(
            CommonMetricData {
                name: format!("datetime_metric_{}", t.case_name),
                category: "telemetry".into(),
                send_in_pings: vec![store_name.into()],
                disabled: false,
                lifetime: Lifetime::User,
                ..Default::default()
            },
            t.desired_resolution,
        );
        metric.set_sync(&glean, Some(high_res_datetime.into()));

        assert_eq!(
            t.expected_result,
            metric
                .get_value_as_string(&glean, Some(store_name.into()))
                .unwrap()
        );
    }
}

#[test]
fn gotten_value_is_correct() {
    let store_name = "store1";
    let metric = DatetimeMetric::new(
        CommonMetricData {
            name: "like_an_arrow".into(),
            category: "time.flies".into(),
            send_in_pings: vec![store_name.into()],
            ..Default::default()
        },
        TimeUnit::Second,
    );

    let (glean, _t) = new_glean(None);

    // `1985-07-03T12:09:14.000560274+01:00`
    let chrono_dt = FixedOffset::east_opt(3600)
        .unwrap()
        .with_ymd_and_hms(1985, 7, 3, 12, 9, 14)
        .unwrap();

    metric.set_sync(&glean, Some(chrono_dt.into()));
    assert_eq!(
        chrono_dt,
        metric.get_value(&glean, Some(store_name)).unwrap()
    );
}
