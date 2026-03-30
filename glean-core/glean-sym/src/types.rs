// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// Types defined in `glean.udl` and used in the public API.
///
/// For now these are a copy of the same types in `glean-core`.
/// We can probably generate (most of) them from the UDL definition as well
/// (or share the glean-core implementation otherwise),
/// but for the experimentation phase of `glean-sym` we stick with a manual copy to get it going.

#[derive(uniffi::Enum, Default)]
pub enum Lifetime {
    #[default]
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

#[derive(uniffi::Record, Default)]
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

#[derive(uniffi::Record, Debug)]
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

#[derive(uniffi::Enum)]
#[repr(i32)]
pub enum TimeUnit {
    /// Truncate to nanosecond precision.
    Nanosecond,
    /// Truncate to microsecond precision.
    Microsecond,
    /// Truncate to millisecond precision.
    Millisecond,
    /// Truncate to second precision.
    Second,
    /// Truncate to minute precision.
    Minute,
    /// Truncate to hour precision.
    Hour,
    /// Truncate to day precision.
    Day,
}

#[derive(uniffi::Enum)]
#[repr(i32)] // use i32 to be compatible with our JNA definition
pub enum MemoryUnit {
    /// 1 byte
    Byte,
    /// 2^10 bytes
    Kilobyte,
    /// 2^20 bytes
    Megabyte,
    /// 2^30 bytes
    Gigabyte,
}

#[derive(uniffi::Enum)]
pub enum HistogramType {
    /// A histogram with linear distributed buckets.
    Linear,
    /// A histogram with exponential distributed buckets.
    Exponential,
}

pub type CowString = std::borrow::Cow<'static, str>;
