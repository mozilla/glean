// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fmt;
use std::sync::Arc;

use crate::common_metric_data::CommonMetricDataInternal;
use crate::error_recording::{record_error, test_get_num_recorded_errors, ErrorType};
use crate::metrics::time_unit::TimeUnit;
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::util::{get_iso_time_string, local_now_with_offset};
use crate::CommonMetricData;
use crate::Glean;

use chrono::{DateTime, Datelike, FixedOffset, TimeZone, Timelike};
use time::{Date, OffsetDateTime, Time, UtcOffset};

/// Representation of a date, time and timezone.
#[derive(Clone, PartialEq, Eq)]
pub struct Datetime {
    /// The year, e.g. 2021.
    pub year: i32,
    /// The month, 1=January.
    pub month: u32,
    /// The day of the month.
    pub day: u32,
    /// The hour. 0-23
    pub hour: u32,
    /// The minute. 0-59.
    pub minute: u32,
    /// The second. 0-60.
    pub second: u32,
    /// The nanosecond part of the time.
    pub nanosecond: u32,
    /// The timezone offset from UTC in seconds.
    /// Negative for west, positive for east of UTC.
    pub offset_seconds: i32,
}

impl fmt::Debug for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Datetime({:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}{}{:02}{:02})",
            self.year,
            self.month,
            self.day,
            self.hour,
            self.minute,
            self.second,
            self.nanosecond,
            if self.offset_seconds < 0 { "-" } else { "+" },
            self.offset_seconds / 3600,        // hour part
            (self.offset_seconds % 3600) / 60, // minute part
        )
    }
}

impl Default for Datetime {
    fn default() -> Self {
        Datetime {
            year: 1970,
            month: 1,
            day: 1,
            hour: 0,
            minute: 0,
            second: 0,
            nanosecond: 0,
            offset_seconds: 0,
        }
    }
}

/// A datetime metric.
///
/// Used to record an absolute date and time, such as the time the user first ran
/// the application.
#[derive(Clone, Debug)]
pub struct DatetimeMetric {
    meta: Arc<CommonMetricDataInternal>,
    time_unit: TimeUnit,
}

impl MetricType for DatetimeMetric {
    fn meta(&self) -> &CommonMetricDataInternal {
        &self.meta
    }
}

impl From<OffsetDateTime> for Datetime {
    fn from(dt: OffsetDateTime) -> Self {
        let date = dt.date();
        let time = dt.time();
        let tz = dt.offset();
        Self {
            year: date.year(),
            month: date.month() as u32,
            day: date.day() as u32,
            hour: time.hour() as u32,
            minute: time.minute() as u32,
            second: time.second() as u32,
            nanosecond: time.nanosecond(),
            offset_seconds: tz.whole_seconds(),
        }
    }
}

// IMPORTANT:
//
// When changing this implementation, make sure all the operations are
// also declared in the related trait in `../traits/`.
impl DatetimeMetric {
    /// Creates a new datetime metric.
    pub fn new(meta: CommonMetricData, time_unit: TimeUnit) -> Self {
        Self {
            meta: Arc::new(meta.into()),
            time_unit,
        }
    }

    /// Sets the metric to a date/time including the timezone offset.
    ///
    /// # Arguments
    ///
    /// * `dt` - the optinal datetime to set this to. If missing the current date is used.
    pub fn set(&self, dt: Option<Datetime>) {
        let metric = self.clone();
        crate::launch_with_glean(move |glean| {
            metric.set_sync(glean, dt);
        })
    }

    /// Sets the metric to a date/time which including the timezone offset synchronously.
    ///
    /// Use [`set`](Self::set) instead.
    #[doc(hidden)]
    pub fn set_sync(&self, glean: &Glean, value: Option<Datetime>) {
        if !self.should_record(glean) {
            return;
        }

        let value = match value {
            None => local_now_with_offset(),
            Some(dt) => {
                let timezone_offset = UtcOffset::from_whole_seconds(dt.offset_seconds);
                if timezone_offset.is_err() {
                    let msg = format!(
                        "Invalid timezone offset {}. Not recording.",
                        dt.offset_seconds
                    );
                    record_error(glean, &self.meta, ErrorType::InvalidValue, msg, None);
                    return;
                };

                let date = Date::from_calendar_date(dt.year, (dt.month as u8).try_into().unwrap(), dt.day as u8).unwrap();
                let time = Time::from_hms_nano(dt.hour as u8, dt.minute as u8, dt.second as u8, dt.nanosecond).unwrap();
                let datetime_obj = OffsetDateTime::from_unix_timestamp(0)
                    .unwrap()
                    .replace_date(date)
                    .replace_time(time)
                    .replace_offset(timezone_offset.unwrap());

                datetime_obj
            }
        };

        self.set_sync_chrono(glean, value);
    }

