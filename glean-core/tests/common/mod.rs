// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// #[allow(dead_code)] is required on this module as a workaround for
// https://github.com/rust-lang/rust/issues/46379
#![allow(dead_code)]
use glean_core::Glean;

use chrono;
use chrono::offset::TimeZone;
use iso8601;
use iso8601::Date::YMD;

pub fn tempdir() -> (tempfile::TempDir, String) {
    let t = tempfile::tempdir().unwrap();
    let name = t.path().display().to_string();
    (t, name)
}

pub const GLOBAL_APPLICATION_ID: &str = "org.mozilla.glean.test.app";

// Create a new instance of Glean with a temporary directory.
// We need to keep the `TempDir` alive, so that it's not deleted before we stop using it.
pub fn new_glean() -> (Glean, tempfile::TempDir) {
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().display().to_string();

    let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID, true).unwrap();

    (glean, dir)
}

/// Convert an iso8601::DateTime to a chrono::DateTime<FixedOffset>
pub fn iso8601_to_chrono(datetime: &iso8601::DateTime) -> chrono::DateTime<chrono::FixedOffset> {
    if let YMD { year, month, day } = datetime.date {
        return chrono::FixedOffset::east(datetime.time.tz_offset_hours * 3600)
            .ymd(year, month, day)
            .and_hms_milli(
                datetime.time.hour,
                datetime.time.minute,
                datetime.time.second,
                datetime.time.millisecond,
            );
    };
    panic!("Unsupported datetime format");
}
