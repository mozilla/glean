// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

// Maximum length of any list
const MAX_LIST_LENGTH: usize = 20;
// Maximum length of any string in the list
const MAX_STRING_LENGTH: usize = 50;

/// A string list metric.
///
/// This allows appending a string value with arbitrary content to a list.
#[derive(Clone, Debug)]
pub struct StringListMetric {
    meta: CommonMetricData,
}

impl MetricType for StringListMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl StringListMetric {
    /// Create a new string list metric.
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    /// Add a new string to the list.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `value` - The string to add.
    ///
    /// ## Notes
    ///
    /// Truncates the value if it is longer than `MAX_STRING_LENGTH` bytes and logs an error.
    pub fn add<S: Into<String>>(&self, glean: &Glean, value: S) {
        if !self.should_record(glean) {
            return;
        }

        let value = value.into();
        let value = if value.len() > MAX_STRING_LENGTH {
            let msg = format!(
                "Individual value length {} exceeds maximum of {}",
                value.len(),
                MAX_STRING_LENGTH
            );
            record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
            value[0..MAX_STRING_LENGTH].to_string()
        } else {
            value
        };

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::StringList(mut old_value)) => {
                    if old_value.len() == MAX_LIST_LENGTH {
                        let msg = format!(
                            "String list length of {} exceeds maximum of {}",
                            old_value.len() + 1,
                            MAX_LIST_LENGTH
                        );
                        record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
                        return Metric::StringList(old_value);
                    }
                    old_value.push(value.clone());
                    Metric::StringList(old_value)
                }
                _ => Metric::StringList(vec![value.clone()]),
            })
    }

    /// Set to a specific list of strings.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `value` - The list of string to set the metric to.
    ///
    /// ## Notes
    ///
    /// Truncates the list if it is longer than `MAX_LIST_LENGTH` and logs an error.
    /// Truncates any value in the list if it is longer than `MAX_STRING_LENGTH` and logs an error.
    pub fn set(&self, glean: &Glean, value: Vec<String>) {
        if !self.should_record(glean) {
            return;
        }

        let value = if value.len() > MAX_LIST_LENGTH {
            let msg = format!(
                "Individual value length {} exceeds maximum of {}",
                value.len(),
                MAX_STRING_LENGTH
            );
            record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
            value[0..MAX_LIST_LENGTH].to_vec()
        } else {
            value
        };

        let value = value
            .into_iter()
            .map(|elem| {
                if elem.len() > MAX_STRING_LENGTH {
                    let msg = format!(
                        "String list length of {} exceeds maximum of {}",
                        elem.len() + 1,
                        MAX_LIST_LENGTH
                    );
                    record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
                    elem[0..MAX_STRING_LENGTH].to_string()
                } else {
                    elem
                }
            })
            .collect();

        let value = Metric::StringList(value);
        glean.storage().record(&self.meta, &value);
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently-stored values.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<Vec<String>> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::StringList(values)) => Some(values),
            _ => None,
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently-stored values as a JSON String of the format
    /// ["string1", "string2", ...]
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_json_string(
        &self,
        glean: &Glean,
        storage_name: &str,
    ) -> Option<String> {
        self.test_get_value(glean, storage_name)
            .map(|values| serde_json::to_string(&values).unwrap())
    }
}
