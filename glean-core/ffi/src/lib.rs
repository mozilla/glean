// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(clippy::redundant_closure)]

use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{
    define_handle_map_deleter, define_string_destructor, ConcurrentHandleMap, FfiStr,
};
use lazy_static::lazy_static;

use glean_core::{metrics::*, CommonMetricData, Glean, Lifetime};

mod handlemap_ext;
mod labeled;

use handlemap_ext::HandleMapExtension;
pub use labeled::*;

lazy_static! {
    static ref GLEAN: ConcurrentHandleMap<Glean> = ConcurrentHandleMap::new();
    static ref PING_TYPES: ConcurrentHandleMap<PingType> = ConcurrentHandleMap::new();
    static ref BOOLEAN_METRICS: ConcurrentHandleMap<BooleanMetric> = ConcurrentHandleMap::new();
    static ref COUNTER_METRICS: ConcurrentHandleMap<CounterMetric> = ConcurrentHandleMap::new();
    static ref DATETIME_METRICS: ConcurrentHandleMap<DatetimeMetric> = ConcurrentHandleMap::new();
    static ref STRING_METRICS: ConcurrentHandleMap<StringMetric> = ConcurrentHandleMap::new();
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

/// Initialize the logging system based on the target platform. This ensures
/// that logging is shown when executing glean unit tests.
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
pub extern "C" fn glean_is_initialized(glean_handle: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| glean.is_initialized())
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
pub extern "C" fn glean_new_ping_type(ping_name: FfiStr, include_client_id: u8) -> u64 {
    PING_TYPES.insert_with_log(|| Ok(PingType::new(ping_name.as_str(), include_client_id != 0)))
}

#[no_mangle]
pub extern "C" fn glean_test_has_ping_type(glean_handle: u64, ping_name: FfiStr) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        glean.get_ping_by_name(ping_name.as_str()).is_some()
    })
}

#[no_mangle]
pub extern "C" fn glean_register_ping_type(glean_handle: u64, ping_type_handle: u64) {
    PING_TYPES.call_infallible(ping_type_handle, |ping_type| {
        GLEAN.call_infallible_mut(glean_handle, |glean| glean.register_ping_type(ping_type))
    })
}

#[no_mangle]
pub extern "C" fn glean_new_boolean_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
) -> u64 {
    BOOLEAN_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;

        Ok(BooleanMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings,
            lifetime,
            disabled: disabled != 0,
        }))
    })
}

#[no_mangle]
pub extern "C" fn glean_new_string_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
) -> u64 {
    STRING_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;

        Ok(StringMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings,
            lifetime,
            disabled: disabled != 0,
        }))
    })
}

#[no_mangle]
pub extern "C" fn glean_new_counter_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
) -> u64 {
    COUNTER_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;

        Ok(CounterMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings,
            lifetime,
            disabled: disabled != 0,
        }))
    })
}

#[no_mangle]
pub extern "C" fn glean_counter_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        COUNTER_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_counter_add(glean_handle: u64, metric_id: u64, amount: i32) {
    GLEAN.call_infallible(glean_handle, |glean| {
        COUNTER_METRICS.call_infallible(metric_id, |metric| {
            metric.add(glean, amount);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_counter_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        COUNTER_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_counter_test_get_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> i32 {
    GLEAN.call_infallible(glean_handle, |glean| {
        COUNTER_METRICS.call_infallible(metric_id, |metric| {
            metric.test_get_value(glean, storage_name.as_str()).unwrap()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_boolean_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        BOOLEAN_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_boolean_set(glean_handle: u64, metric_id: u64, value: u8) {
    GLEAN.call_infallible(glean_handle, |glean| {
        BOOLEAN_METRICS.call_infallible(metric_id, |metric| {
            metric.set(glean, value != 0);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_boolean_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        BOOLEAN_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_boolean_test_get_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        BOOLEAN_METRICS.call_infallible(metric_id, |metric| {
            metric.test_get_value(glean, storage_name.as_str()).unwrap()
        })
    })
}

// *** Start of the Datetime FFI part ***

#[no_mangle]
pub extern "C" fn glean_new_datetime_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
    time_unit: i32,
) -> u64 {
    DATETIME_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;
        let tu = TimeUnit::try_from(time_unit)?;

        Ok(DatetimeMetric::new(
            CommonMetricData {
                name: name.into_string(),
                category: category.into_string(),
                send_in_pings,
                lifetime,
                disabled: disabled != 0,
            },
            tu,
        ))
    })
}

#[no_mangle]
pub extern "C" fn glean_datetime_set(
    glean_handle: u64,
    metric_id: u64,
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    nano: i64,
    offset_seconds: i32,
) {
    // Convert and truncate the nanos to u32, as that's what the underlying
    // library uses. Unfortunately, not all platform have unsigned integers
    // so we need to work with what we have.
    if nano < 0 || nano > i64::from(std::u32::MAX) {
        log::error!("Unexpected `nano` value coming from platform code {}", nano);
        return;
    }

    // We are within the u32 boundaries for nano, we should be ok converting.
    let converted_nanos = nano as u32;
    GLEAN.call_infallible(glean_handle, |glean| {
        DATETIME_METRICS.call_infallible(metric_id, |metric| {
            metric.set_with_details(
                glean,
                year,
                month,
                day,
                hour,
                minute,
                second,
                converted_nanos,
                offset_seconds,
            );
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_datetime_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        DATETIME_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_datetime_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        DATETIME_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value_as_string(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_datetime_test_get_value_as_string(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        let res: glean_core::Result<String> = DATETIME_METRICS.get_u64(metric_id, |metric| {
            Ok(metric
                .test_get_value_as_string(glean, storage_name.as_str())
                .unwrap())
        });
        res.unwrap()
    })
}

// *** End of the Datetime FFI part ***

#[no_mangle]
pub extern "C" fn glean_string_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_string_set(glean_handle: u64, metric_id: u64, value: FfiStr) {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_METRICS.call_infallible(metric_id, |metric| {
            let value = value.into_string();
            metric.set(glean, value);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_string_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_string_test_get_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        let res: glean_core::Result<String> = STRING_METRICS.get_u64(metric_id, |metric| {
            Ok(metric.test_get_value(glean, storage_name.as_str()).unwrap())
        });
        res.unwrap()
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
define_handle_map_deleter!(PING_TYPES, glean_destroy_ping_type);
define_handle_map_deleter!(BOOLEAN_METRICS, glean_destroy_boolean_metric);
define_handle_map_deleter!(STRING_METRICS, glean_destroy_string_metric);
define_handle_map_deleter!(COUNTER_METRICS, glean_destroy_counter_metric);
define_handle_map_deleter!(DATETIME_METRICS, glean_destroy_datetime_metric);
define_string_destructor!(glean_str_free);
