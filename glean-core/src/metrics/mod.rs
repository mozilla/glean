// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};

mod boolean;
mod counter;
mod string;
mod string_list;
mod uuid;

pub use self::boolean::BooleanMetric;
pub use self::counter::CounterMetric;
pub use self::string::StringMetric;
pub use self::string_list::StringListMetric;
pub use self::uuid::UuidMetric;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    String(String),
    Boolean(bool),
    Counter(i32),
    Uuid(String),
    StringList(Vec<String>),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::String(_) => "string",
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
            Metric::Uuid(_) => "uuid",
            Metric::StringList(_) => "string_list",
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::String(s) => json!(s),
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
            Metric::Uuid(s) => json!(s),
            Metric::StringList(v) => json!(v),
        }
    }
}
