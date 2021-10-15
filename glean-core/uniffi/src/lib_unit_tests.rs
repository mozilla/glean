// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// NOTE: This is a test-only file that contains unit tests for
// the lib.rs file.

use super::*;

const GLOBAL_APPLICATION_ID: &str = "org.mozilla.glean.test.app";
pub fn new_glean(tempdir: Option<tempfile::TempDir>) -> (Glean, tempfile::TempDir) {
    let dir = match tempdir {
        Some(tempdir) => tempdir,
        None => tempfile::tempdir().unwrap(),
    };
    let tmpname = dir.path().display().to_string();
    let glean = Glean::with_options(&tmpname, GLOBAL_APPLICATION_ID, true);
    (glean, dir)
}
