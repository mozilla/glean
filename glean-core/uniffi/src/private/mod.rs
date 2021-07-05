// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

use crate::CommonMetricData;
use crate::Glean;

mod counter;
pub(crate) mod labeled;

pub use counter::CounterMetric;

/// The available metrics.
///
/// This is the in-memory and persisted layout of a metric.
///
/// ## Note
///
/// The order of metrics in this enum is important, as it is used for serialization.
/// Do not reorder the variants.
///
/// **Any new metric must be added at the end.**
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum Metric {
    /// A boolean metric. See [`BooleanMetric`] for more information.
    Boolean(bool),
    /// A counter metric. See [`CounterMetric`] for more information.
    Counter(i32),
}

/// A [`MetricType`] describes common behavior across all metrics.
pub trait MetricType {
    /// Access the stored metadata
    fn meta(&self) -> &CommonMetricData;

    fn with_name(&self, name: String) -> Self;
    fn with_dynamic_label(&self, label: String) -> Self;

    /// Whether this metric should currently be recorded
    ///
    /// This depends on the metrics own state, as determined by its metadata,
    /// and whether upload is enabled on the Glean object.
    fn should_record(&self, glean: &Glean) -> bool {
        glean.is_upload_enabled() && self.meta().should_record()
    }
}

impl Metric {
    /// Gets the ping section the metric fits into.
    ///
    /// This determines the section of the ping to place the metric data in when
    /// assembling the ping payload.
    pub fn ping_section(&self) -> &'static str {
        match self {
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
        }
    }

    /// The JSON representation of the metric's data
    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
        }
    }
}
