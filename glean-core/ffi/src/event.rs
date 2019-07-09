// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{EventMetric, MetricType};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{
    from_raw_int_array_and_string_array, from_raw_string_array, RawIntArray, RawStringArray, GLEAN,
};

lazy_static! {
    pub static ref EVENT_METRICS: ConcurrentHandleMap<EventMetric> = ConcurrentHandleMap::new();
}
define_handle_map_deleter!(EVENT_METRICS, glean_destroy_event_metric);

#[no_mangle]
pub extern "C" fn glean_new_event_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
    extra_keys: RawStringArray,
    extra_keys_len: i32,
) -> u64 {
    EVENT_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;
        let extra_keys = unsafe { from_raw_string_array(extra_keys, extra_keys_len) };

        Ok(EventMetric::new(
            CommonMetricData {
                name: name.into_string(),
                category: category.into_string(),
                send_in_pings,
                lifetime,
                disabled: disabled != 0,
            },
            extra_keys,
        ))
    })
}

#[no_mangle]
pub extern "C" fn glean_event_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        EVENT_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_event_record(
    glean_handle: u64,
    metric_id: u64,
    timestamp: u64,
    extra_keys: RawIntArray,
    extra_values: RawStringArray,
    extra_len: i32,
) {
    GLEAN.call_infallible(glean_handle, |glean| {
        EVENT_METRICS.call_infallible(metric_id, |metric| {
            metric.record(glean, timestamp, unsafe {
                from_raw_int_array_and_string_array(extra_keys, extra_values, extra_len)
            });
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_event_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        EVENT_METRICS.call_infallible(metric_id, |metric| {
            metric.test_has_value(glean, storage_name.as_str())
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_event_test_get_value_as_json_string(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        let res: glean_core::Result<String> = EVENT_METRICS.get_u64(metric_id, |metric| {
            Ok(metric
                .test_get_value_as_json_string(glean, storage_name.as_str())
                .unwrap())
        });
        res.unwrap()
    })
}
