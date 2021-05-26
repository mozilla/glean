// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! # Platform Panic
//!
//! Some test functions may need to panic because they fail an invariant.
//! This system allows platforms to supply custom panic fns so we don't just
//! take down the process impolitely.

use once_cell::sync::Lazy;
use std::sync::RwLock;

type Callback = Box<dyn Fn(&str) + Send + Sync>;
static GLOBAL_PLATFORM_PANIC: Lazy<RwLock<Option<Callback>>> = Lazy::new(|| RwLock::new(None));

pub(crate) fn register<F: 'static>(panic_fn: F)
where
    F: Fn(&str) + Send + Sync,
{
    let mut gpp = GLOBAL_PLATFORM_PANIC.write().unwrap();
    *gpp = Some(Box::new(panic_fn));
}

pub(crate) fn panic(msg: &str) {
    if let Some(panic_fn) = &*GLOBAL_PLATFORM_PANIC.read().unwrap() {
        panic_fn(&msg);
    } else {
        panic!("{}", msg);
    }
}
