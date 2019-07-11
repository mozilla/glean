// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! The different metric types supported by the Glean SDK to handle data.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod boolean;
mod counter;
mod datetime;
mod event;
mod experiment;mod labeled;
mod ping;
mod string;
mod string_list;
mod time_unit;
mod timespan;
mod timing_distribution;
mod uuid;

use crate::histogram::Histogram;
use crate::util::get_iso_time_string;
use crate::CommonMetricData;
use crate::Glean;

pub use self::boolean::BooleanMetric;
pub use self::counter::CounterMetric;
pub use self::datetime::DatetimeMetric;
pub use self::event::EventMetric;
pub use self::experiment::ExperimentMetric; // FIXME: should this be crate-only? But then how to run tests? Using Glean.set*?
pub use self::labeled::LabeledMetric;
pub use self::ping::PingType;
pub use self::string::StringMetric;
pub use self::string_list::StringListMetric;
pub use self::time_unit::TimeUnit;
pub use self::timespan::TimespanMetric;
pub use self::timing_distribution::TimerId;
pub use self::timing_distribution::TimingDistributionMetric;
pub use self::uuid::UuidMetric;

/// The available metrics.
///
/// This is the in-memory and persisted layout of a metric.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Metric {
    /// A boolean metric. See [`BooleanMetric`](struct.BooleanMetric.html) for more information.
    Boolean(bool),
    /// A counter metric. See [`CounterMetric`](struct.CounterMetric.html) for more information.
    Counter(i32),
    /// A datetime metric. See [`DatetimeMetric`](struct.DatetimeMetric.html) for more information.
    Datetime(DateTime<FixedOffset>, TimeUnit),
    /// An experiment metric. See [`ExperimentMetric`](struct.ExperimentMetric.html) for more information.
    Experiment(experiment::RecordedExperimentData),
    /// A string metric. See [`StringMetric`](struct.StringMetric.html) for more information.
    String(String),
    /// A string list metric. See [`StringListMetric`](struct.StringListMetric.html) for more information.
    StringList(Vec<String>),
    /// A UUID metric. See [`UuidMetric`](struct.UuidMetric.html) for more information.
    Uuid(String),
    /// A timespan metric. See [`TimespanMetric`](struct.TimespanMetric.html) for more information.
    Timespan(std::time::Duration, TimeUnit),
    /// A timing distribution. See [`TimingDistributionMetric`](struct.TimingDistributionMetric.html) for more information.
    TimingDistribution(Histogram, TimeUnit),
}

/// A `MetricType` describes common behavior across all metrics.
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
    /// The category the metric fits into.
    ///
    /// This determines the category to place the metric data in when assembling the ping payload.
    pub fn category(&self) -> &'static str {
        match self {
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
            Metric::Datetime(_, _) => "datetime",
            Metric::Experiment(_) => "experiments",
            Metric::String(_) => "string",
            Metric::StringList(_) => "string_list",
            Metric::Uuid(_) => "uuid",
            Metric::Timespan(..) => "timespan",
            Metric::TimingDistribution(..) => "timing_distribution",
        }
    }

    /// The JSON representation of the metric's data
    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
            Metric::Datetime(d, time_unit) => json!(get_iso_time_string(*d, *time_unit)),
            Metric::Experiment(e) => json!(e),
            Metric::String(s) => json!(s),
            Metric::StringList(v) => json!(v),
            Metric::Uuid(s) => json!(s),
            Metric::Timespan(time, time_unit) => {
                json!({"value": time_unit.duration_convert(*time), "time_unit": time_unit})
            }
            Metric::TimingDistribution(hist, time_unit) => {
                json!(timing_distribution::snapshot(hist, *time_unit))
            }
        }
    }
}
