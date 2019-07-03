// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::os::raw::c_char;

use ffi_support::FfiStr;

use crate::{define_metric, handlemap_ext::HandleMapExtension, GLEAN};
use glean_core::metrics::TimerId;

define_metric!(TimingDistributionMetric => TIMING_DISTRIBUTION_METRICS {
    new           -> glean_new_timing_distribution_metric(time_unit: i32),
    destroy       -> glean_destroy_timing_distribution_metric,
    should_record -> glean_timing_distribution_should_record,
});

#[no_mangle]
pub extern "C" fn glean_timing_distribution_set_start(
    glean_handle: u64,
    metric_id: u64,
    start_time: u64,
) -> TimerId {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMING_DISTRIBUTION_METRICS
            .call_infallible_mut(metric_id, |metric| metric.set_start(glean, start_time))
    })
}

#[no_mangle]
pub extern "C" fn glean_timing_distribution_set_stop_and_accumulate(
    glean_handle: u64,
    metric_id: u64,
    timer_id: TimerId,
    stop_time: u64,
) {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMING_DISTRIBUTION_METRICS.call_infallible_mut(metric_id, |metric| {
            metric.set_stop_and_accumulate(glean, timer_id, stop_time);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timing_distribution_cancel(metric_id: u64, timer_id: TimerId) {
    TIMING_DISTRIBUTION_METRICS.call_infallible_mut(metric_id, |metric| {
        metric.cancel(timer_id);
    })
}

#[no_mangle]
pub extern "C" fn glean_timing_distribution_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMING_DISTRIBUTION_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timing_distribution_test_get_value_as_json_string(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        // Use get_u64 so we can coerce to the proper type for the cstring return.
        let res: glean_core::Result<String> =
            TIMING_DISTRIBUTION_METRICS.get_u64(metric_id, |metric| {
                Ok(metric
                    .test_get_value_as_json_string(glean, storage_name.as_str())
                    .unwrap())
            });
        res.unwrap()
    })
}
