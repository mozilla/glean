// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

use glean_ffi_sys::GleanSys;
use once_cell::sync::OnceCell;

use crate::dispatcher;

static GLEAN_SYS: OnceCell<Mutex<GleanSys>> = OnceCell::new();
pub static NEEDS_FLUSH: AtomicBool = AtomicBool::new(false);

pub fn setup_glean(libname: &str) -> Result<(), ::libloading::Error> {
    let glean = unsafe { GleanSys::new(libname) };
    match glean {
        Ok(glean) => {
            if GLEAN_SYS.set(Mutex::new(glean)).is_err() {
                log::warn!(
                    "Global Glean-sys object is initialized already. This probably happened concurrently."
                )
            } else {
                log::info!("glean-sys setup done. dynamic Glean usable now");

                if NEEDS_FLUSH.swap(false, Ordering::SeqCst) {
                    if let Err(err) = dispatcher::flush_init() {
                        log::error!("Unable to flush the preinit queue: {}", err);
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            log::info!(
                "glean-sys not loaded. No Glean functionality will be available. Error: {:?}",
                e
            );
            Err(e)
        }
    }
}

/// Gets a reference to the global Glean object.
pub fn global_glean_sys() -> Option<&'static Mutex<GleanSys>> {
    GLEAN_SYS.get()
}

pub fn with_glean<F, R>(f: F) -> Option<R>
where
    F: FnOnce(&GleanSys) -> R,
{
    log::info!("Getting global glean-sys");
    let glean = match global_glean_sys() {
        Some(glean) => glean,
        None => {
            log::warn!("No global glean-sys found. Returning None.");
            return None;
        }
    };
    log::info!("Got global glean-sys, running user function.");

    let lock = glean.lock().unwrap();
    Some(f(&lock))
}
