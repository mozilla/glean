// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{MetricType, StringListMetric};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{from_raw_string_array, RawStringArray, GLEAN};

lazy_static! {
    pub static ref STRING_LIST_METRICS: ConcurrentHandleMap<StringListMetric> =
        ConcurrentHandleMap::new();
}
define_handle_map_deleter!(STRING_LIST_METRICS, glean_destroy_string_list_metric);

#[no_mangle]
pub extern "C" fn glean_new_string_list_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
) -> u64 {
    STRING_LIST_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;

        Ok(StringListMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings,
            lifetime,
            disabled: disabled != 0,
        }))
    })
}

#[no_mangle]
pub extern "C" fn glean_string_list_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_LIST_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_string_list_add(glean_handle: u64, metric_id: u64, value: FfiStr) {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_LIST_METRICS.call_infallible(metric_id, |metric| {
            let value = value.into_string();
            metric.add(glean, value);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_string_list_set(
    glean_handle: u64,
    metric_id: u64,
    values: RawStringArray,
    values_len: i32,
) {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_LIST_METRICS.call_infallible(metric_id, |metric| {
            let values = unsafe { from_raw_string_array(values, values_len) };
            metric.set(glean, values);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_string_list_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        STRING_LIST_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_string_list_test_get_value_as_json_string(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        // Use get_u64 so we can coerce to the proper type for the cstring return.
        let res: glean_core::Result<String> = STRING_LIST_METRICS.get_u64(metric_id, |metric| {
            Ok(metric
                .test_get_value_as_json_string(glean, storage_name.as_str())
                .unwrap())
        });
        res.unwrap()
    })
}
