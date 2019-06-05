// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Metric {
    Boolean(bool),
    Counter(i32),
    Datetime(DateTime<FixedOffset>, TimeUnit),
    String(String),
    StringList(Vec<String>),
    Uuid(String),
}

pub trait MetricType {
    fn meta(&self) -> &CommonMetricData;

    fn meta_mut(&mut self) -> &mut CommonMetricData;

    fn should_record(&self, glean: &Glean) -> bool {
        glean.is_upload_enabled() && self.meta().should_record()
    }
}

impl Metric {
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

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
            Metric::Datetime(d, time_unit) => json!(get_iso_time_string(*d, *time_unit)),
            Metric::String(s) => json!(s),
            Metric::StringList(v) => json!(v),
            Metric::Uuid(s) => json!(s),
        }
    }
}
