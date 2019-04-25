use serde::{Serialize, Deserialize};
use serde_json::{json, Value as JsonValue};

mod boolean;
mod string;
mod counter;
mod uuid;

pub use self::boolean::BooleanMetric;
pub use self::string::StringMetric;
pub use self::counter::CounterMetric;
pub use self::uuid::UuidMetric;

#[derive(Serialize, Deserialize, Debug)]
pub enum Metric {
    String(String),
    Boolean(bool),
    Counter(u64),
    Uuid(String),
}

impl Metric {
    pub fn category(&self) -> &'static str {
        match self {
            Metric::String(_) => "string",
            Metric::Boolean(_) => "boolean",
            Metric::Counter(_) => "counter",
            Metric::Uuid(_) => "uuid",
        }
    }

    pub fn as_json(&self) -> JsonValue {
        match self {
            Metric::String(s) => json!(s),
            Metric::Boolean(b) => json!(b),
            Metric::Counter(c) => json!(c),
            Metric::Uuid(s) => json!(s),
        }
    }
}
