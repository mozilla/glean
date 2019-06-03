// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

const MAX_LENGTH_VALUE: usize = 50;

#[derive(Clone, Debug)]
pub struct StringMetric {
    meta: CommonMetricData,
}

impl MetricType for StringMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, glean: &Glean, value: S) {
        if !self.should_record(glean) {
            return;
        }

        let s = value.into();
        let s = if s.len() > MAX_LENGTH_VALUE {
            record_error(glean, &self.meta, ErrorType::InvalidValue);
            s[0..MAX_LENGTH_VALUE].to_string()
        } else {
            s
        };

        let value = Metric::String(s);
        glean.storage().record(&self.meta, &value)
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as a string.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<String> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::String(s)) => Some(s),
            _ => None,
        }
    }
}
