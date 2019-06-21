// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::time::Duration;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{MetricType, TimeUnit, TimespanMetric};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{from_raw_string_array, RawStringArray, GLEAN};

lazy_static! {
    pub static ref TIMESPAN_METRICS: ConcurrentHandleMap<TimespanMetric> =
        ConcurrentHandleMap::new();
}
define_handle_map_deleter!(TIMESPAN_METRICS, glean_destroy_timespan_metric);

#[no_mangle]
pub extern "C" fn glean_new_timespan_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
    time_unit: i32,
) -> u64 {
    TIMESPAN_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;
        let tu = TimeUnit::try_from(time_unit)?;

        Ok(TimespanMetric::new(
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
pub extern "C" fn glean_timespan_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_set_start(glean_handle: u64, metric_id: u64, start_time: u64) {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
            metric.set_start(glean, start_time);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_set_stop(glean_handle: u64, metric_id: u64, stop_time: u64) {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
            metric.set_stop(glean, stop_time);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_cancel(metric_id: u64) {
    TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
        metric.cancel();
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_set_raw_nanos(
    glean_handle: u64,
    metric_id: u64,
    elapsed_nanos: u64,
) {
    let elapsed = Duration::from_nanos(elapsed_nanos);
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
            metric.set_raw(glean, elapsed, true);
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_timespan_test_get_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u64 {
    GLEAN.call_infallible(glean_handle, |glean| {
        TIMESPAN_METRICS.call_infallible(metric_id, |metric| {
            metric.test_get_value(glean, storage_name.as_str()).unwrap()
        })
    })
}
