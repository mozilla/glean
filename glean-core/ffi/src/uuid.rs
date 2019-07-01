// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::convert::TryFrom;
use std::os::raw::c_char;

use ffi_support::{define_handle_map_deleter, ConcurrentHandleMap, FfiStr};
use lazy_static::lazy_static;

use glean_core::metrics::{MetricType, UuidMetric};
use glean_core::{CommonMetricData, Lifetime};

use crate::handlemap_ext::HandleMapExtension;
use crate::{from_raw_string_array, RawStringArray, GLEAN};

lazy_static! {
    static ref UUID_METRICS: ConcurrentHandleMap<UuidMetric> = ConcurrentHandleMap::new();
}
define_handle_map_deleter!(UUID_METRICS, glean_destroy_uuid_metric);

#[no_mangle]
pub extern "C" fn glean_new_uuid_metric(
    category: FfiStr,
    name: FfiStr,
    send_in_pings: RawStringArray,
    send_in_pings_len: i32,
    lifetime: i32,
    disabled: u8,
) -> u64 {
    UUID_METRICS.insert_with_log(|| {
        let send_in_pings = unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
        let lifetime = Lifetime::try_from(lifetime)?;

        Ok(UuidMetric::new(CommonMetricData {
            name: name.into_string(),
            category: category.into_string(),
            send_in_pings,
            lifetime,
            disabled: disabled != 0,
        }))
    })
}

#[no_mangle]
pub extern "C" fn glean_uuid_should_record(glean_handle: u64, metric_id: u64) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        UUID_METRICS.call_infallible(metric_id, |metric| metric.should_record(&glean))
    })
}

#[no_mangle]
pub extern "C" fn glean_uuid_set(glean_handle: u64, metric_id: u64, value: FfiStr) {
    GLEAN.call_infallible(glean_handle, |glean| {
        UUID_METRICS.call_infallible(metric_id, |metric| {
            let uuid = uuid::Uuid::parse_str(&value.into_string());
            metric.set(glean, uuid.unwrap());
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_uuid_test_has_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> u8 {
    GLEAN.call_infallible(glean_handle, |glean| {
        UUID_METRICS.call_infallible(metric_id, |metric| {
            metric
                .test_get_value(glean, storage_name.as_str())
                .is_some()
        })
    })
}

#[no_mangle]
pub extern "C" fn glean_uuid_test_get_value(
    glean_handle: u64,
    metric_id: u64,
    storage_name: FfiStr,
) -> *mut c_char {
    GLEAN.call_infallible(glean_handle, |glean| {
        let res: glean_core::Result<String> = UUID_METRICS.get_u64(metric_id, |metric| {
            Ok(metric.test_get_value(glean, storage_name.as_str()).unwrap())
        });
        res.unwrap()
    })
}
