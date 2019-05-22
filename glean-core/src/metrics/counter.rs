// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn should_record(&self, glean: &Glean) -> bool {
        glean.is_upload_enabled() && self.meta.should_record()
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
    }
}
