// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

const MAX_LENGTH_VALUE: usize = 100;

/// A string metric.
///
/// Record an Unicode string value with arbitrary content.
/// Strings are length-limited to `MAX_LENGTH_VALUE` bytes.
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
    /// Create a new string metric.
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    /// Set to the specified value.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `value` - The string to set the metric to.
    ///
    /// ## Notes
    ///
    /// Truncates the value if it is longer than `MAX_STRING_LENGTH` bytes and logs an error.
    pub fn set<S: Into<String>>(&self, glean: &Glean, value: S) {
        if !self.should_record(glean) {
            return;
        }

        let s = value.into();
        let s = if s.len() > MAX_LENGTH_VALUE {
            let msg = format!(
                "Value length {} exceeds maximum of {}",
                s.len(),
                MAX_LENGTH_VALUE
            );
            record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
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
