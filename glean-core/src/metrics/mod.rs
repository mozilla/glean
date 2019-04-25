use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};

mod boolean;
mod string;
mod counter;

pub use boolean::BooleanMetric;
pub use string::StringMetric;
pub use counter::CounterMetric;

#[derive(Serialize, Deserialize, Debug)]
pub enum Metric {
    String(String),
    Boolean(bool),
    Counter(u64),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::String(_) => "string",
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::String(s) => json!(s),
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
        }
    }
}
