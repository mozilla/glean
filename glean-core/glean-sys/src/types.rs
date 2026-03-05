#[derive(uniffi::Enum)]
pub enum Lifetime {
    Ping,
    Application,
    User,
}

#[derive(uniffi::Enum)]
pub enum DynamicLabelType {
    Label(String),
    KeyOnly(String),
    CategoryOnly(String),
    KeyAndCategory(String),
}

#[derive(uniffi::Record)]
pub struct CommonMetricData {
    pub category: String,
    pub name: String,
    pub send_in_pings: Vec<String>,
    pub lifetime: Lifetime,
    pub disabled: bool,
    pub dynamic_label: Option<DynamicLabelType>,
}

#[derive(uniffi::Record)]
pub struct Rate {
    numerator: i32,
    denominator: i32,
}

pub type JsonValue = String;

#[derive(uniffi::Record)]
pub struct RecordedEvent {
    timestamp: u64,
    category: String,
    name: String,
    extra: Option<::std::collections::HashMap<String, String>>,
}

#[derive(uniffi::Record)]
pub struct Datetime {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    nanosecond: u32,
    offset_seconds: i32,
}

#[derive(uniffi::Record)]
pub struct DistributionData {
    values: ::std::collections::HashMap<i64, i64>,
    sum: i64,
    count: i64,
}

#[derive(uniffi::Record)]
pub struct TimerId {
    id: u64,
}

#[derive(uniffi::Enum)]
pub enum ErrorType {
    InvalidValue,
    InvalidLabel,
    InvalidState,
    InvalidOverflow,
}
