// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(clippy::redundant_closure)]

use std::os::raw::c_char;

use ffi_support::{
    define_handle_map_deleter, define_string_destructor, ConcurrentHandleMap, FfiStr,
};
use lazy_static::lazy_static;

use glean_core::Glean;

mod macros;

mod boolean;
mod counter;
mod datetime;
mod event;
mod handlemap_ext;
mod labeled;
mod ping_type;
mod string;
mod string_list;
mod timespan;
mod timing_distribution;
mod uuid;

use handlemap_ext::HandleMapExtension;
use ping_type::PING_TYPES;

lazy_static! {
    static ref GLEAN: ConcurrentHandleMap<Glean> = ConcurrentHandleMap::new();
}

type RawStringArray = *const *const c_char;

/// Create a vector of strings from a raw C-like string array
unsafe fn from_raw_string_array(arr: RawStringArray, len: i32) -> Vec<String> {
    if arr.is_null() || len == 0 {
        return vec![];
    }

    // FIXME: We should double check for null pointers and handle that instead of crashing
    let arr_ptrs = std::slice::from_raw_parts(arr, len as usize);
    arr_ptrs
        .iter()
        .map(|&p| FfiStr::from_raw(p).into_string())
        .collect()
}

type RawIntArray = *const i32;

/// Initialize the logging system based on the target platform. This ensures
/// that logging is shown when executing the Glean SDK unit tests.
#[no_mangle]
pub extern "C" fn glean_enable_logging() {
    #[cfg(target_os = "android")]
    {
        let _ = std::panic::catch_unwind(|| {
            android_logger::init_once(
                android_logger::Filter::default().with_min_level(log::Level::Debug),
                Some("libglean_ffi"),
            );
            log::debug!("Android logging should be hooked up!")
        });
    }
    // Make sure logging does something on non Android platforms as well. Use
    // the RUST_LOG environment variable to set the desired log level, e.g.
    // setting RUST_LOG=debug sets the log level to debug.
    #[cfg(not(target_os = "android"))]
    {
        match env_logger::try_init() {
            Ok(_) => log::debug!("stdout logging should be hooked up!"),
            // Please note that this is only expected to fail during unit tests,
            // where the logger might have already been initialized by a previous
            // test. So it's fine to print with the "logger".
            Err(_) => log::debug!("stdout was already initialized"),
        };
    }
}

#[no_mangle]
pub extern "C" fn glean_initialize(
    data_dir: FfiStr,
    application_id: FfiStr,
    upload_enabled: u8,
) -> u64 {
    GLEAN.insert_with_log(|| {
        let data_dir = data_dir.into_string();
        let application_id = application_id.into_string();
        let glean = Glean::new(&data_dir, &application_id, upload_enabled != 0)?;
        log::info!("Glean initialized");
        Ok(glean)
    })
}

#[no_mangle]
pub extern "C" fn glean_on_ready_to_send_pings(glean_handle: u64) {
    GLEAN.call_infallible(glean_handle, |glean| glean.on_ready_to_send_pings())
}

#[no_mangle]
pub extern "C" fn glean_is_upload_enabled(glean_handle: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| glean.is_upload_enabled())
}

#[no_mangle]
pub extern "C" fn glean_set_upload_enabled(glean_handle: u64, flag: u8) {
    GLEAN.call_infallible_mut(glean_handle, |glean| glean.set_upload_enabled(flag != 0))
}

#[no_mangle]
pub extern "C" fn glean_send_ping(glean_handle: u64, ping_type_handle: u64, log_ping: u8) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        PING_TYPES.call_with_log(ping_type_handle, |ping_type| {
            glean.send_ping(ping_type, log_ping != 0)
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_send_ping_by_name(
    glean_handle: u64,
    ping_name: FfiStr,
    log_ping: u8,
) -> u8 {
    GLEAN.call_with_log(glean_handle, |glean| {
        glean.send_ping_by_name(ping_name.as_str(), log_ping != 0)
    })
}

#[no_mangle]
pub extern "C" fn glean_ping_collect(glean_handle: u64, ping_type_handle: u64) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        let res: glean_core::Result<String> = PING_TYPES.get_u64(ping_type_handle, |ping_type| {
            let ping_maker = glean_core::ping::PingMaker::new();
            let data = ping_maker
                .collect_string(glean, ping_type)
                .unwrap_or_else(|| String::from(""));
            log::info!("Ping({}): {}", ping_type.name.as_str(), data);
            Ok(data)
        });
        res.unwrap()
    })
}

define_handle_map_deleter!(GLEAN, glean_destroy_glean);
define_string_destructor!(glean_str_free);
