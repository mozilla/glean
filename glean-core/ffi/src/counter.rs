// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{CounterMetric, MetricType};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{from_raw_string_array, RawStringArray, GLEAN};

lazy_static! {
    pub static ref COUNTER_METRICS: ConcurrentHandleMap<CounterMetric> = ConcurrentHandleMap::new();
}
define_handle_map_deleter!(COUNTER_METRICS, glean_destroy_counter_metric);

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
