// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{DatetimeMetric, MetricType, TimeUnit};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{from_raw_string_array, RawStringArray, GLEAN};

lazy_static! {
    static ref DATETIME_METRICS: ConcurrentHandleMap<DatetimeMetric> = ConcurrentHandleMap::new();
}
define_handle_map_deleter!(DATETIME_METRICS, glean_destroy_datetime_metric);

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
