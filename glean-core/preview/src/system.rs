// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Detect and expose `target_os` as a constant.
// Whether this is a good idea is somewhat debatable.
//
// Code adopted from the "platforms" crate: <https://github.com/RustSec/platforms-crate>.

#[cfg(target_os = "android")]
/// `target_os` when building this crate: `android`
pub const OS: &str = "Android";

#[cfg(target_os = "ios")]
/// `target_os` when building this crate: `ios`
pub const OS: &str = "iOS";

#[cfg(target_os = "linux")]
/// `target_os` when building this crate: `linux`
pub const OS: &str = "Linux";

#[cfg(target_os = "macos")]
/// `target_os` when building this crate: `macos`
pub const OS: &str = "MacOS";

#[cfg(target_os = "windows")]
/// `target_os` when building this crate: `windows`
pub const OS: &str = "Windows";

#[cfg(not(any(
    target_os = "android",
    target_os = "ios",
    target_os = "linux",
    target_os = "macos",
    target_os = "windows",
)))]
pub const OS: &str = "unknown";
