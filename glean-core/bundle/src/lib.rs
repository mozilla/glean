/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

pub use glean_core;

/// Workaround to force a re-export of the `no_mangle` symbols from `glean_core`
///
/// Due to how linking works and hides symbols the symbols from `glean_core` might not be
/// re-exported and thus not usable.
/// By forcing use of _at least one_ symbol in an exported function the functions will also be
/// rexported.
/// This is only required for debug builds (and `debug_assertions` is the closest thing we have to
/// check that).
/// In release builds we rely on LTO builds to take care of it.
/// Our tests should ensure this actually happens.
///
/// See https://github.com/rust-lang/rust/issues/50007
#[cfg(debug_assertions)]
#[no_mangle]
pub extern "C" fn _glean_force_reexport_donotcall() {
    glean_core::glean_enable_logging();
}
