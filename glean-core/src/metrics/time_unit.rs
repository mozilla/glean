// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use time::macros::format_description;
use time::format_description::FormatItem;

use crate::error::{Error, ErrorKind};

/// Different resolutions supported by the time related
/// metric types (e.g. DatetimeMetric).
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
#[repr(i32)] // use i32 to be compatible with our JNA definition
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

impl TimeUnit {
    /// Formats the given time unit, truncating the time if needed.
    pub fn format_pattern(self) -> &'static [FormatItem<'static>] {
        use TimeUnit::*;
        match self {
            Nanosecond => format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
            Microsecond => format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
            Millisecond => format_description!("[year]-[month]-[day]T[hour]:[minute]:[second].[[subsecond][offset_hour sign:mandatory]:[offset_minute]"),
            Second => format_description!("[year]-[month]-[day]T[hour]:[minute]:[second][offset_hour sign:mandatory]:[offset_minute]"),
            Minute => format_description!("[year]-[month]-[day]T[hour]:[minute][offset_hour sign:mandatory]:[offset_minute]"),
            Hour => format_description!("[year]-[month]-[day]T[hour][offset_hour sign:mandatory]:[offset_minute]"),
            Day => format_description!("[year]-[month]-[day][offset_hour sign:mandatory]:[offset_minute]"),
        }
    }

    /// Converts a duration to the requested time unit.
    ///
    /// # Arguments
    ///
    /// * `duration` - the duration to convert.
    ///
    /// # Returns
    ///
    /// The integer representation of the converted duration.
    pub fn duration_convert(self, duration: Duration) -> u64 {
        use TimeUnit::*;
        match self {
            Nanosecond => duration.as_nanos() as u64,
            Microsecond => duration.as_micros() as u64,
            Millisecond => duration.as_millis() as u64,
            Second => duration.as_secs(),
            Minute => duration.as_secs() / 60,
            Hour => duration.as_secs() / 60 / 60,
            Day => duration.as_secs() / 60 / 60 / 24,
        }
    }

    /// Converts a duration in the given unit to nanoseconds.
    ///
    /// # Arguments
    ///
    /// * `duration` - the duration to convert.
    ///
    /// # Returns
    ///
    /// The integer representation of the nanosecond duration.
    pub fn as_nanos(self, duration: u64) -> u64 {
        use TimeUnit::*;
        let duration = match self {
            Nanosecond => Duration::from_nanos(duration),
            Microsecond => Duration::from_micros(duration),
            Millisecond => Duration::from_millis(duration),
            Second => Duration::from_secs(duration),
            Minute => Duration::from_secs(duration * 60),
            Hour => Duration::from_secs(duration * 60 * 60),
            Day => Duration::from_secs(duration * 60 * 60 * 24),
        };

        duration.as_nanos() as u64
    }
}

/// Trait implementation for converting an integer value to a TimeUnit.
///
/// This is used in the FFI code.
///
/// Please note that values should match the ordering of the
/// platform specific side of things (e.g. Kotlin implementation).
impl TryFrom<i32> for TimeUnit {
    type Error = Error;

    fn try_from(value: i32) -> Result<TimeUnit, Self::Error> {
        match value {
            0 => Ok(TimeUnit::Nanosecond),
            1 => Ok(TimeUnit::Microsecond),
            2 => Ok(TimeUnit::Millisecond),
            3 => Ok(TimeUnit::Second),
            4 => Ok(TimeUnit::Minute),
            5 => Ok(TimeUnit::Hour),
            6 => Ok(TimeUnit::Day),
            e => Err(ErrorKind::TimeUnit(e).into()),
        }
    }
}
