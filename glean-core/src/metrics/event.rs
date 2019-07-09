// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::iter::Iterator;

use serde_json::json;

use crate::error_recording::{record_error, ErrorType};
use crate::event_database::RecordedEventData;
use crate::metrics::MetricType;
use crate::CommonMetricData;
use crate::Glean;

const MAX_LENGTH_EXTRA_KEY_VALUE: usize = 100;

/// An event metric.
///
/// Events allow recording of e.g. individual occurences of user actions, say
/// every time a view was open and from where. Each time you record an event, it
/// records a timestamp, the event's name and a set of custom values.
#[derive(Clone, Debug)]
pub struct EventMetric {
    meta: CommonMetricData,
    allowed_extra_keys: Vec<String>,
}

impl MetricType for EventMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl EventMetric {
    /// Create a new event metric.
    pub fn new(meta: CommonMetricData, allowed_extra_keys: Vec<String>) -> Self {
        Self {
            meta,
            allowed_extra_keys,
        }
    }

    /// Record an event.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `timestamp` - A monotonically increasing timestamp, in nanoseconds.
    ///   This must be provided since the actual recording of the event may
    ///   happen some time later than the moment the event occurred.
    /// * `extra` - A HashMap of (key, value) pairs. The key is an index
    ///   into the metric's `allowed_extra_keys` vector where the key's string
    ///   is looked up.
    pub fn record(&self, glean: &Glean, timestamp: u64, extra: Option<HashMap<i32, String>>) {
        if !self.should_record(glean) {
            return;
        }

        let extra_strings = extra.and_then(|extra| {
            Some(
                extra
                    .into_iter()
                    .map(|(k, v)| (self.allowed_extra_keys.get(k as usize).unwrap(), v))
                    .map(|(k, v)| (k.to_string(), self.truncate_value(glean, k, &v).to_string()))
                    .collect(),
            )
        });

        glean
            .event_storage()
            .record(glean, &self.meta, timestamp, extra_strings);
    }

    /// Truncates values to MAX_LENGTH_EXTRA_KEY_VALUE, and reports an error
    /// when doing so.
    fn truncate_value<'a>(&self, glean: &Glean, key: &str, value: &'a str) -> &'a str {
        if value.len() > MAX_LENGTH_EXTRA_KEY_VALUE {
            let msg = format!(
                "Value length {} for key {} exceeds maximum of {}",
                value.len(),
                key,
                MAX_LENGTH_EXTRA_KEY_VALUE
            );
            record_error(glean, &self.meta, ErrorType::InvalidValue, msg);
            &value[0..MAX_LENGTH_EXTRA_KEY_VALUE]
        } else {
            value
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Test whether there are currently stored events for this event metric.
    ///
    /// This doesn't clear the stored value.
    pub fn test_has_value(&self, glean: &Glean, store_name: &str) -> bool {
        glean.event_storage().test_has_value(&self.meta, store_name)
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the vector of currently stored events for this event metric.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(
        &self,
        glean: &Glean,
        store_name: &str,
    ) -> Option<Vec<RecordedEventData>> {
        glean.event_storage().test_get_value(&self.meta, store_name)
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored events for this event metric as a JSON-encoded string.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_json_string(&self, glean: &Glean, store_name: &str) -> Option<String> {
        self.test_get_value(glean, store_name)
            .and_then(|value| Some(json!(value).to_string()))
    }
}
