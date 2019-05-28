// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use lazy_static::lazy_static;
use std::convert::TryFrom;

use glean_core::{metrics::*, CommonMetricData, Lifetime};

use super::handlemap_ext::HandleMapExtension;
use super::*;

macro_rules! impl_labeled_metric {
    ($metric:ty, $global:ident, $metric_global:ident, $new_name:ident, $destroy:ident, $get_name:ident) => {
        lazy_static! {
            static ref $global: ConcurrentHandleMap<LabeledMetric<$metric>> =
                ConcurrentHandleMap::new();
        }
        define_handle_map_deleter!($global, $destroy);

        #[no_mangle]
        pub extern "C" fn $new_name(
            category: FfiStr,
            name: FfiStr,
            send_in_pings: RawStringArray,
            send_in_pings_len: i32,
            lifetime: i32,
            disabled: u8,
            labels: RawStringArray,
            label_count: i32,
        ) -> u64 {
            $global.insert_with_log(|| {
                let send_in_pings =
                    unsafe { from_raw_string_array(send_in_pings, send_in_pings_len) };
                let labels = unsafe { from_raw_string_array(labels, label_count) };
                let labels = if labels.is_empty() {
                    None
                } else {
                    Some(labels)
                };
                let lifetime = Lifetime::try_from(lifetime)?;

                Ok(LabeledMetric::new(
                    CommonMetricData {
                        name: name.into_string(),
                        category: category.into_string(),
                        send_in_pings,
                        lifetime,
                        disabled: disabled != 0,
                    },
                    labels,
                ))
            })
        }

        #[no_mangle]
        pub extern "C" fn $get_name(glean_handle: u64, handle: u64, label: FfiStr) -> u64 {
            GLEAN.call_infallible(glean_handle, |glean| {
                $global.call_infallible_mut(handle, |labeled| {
                    let metric = labeled.get(glean, label.as_str());
                    $metric_global.insert_with_log(|| Ok(metric))
                })
            })
        }
    };
}

impl_labeled_metric!(
    CounterMetric,
    LABELED_COUNTER,
    COUNTER_METRICS,
    glean_new_labeled_counter_metric,
    glean_destroy_labeled_counter_metric,
    glean_labeled_counter_metric_get
);

impl_labeled_metric!(
    BooleanMetric,
    LABELED_BOOLEAN,
    BOOLEAN_METRICS,
    glean_new_labeled_boolean_metric,
    glean_destroy_labeled_boolean_metric,
    glean_labeled_boolean_metric_get
);

impl_labeled_metric!(
    StringMetric,
    LABELED_STRING,
    STRING_METRICS,
    glean_new_labeled_string_metric,
    glean_destroy_labeled_string_metric,
    glean_labeled_string_metric_get
);
