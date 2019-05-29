// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # Metrics
//!
//! Glean supports different metric types to store data.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod boolean;
mod counter;
mod datetime;
mod labeled;
mod ping;
mod string;
mod string_list;
mod time_unit;
mod uuid;

use crate::util::get_iso_time_string;
use crate::CommonMetricData;
use crate::Glean;

pub use self::boolean::BooleanMetric;
pub use self::counter::CounterMetric;
pub use self::datetime::DatetimeMetric;
pub use self::labeled::LabeledMetric;
pub use self::ping::PingType;
pub use self::string::StringMetric;
pub use self::string_list::StringListMetric;
pub use self::time_unit::TimeUnit;
pub use self::uuid::UuidMetric;

/// The available metrics
///
/// This is the in-memory and persisted layout of a metric
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Metric {
    /// A boolean metric. See [`BooleanMetric`](struct.BooleanMetric.html) for more information.
    Boolean(bool),
    /// A counter metric. See [`CounterMetric`](struct.CounterMetric.html) for more information.
    Counter(i32),
    /// A datetime metric. See [`DatetimeMetric`](struct.DatetimeMetric.html) for more information.
    Datetime(DateTime<FixedOffset>, TimeUnit),
    /// A string metric. See [`StringMetric`](struct.StringMetric.html) for more information.
    String(String),
    /// A string list metric. See [`StringListMetric`](struct.StringListMetric.html) for more information.
    StringList(Vec<String>),
    /// A UUID metric. See [`UuidMetric`](struct.UuidMetric.html) for more information.
    Uuid(String),
}

/// A `MetricType` describes common behavior across all metrics
pub trait MetricType {
    /// Access the stored metadata
    fn meta(&self) -> &CommonMetricData;

    /// Access the stored metadata mutable
    fn meta_mut(&mut self) -> &mut CommonMetricData;

    /// Whether this metric should currently be recorded
    ///
    /// This depends on the metrics own state, as determined by its metadata,
    /// and whether upload is enabled on the Glean object.
    fn should_record(&self, glean: &Glean) -> bool {
        glean.is_upload_enabled() && self.meta().should_record()
    }
}

impl Metric {
    /// The category the metric fits into
    pub fn category(&self) -> &'static str {
        match self {
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
            Metric::Datetime(_, _) => "datetime",
            Metric::String(_) => "string",
            Metric::StringList(_) => "string_list",
            Metric::Uuid(_) => "uuid",
        }
    }

    /// The JSON representation of the metric's data
    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
            Metric::Datetime(d, tz) => json!(get_iso_time_string(*d, *tz)),
            Metric::String(s) => json!(s),
            Metric::StringList(v) => json!(v),
            Metric::Uuid(s) => json!(s),
        }
    }
}
