// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use chrono::prelude::*;
use serde_json::json;

use glean_core::metrics::*;
use glean_core::storage::StorageManager;
use glean_core::{CommonMetricData, Glean, Lifetime};

// SKIPPED from glean-ac: string deserializer should correctly parse integers
// This test doesn't really apply to rkv

#[test]
fn datetime_serializer_should_correctly_serialize_datetime() {
    let (_t, tmpname) = tempdir();
    {
        let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

        let metric = DatetimeMetric::new(CommonMetricData {
            name: "datetime_metric".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["store1".into()],
            disabled: false,
            lifetime: Lifetime::User,
        }, TimeUnit::Minute);

        // `2014-07-08T09:10:11Z`
        let dt = Utc.ymd(2014, 7, 8).and_hms(9, 10, 11);
        metric.set(&glean, dt);

        let snapshot = StorageManager
            .snapshot_as_json(glean.storage(), "store1", true)
            .unwrap();
        assert_eq!(
            json!({"datetime": {"telemetry.datetime_metric": "2014-07-08T09:10:11Z"}}),
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
            json!({"datetime": {"telemetry.datetime_metric": "2014-07-08T09:10:11Z"}}),
            snapshot
        );
    }
}
