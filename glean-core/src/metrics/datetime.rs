// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::metrics::time_unit::TimeUnit;
use crate::CommonMetricData;
use crate::Glean;

use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct DatetimeMetric {
    meta: CommonMetricData,
    time_unit: TimeUnit,
}

impl MetricType for DatetimeMetric {
    fn with_meta(meta: CommonMetricData) -> Self {
        Self { meta, time_unit: TimeUnit::Day } // FIXME: How do we handle this?
    }

    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }
}

impl DatetimeMetric {
    pub fn new(meta: CommonMetricData, time_unit: TimeUnit) -> Self {
        Self { meta, time_unit }
    }

    /// Public facing API for setting
    pub fn set(&self, glean: &Glean, value: DateTime<FixedOffset>) {
        if !self.should_record(glean) {
            return;
        }

        let value = Metric::Datetime(value, self.time_unit.clone());
        glean
            .storage()
            .record(&self.meta, &value)
    }
/*
    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<i32> {
        let snapshot = match StorageManager.snapshot_as_json(glean.storage(), storage_name, false) {
            Some(snapshot) => snapshot,
            None => return None,
        };
        snapshot
            .as_object()
            .and_then(|o| o.get("counter"))
            .and_then(|o| o.as_object())
            .and_then(|o| o.get(&self.meta.identifier()))
            .and_then(|o| o.as_i64().map(|i| i as i32))
    }*/
}
