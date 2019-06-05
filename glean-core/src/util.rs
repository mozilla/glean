// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use chrono::offset::TimeZone;
use chrono::{DateTime, FixedOffset, Local};

use crate::metrics::TimeUnit;

/// Sanitizes the application id, generating a pipeline-friendly string that replaces
/// non alphanumeric characters with dashes.
pub fn sanitize_application_id(application_id: &str) -> String {
    let mut last_dash = false;
    application_id
        .chars()
        .filter_map(|x| match x {
            'A'...'Z' | 'a'...'z' | '0'...'9' => {
                last_dash = false;
                Some(x)
            }
            _ => {
                let result = if last_dash { None } else { Some('-') };
                last_dash = true;
                result
            }
        })
        .collect()
}

/// Generate an ISO8601 compliant date/time string for the given time, truncating
/// it to the provided TimeUnit.
///
/// ## Arguments:
///
/// * `datetime`: the `DateTime` object that holds the date, time and timezone information.
/// * `truncate_to`: the desired resolution to use for the output string.
///
/// ## Return value:
///
/// Returns a string representing the provided date/time truncated to the requested time unit.
pub fn get_iso_time_string(datetime: DateTime<FixedOffset>, truncate_to: TimeUnit) -> String {
    datetime.format(truncate_to.format_pattern()).to_string()
}

/// Get the current date & time with a fixed-offset timezone.
///
/// This converts from the `Local` timezone into its fixed-offset equivalent.
pub(crate) fn local_now_with_offset() -> DateTime<FixedOffset> {
    // This looks more complicated than I imagined.

    let now: DateTime<Local> = Local::now();
    let naive = now.naive_utc();
    let fixed_tz = Local.offset_from_utc_datetime(&naive);
    fixed_tz.from_utc_datetime(&naive)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sanitize_application_id() {
        assert_eq!(
            "org-mozilla-test-app",
            sanitize_application_id("org.mozilla.test-app")
        );
        assert_eq!(
            "org-mozilla-test-app",
            sanitize_application_id("org.mozilla..test---app")
        );
        assert_eq!(
            "org-mozilla-test-app",
            sanitize_application_id("org-mozilla-test-app")
        );
    }

    #[test]
    fn test_get_iso_time_string() {
        // `1985-07-03T12:09:14.000560274+01:00`
        let dt = FixedOffset::east(3600)
            .ymd(1985, 7, 3)
            .and_hms_nano(12, 9, 14, 1_560_274);
        assert_eq!(
            "1985-07-03T12:09:14.001560274+01:00",
            get_iso_time_string(dt, TimeUnit::Nanosecond)
        );
        assert_eq!(
            "1985-07-03T12:09:14.001560+01:00",
            get_iso_time_string(dt, TimeUnit::Microsecond)
        );
        assert_eq!(
            "1985-07-03T12:09:14.001+01:00",
            get_iso_time_string(dt, TimeUnit::Millisecond)
        );
        assert_eq!(
            "1985-07-03T12:09:14+01:00",
            get_iso_time_string(dt, TimeUnit::Second)
        );
        assert_eq!(
            "1985-07-03T12:09+01:00",
            get_iso_time_string(dt, TimeUnit::Minute)
        );
        assert_eq!(
            "1985-07-03T12+01:00",
            get_iso_time_string(dt, TimeUnit::Hour)
        );
        assert_eq!("1985-07-03+01:00", get_iso_time_string(dt, TimeUnit::Day));
    }

    #[test]
    fn local_now_gets_the_time() {
        let now = Local::now();
        let fixed_now = local_now_with_offset();

        // We can't compare across differing timezones, so we just compare the UTC timestamps.
        // The second timestamp should be just a few nanoseconds later.
        assert!(
            fixed_now.naive_utc() >= now.naive_utc(),
            "Time mismatch. Local now: {:?}, Fixed now: {:?}",
            now,
            fixed_now
        );
    }
}
