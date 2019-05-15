// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::error_recording::{record_error, ErrorType};
use crate::metrics::Metric;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

const MAX_LENGTH_VALUE: usize = 50;

#[derive(Debug)]
pub struct StringMetric {
    meta: CommonMetricData,
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, glean: &Glean, value: S) {
        if !self.meta.should_record() || !glean.is_upload_enabled() {
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
        let snapshot = match StorageManager.snapshot_as_json(glean.storage(), storage_name, false) {
            Some(snapshot) => snapshot,
            None => return None,
        };
        snapshot
            .as_object()
            .and_then(|o| o.get("string"))
            .and_then(|o| o.as_object())
            .and_then(|o| o.get(&self.meta.identifier()))
            .and_then(|o| o.as_str().map(|s| s.into()))
    }
}