    pub(crate) fn set_sync_chrono(&self, glean: &Glean, value: OffsetDateTime) {
        let value = Metric::Datetime(value.into(), self.time_unit);
        glean.storage().record(glean, &self.meta, &value)
    }

    /// Gets the stored datetime value.
    #[doc(hidden)]
    pub fn get_value<'a, S: Into<Option<&'a str>>>(
        &self,
        glean: &Glean,
        ping_name: S,
    ) -> Option<OffsetDateTime> {
        let (d, tu) = self.get_value_inner(glean, ping_name.into())?;

        // The string version of the test function truncates using string
        // parsing. Unfortunately `parse_from_str` errors with `NotEnough` if we
        // try to truncate with `get_iso_time_string` and then parse it back
        // in a `Datetime`. So we need to truncate manually.
        let time = d.time();
        let dt = match tu {
            TimeUnit::Nanosecond => d,
            TimeUnit::Microsecond => d.replace_nanosecond(0).unwrap(),
            TimeUnit::Millisecond => d.replace_microsecond(0).unwrap(),
            TimeUnit::Second => d.replace_millisecond(0).unwrap(),
            TimeUnit::Minute => d.replace_millisecond(0).unwrap().replace_second(0).unwrap(),
            TimeUnit::Hour => d
                .replace_millisecond(0)
                .unwrap()
                .replace_second(0)
                .unwrap()
                .replace_minute(0)
                .unwrap(),
            TimeUnit::Day => d
                .replace_millisecond(0)
                .unwrap()
                .replace_second(0)
                .unwrap()
                .replace_minute(0)
                .unwrap()
                .replace_hour(0)
                .unwrap(),
        };
        Some(dt)
    }

    fn get_value_inner(
        &self,
        glean: &Glean,
        ping_name: Option<&str>,
    ) -> Option<(OffsetDateTime, TimeUnit)> {
        let queried_ping_name = ping_name.unwrap_or_else(|| &self.meta().inner.send_in_pings[0]);

        match StorageManager.snapshot_metric(
            glean.storage(),
            queried_ping_name,
            &self.meta.identifier(glean),
            self.meta.inner.lifetime,
        ) {
            Some(Metric::Datetime(d, tu)) => Some((d.inner, tu)),
            _ => None,
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Gets the stored datetime value.
    ///
    /// The precision of this value is truncated to the `time_unit` precision.
    ///
    /// # Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `storage_name` - the storage name to look into.
    ///
    /// # Returns
    ///
    /// The stored value or `None` if nothing stored.
    pub fn test_get_value(&self, ping_name: Option<String>) -> Option<Datetime> {
        crate::block_on_dispatcher();
        crate::core::with_glean(|glean| {
            let dt = self.get_value(glean, ping_name.as_deref());
            dt.map(Datetime::from)
        })
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Gets the stored datetime value, formatted as an ISO8601 string.
    ///
    /// The precision of this value is truncated to the `time_unit` precision.
    ///
    /// # Arguments
    ///
    /// * `glean` - the Glean instance this metric belongs to.
    /// * `storage_name` - the storage name to look into.
    ///
    /// # Returns
    ///
    /// The stored value or `None` if nothing stored.
    pub fn test_get_value_as_string(&self, ping_name: Option<String>) -> Option<String> {
        crate::block_on_dispatcher();
        crate::core::with_glean(|glean| self.get_value_as_string(glean, ping_name))
    }

    /// **Test-only API**
    ///
    /// Gets the stored datetime value, formatted as an ISO8601 string.
    #[doc(hidden)]
    pub fn get_value_as_string(&self, glean: &Glean, ping_name: Option<String>) -> Option<String> {
        let value = self.get_value_inner(glean, ping_name.as_deref());
        value.map(|(dt, tu)| get_iso_time_string(dt, tu))
    }

    /// **Exported for test purposes.**
    ///
    /// Gets the number of recorded errors for the given metric and error type.
    ///
    /// # Arguments
    ///
    /// * `error` - The type of error
    /// * `ping_name` - represents the optional name of the ping to retrieve the
    ///   metric for. inner to the first value in `send_in_pings`.
    ///
    /// # Returns
    ///
    /// The number of errors reported.
    pub fn test_get_num_recorded_errors(&self, error: ErrorType) -> i32 {
        crate::block_on_dispatcher();

        crate::core::with_glean(|glean| {
            test_get_num_recorded_errors(glean, self.meta(), error).unwrap_or(0)
        })
    }
}
