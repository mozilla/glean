// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
}

impl MetricType for CounterMetric {
    fn with_meta(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn add(&self, glean: &Glean, amount: i32) {
        if !self.should_record(glean) {
            return;
        }

        if amount <= 0 {
            // TODO: Turn this into logging an error
            log::warn!("CounterMetric::add: got negative amount. Not recording.");
            return;
        }

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
                _ => Metric::Counter(amount),
            })
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<i32> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::Counter(i)) => Some(i),
            _ => None,
        }
    }
}
