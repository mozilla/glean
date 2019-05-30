// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::metrics::time_unit::TimeUnit;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;
use crate::util::get_iso_time_string;

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

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as a DateTime<FixedOffset>.
    /// The precision of this value is truncated to the `time_unit`
    /// precision.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<DateTime<FixedOffset>> {
        panic!("This is not yet implemented. Please consider using `test_get_value_as_string"`);
        None
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as a String.
    /// The precision of this value is truncated to the `time_unit`
    /// precision.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_string(&self, glean: &Glean, storage_name: &str) -> Option<String> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::Datetime(d, tu)) => Some(get_iso_time_string(d, tu)),
            _ => None,
        }
    }
}
