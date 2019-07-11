// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::os::raw::c_char;

use ffi_support::FfiStr;

use crate::{
    define_metric, from_raw_string_array, handlemap_ext::HandleMapExtension, RawStringArray, GLEAN,
};

define_metric!(StringListMetric => STRING_LIST_METRICS {
    new           -> glean_new_string_list_metric(),
    destroy       -> glean_destroy_string_list_metric,
    should_record -> glean_string_list_should_record,

    add -> glean_string_list_add(value: FfiStr),
});

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
