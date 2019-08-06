// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use std::time::Duration;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{test_get_num_recorded_errors, ErrorType};
use glean_core::{CommonMetricData, Glean, Lifetime};

// Tests ported from glean-ac

#[test]
fn serializer_should_correctly_serialize_timing_distribution() {
    let (_t, tmpname) = tempdir();

    let duration = 60;
    let time_unit = TimeUnit::Nanosecond;

    let cfg = glean_core::Configuration {
        data_path: tmpname,
        application_id: GLOBAL_APPLICATION_ID.into(),
        upload_enabled: true,
        max_events: None,
    };

    {
        let glean = Glean::new(cfg.clone()).unwrap();

        let mut metric = TimingDistributionMetric::new(
            CommonMetricData {
                name: "distribution".into(),
                category: "telemetry".into(),
                send_in_pings: vec!["store1".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
            },
            time_unit,
        );

        let id = metric.set_start(&glean, 0);
        metric.set_stop_and_accumulate(&glean, id, duration);

        let val = metric
            .test_get_value(&glean, "store1")
            .expect("Value should be stored");

        assert_eq!(val.sum(), duration);
    }

    // Make a new Glean instance here, which should force reloading of the data from disk
    // so we can ensure it persisted, because it has User lifetime
    {
        let glean = Glean::new(cfg.clone()).unwrap();
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();

        assert_eq!(
            json!(duration),
            snapshot["timing_distribution"]["telemetry.distribution"]["sum"]
        );
    }
}

#[test]
fn set_value_properly_sets_the_value_in_all_stores() {
    let (glean, _t) = new_glean();
    let store_names: Vec<String> = vec!["store1".into(), "store2".into()];

    let duration = 60;

    let mut metric = TimingDistributionMetric::new(
        CommonMetricData {
            name: "distribution".into(),
            category: "telemetry".into(),
            send_in_pings: store_names.clone(),
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        TimeUnit::Nanosecond,
    );

    let id = metric.set_start(&glean, 0);
    metric.set_stop_and_accumulate(&glean, id, duration);

    for store_name in store_names {
        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), &store_name, true)
            .unwrap();

        assert_eq!(
            json!(duration),
            snapshot["timing_distribution"]["telemetry.distribution"]["sum"]
        );
        assert_eq!(
            json!(1),
            snapshot["timing_distribution"]["telemetry.distribution"]["values"]["56"]
        );
    }
}

#[test]
fn timing_distributions_must_not_accumulate_negative_values() {
    let (glean, _t) = new_glean();

    let duration = 60;
    let time_unit = TimeUnit::Nanosecond;

    let mut metric = TimingDistributionMetric::new(
        CommonMetricData {
            name: "distribution".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        time_unit,
    );

    // Flip around the timestamps, this should result in a negative value which should be
    // discarded.
    let id = metric.set_start(&glean, duration);
    metric.set_stop_and_accumulate(&glean, id, 0);

    assert!(metric.test_get_value(&glean, "store1").is_none());

    // Make sure that the errors have been recorded
    assert_eq!(
        Ok(1),
        test_get_num_recorded_errors(
            &glean,
            metric.meta(),
            ErrorType::InvalidValue,
            Some("store1")
        )
    );
}

#[test]
fn the_accumulate_samples_api_correctly_stores_timing_values() {
    let (glean, _t) = new_glean();

    let mut metric = TimingDistributionMetric::new(
        CommonMetricData {
            name: "distribution".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        TimeUnit::Second,
    );

    // Accumulate the samples. We intentionally do not report
    // negative values to not trigger error reporting.
    metric.accumulate_samples_signed(&glean, [1, 2, 3].to_vec());

    let val = metric
        .test_get_value(&glean, "store1")
        .expect("Value should be stored");

    // Check that we got the right sum and number of samples.
    assert_eq!(val.sum(), 6);
    assert_eq!(val.count(), 3);

    // We should get a sample in each of the first 3 buckets.
    assert_eq!(1, val.values()[1]);
    assert_eq!(1, val.values()[2]);
    assert_eq!(1, val.values()[3]);

    // No errors should be reported.
    assert!(test_get_num_recorded_errors(
        &glean,
        metric.meta(),
        ErrorType::InvalidValue,
        Some("store1")
    )
    .is_err());
}

#[test]
fn the_accumulate_samples_api_correctly_handles_negative_values() {
    let (glean, _t) = new_glean();

    let mut metric = TimingDistributionMetric::new(
        CommonMetricData {
            name: "distribution".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        TimeUnit::Second,
    );

    // Accumulate the samples.
    metric.accumulate_samples_signed(&glean, [-1, 1, 2, 3].to_vec());

    let val = metric
        .test_get_value(&glean, "store1")
        .expect("Value should be stored");

    // Check that we got the right sum and number of samples.
    assert_eq!(val.sum(), 6);
    assert_eq!(val.count(), 3);

    // We should get a sample in each of the first 3 buckets.
    assert_eq!(1, val.values()[1]);
    assert_eq!(1, val.values()[2]);
    assert_eq!(1, val.values()[3]);

    // 1 error should be reported.
    assert_eq!(
        Ok(1),
        test_get_num_recorded_errors(
            &glean,
            metric.meta(),
            ErrorType::InvalidValue,
            Some("store1")
        )
    );
}

#[test]
fn large_nanoseconds_values() {
    let (glean, _t) = new_glean();

    let mut metric = TimingDistributionMetric::new(
        CommonMetricData {
            name: "distribution".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
        },
        TimeUnit::Nanosecond,
    );

    let time = Duration::from_secs(10).as_nanos() as u64;
    assert!(time > u64::from(u32::max_value()));

    let id = metric.set_start(&glean, 0);
    metric.set_stop_and_accumulate(&glean, id, time);

    let val = metric
        .test_get_value(&glean, "store1")
        .expect("Value should be stored");

    // Check that we got the right sum and number of samples.
    assert_eq!(val.sum() as u64, time);
}
