// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

use malloc_size_of::MallocSizeOfOps;

unsafe extern "C" fn size_of_op(_ptr: *const std::ffi::c_void) -> usize {
    0
}

/// Test scenario: Glean not initialized, `alloc_size` called.
#[test]
fn alloc_size_does_not_crash() {
    let mut ops = MallocSizeOfOps {
        size_of_op,
        enclosing_size_of_op: None,
    };
    assert_eq!(0, glean::alloc_size(&mut ops));
}
