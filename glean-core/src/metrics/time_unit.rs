// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;

use crate::error::{Error, ErrorKind};

/// Enumeration of different resolutions supported by the
/// time related metric types (e.g. DatetimeMetric).
//#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[derive(Debug)]
pub enum TimeUnit {
    Nanosecond,
    Microsecond,
    Millisecond,
    Second,
    Minute,
    Hour,
    Day
}
/*
impl Default for Lifetime {
    fn default() -> Self {
        Lifetime::Ping
    }
}

impl Lifetime {
    pub fn as_str(self) -> &'static str {
        match self {
            Lifetime::Ping => "ping",
            Lifetime::Application => "app",
            Lifetime::User => "user",
        }
    }
}*/

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
            e => Err(ErrorKind::TimeUnit(e))?,
        }
    }
}
