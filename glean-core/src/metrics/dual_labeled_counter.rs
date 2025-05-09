// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;
use std::collections::HashMap;
use std::mem;
use std::sync::{Arc, Mutex};

use malloc_size_of::MallocSizeOf;

use crate::common_metric_data::{CommonMetricData, CommonMetricDataInternal, DynamicLabelType};
use crate::error_recording::{record_error, test_get_num_recorded_errors, ErrorType};
use crate::metrics::{CounterMetric, Metric, MetricType};
use crate::Glean;

const MAX_LABELS: usize = 16;
const OTHER_LABEL: &str = "__other__";
const MAX_LABEL_LENGTH: usize = 111;
const RECORD_SEPARATOR: u8 = 0x1E;

pub trait DualLabeledMetricType {
    /// Create a new metric from this with a specific label.
    fn with_dynamic_key(&self, _key: String) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    /// Create a new metric from this with a specific label.
    fn with_dynamic_category(&self, _category: String) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }

    fn with_dynamic_key_and_category(&self, _key: String, _category: String) -> Self
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

/// A dual labled metric, mirrors the Legacy keyed-categorical type.
///
/// Dual labled metrics allow recording multiple sub-metrics of the same type, in relation
/// to two dimensions rather than the single label provided by the standard labeled type.
#[derive(Debug)]
pub struct DualLabeledCounterMetric {
    keys: Option<Vec<Cow<'static, str>>>,
    categories: Option<Vec<Cow<'static, str>>>,
    /// Type of the underlying metric
    /// We hold on to an instance of it, which is cloned to create new modified instances.
    counter: CounterMetric,

    /// A map from a unique ID for the dual labeled submetric to a handle of an instantiated
    /// metric type.
    dual_label_map: Mutex<HashMap<(String, String), Arc<CounterMetric>>>,
}

impl ::malloc_size_of::MallocSizeOf for DualLabeledCounterMetric {
    fn size_of(&self, ops: &mut malloc_size_of::MallocSizeOfOps) -> usize {
        // TODO figure out how to measure the sizeof these metrics

        // let map = self.dual_label_map.lock().unwrap();
        // let keys: Vec<String> = map.keys().collect();
        // let categories = map.values().for_each(|category| -> String  { category.keys() });

        // // Copy of `MallocShallowSizeOf` implementation for `HashMap<K, V>` in `wr_malloc_size_of`.
        // // Note: An instantiated submetric is behind an `Arc`.
        // // `size_of` should only be called from a single thread to avoid double-counting.
        // let shallow_size = if ops.has_malloc_enclosing_size_of() {
        //     map.values()
        //         .next()
        //         .map_or(0, |v| unsafe { ops.malloc_enclosing_size_of(v) })
        // } else {
        //     map.capacity()
        //         * (mem::size_of::<String>() + mem::size_of::<T>() + mem::size_of::<usize>())
        // };

        // let mut map_size = shallow_size;
        // for (k, v) in map.iter() {
        //     map_size += k.size_of(ops);
        //     map_size += v.size_of(ops);
        // }

        // self.labels.size_of(ops) + self.submetric.size_of(ops) + map_size
        0
    }
}

impl DualLabeledCounterMetric {
    /// Creates a new dual labeled counter from the given metric instance and optional list of labels.
    pub fn new(
        meta: CommonMetricData,
        keys: Option<Vec<Cow<'static, str>>>,
        catgories: Option<Vec<Cow<'static, str>>>,
    ) -> DualLabeledCounterMetric {
        let submetric = CounterMetric::new(meta);
        DualLabeledCounterMetric::new_inner(submetric, keys, catgories)
    }

    fn new_inner(
        counter: CounterMetric,
        keys: Option<Vec<Cow<'static, str>>>,
        categories: Option<Vec<Cow<'static, str>>>,
    ) -> DualLabeledCounterMetric {
        let dual_label_map = Default::default();
        DualLabeledCounterMetric {
            keys,
            categories,
            counter,
            dual_label_map,
        }
    }

    /// Creates a new metric with a specific label.
    ///
    /// This is used for dynamic labels where we have to actually validate and correct the
    /// label later when we have a Glean object.
    fn new_counter_metric(&self, key: &str, category: &str) -> CounterMetric {
        // Determine which type of dynamic label it is
        if self.keys.is_none() && self.categories.is_none() {
            self.counter
                .with_dynamic_label(DynamicLabelType::KeyAndCategory(
                    make_label_from_key_and_category(key, category),
                ))
        } else if self.keys.is_none() {
            let static_category = self.static_category(category);
            self.counter.with_dynamic_label(DynamicLabelType::KeyOnly(
                make_label_from_key_and_category(key, static_category),
            ))
        } else if self.categories.is_none() {
            let static_key = self.static_key(key);
            self.counter
                .with_dynamic_label(DynamicLabelType::CategoryOnly(
                    make_label_from_key_and_category(static_key, category),
                ))
        } else {
            // Both labels are static and can be validated now
            let static_key = self.static_key(key);
            let static_category = self.static_category(category);
            let name = combine_base_identifier_and_labels(
                self.counter.meta().base_identifier().as_str(),
                static_key,
                static_category,
            );
            self.counter.with_name(name)
        }
    }

    /// Creates a static label for the key dimension.
    ///
    /// # Safety
    ///
    /// Should only be called when static labels are available on this metric.
    ///
    /// # Arguments
    ///
    /// * `key` - The requested key
    ///
    /// # Returns
    ///
    /// The requested key if it is in the list of allowed labels.
    /// Otherwise `OTHER_LABEL` is returned.
    fn static_key<'a>(&self, key: &'a str) -> &'a str {
        debug_assert!(self.keys.is_some());
        let keys = self.keys.as_ref().unwrap();
        if keys.iter().any(|l| l == key) {
            key
        } else {
            OTHER_LABEL
        }
    }

    /// Creates a static label for the category dimension.
    ///
    /// # Safety
    ///
    /// Should only be called when static labels are available on this metric.
    ///
    /// # Arguments
    ///
    /// * `category` - The requested category
    ///
    /// # Returns
    ///
    /// The requested category if it is in the list of allowed labels.
    /// Otherwise `OTHER_LABEL` is returned.
    fn static_category<'a>(&self, category: &'a str) -> &'a str {
        debug_assert!(self.categories.is_some());
        let categories = self.categories.as_ref().unwrap();
        if categories.iter().any(|l| l == category) {
            category
        } else {
            OTHER_LABEL
        }
    }

    /// Gets a specific metric for a given key/category combination.
    ///
    /// If a set of acceptable labels were specified in the `metrics.yaml` file,
    /// and the given label is not in the set, it will be recorded under the special `OTHER_LABEL` label.
    ///
    /// If a set of acceptable labels was not specified in the `metrics.yaml` file,
    /// only the first 16 unique labels will be used.
    /// After that, any additional labels will be recorded under the special `OTHER_LABEL` label.
    ///
    /// Labels must have a maximum of 111 characters, and may comprise any printable ASCII characters.
    /// If an invalid label is used, the metric will be recorded in the special `OTHER_LABEL` label.
    pub fn get<S: AsRef<str>>(&self, key: S, category: S) -> Arc<CounterMetric> {
        let key = key.as_ref();
        let category = category.as_ref();

        let mut map = self.dual_label_map.lock().unwrap();
        map.entry((key.to_string(), category.to_string()))
            .or_insert_with(|| {
                let metric = self.new_counter_metric(key, category);
                Arc::new(metric)
            })
            .clone()
    }

    /// **Exported for test purposes.**
    ///
    /// Gets the number of recorded errors for the given metric and error type.
    ///
    /// # Arguments
    ///
    /// * `error` - The type of error
    ///
    /// # Returns
    ///
    /// The number of errors reported.
    pub fn test_get_num_recorded_errors(&self, error: ErrorType) -> i32 {
        crate::block_on_dispatcher();
        crate::core::with_glean(|glean| {
            test_get_num_recorded_errors(glean, self.counter.meta(), error).unwrap_or(0)
        })
    }
}

/// Combines a metric's base identifier and label
pub fn combine_base_identifier_and_labels(
    base_identifer: &str,
    key: &str,
    category: &str,
) -> String {
    format!(
        "{}{}",
        base_identifer,
        make_label_from_key_and_category(key, category)
    )
}

/// Strips the label off of a complete identifier
pub fn strip_label(identifier: &str) -> &str {
    identifier
        .split_once(char::from(RECORD_SEPARATOR))
        .map_or(identifier, |s| s.0)
}

/// Separate label into key and category components
pub fn separate_label_into_key_and_category(label: &str) -> Option<(&str, &str)> {
    label
        .split_once(char::from(RECORD_SEPARATOR))
        .map(|s| (s.0, s.1))
}

/// Construct and return a label from a given key and category with the RECORD_SEPARATOR
/// characters in the format: <RS><key><RS><category>
pub fn make_label_from_key_and_category(key: &str, category: &str) -> String {
    format!(
        "{}{}{}{}",
        RECORD_SEPARATOR, key, RECORD_SEPARATOR, category
    )
}

/// Validates a dynamic label, changing it to `OTHER_LABEL` if it's invalid.
///
/// Checks the requested label against limitations, such as the label length and allowed
/// characters.
///
/// # Arguments
///
/// * `label` - The requested label
///
/// # Returns
///
/// The entire identifier for the metric, including the base identifier and the corrected label.
/// The errors are logged.
pub fn validate_dynamic_key_and_or_category(
    glean: &Glean,
    meta: &CommonMetricDataInternal,
    base_identifier: &str,
    label: DynamicLabelType,
) -> String {
    // Pick out the key and category from the supplied label
    if let Some((key, category)) = separate_label_into_key_and_category(&label.to_string()) {
        // Now get the full metric identifier for looking up records in storage
        let full_metric_identifier =
            combine_base_identifier_and_labels(base_identifier, key, category);
        // Loop through the stores we expect to find this metric in, and if we
        // find it then just return the full metric identifier that was found
        for store in &meta.inner.send_in_pings {
            if glean.storage().has_metric(meta.inner.lifetime, store, &key) {
                return full_metric_identifier;
            }
        }

        // If the metric wasn't found, we need to determine which of the key and category
        // are dynamic, and count the labels
        let (validated_key, validated_category) = match label {
            DynamicLabelType::Label(label) => {
                let msg = format!("Invalid `DualLabeledCounter` label format: {label:?}");
                record_error(glean, meta, ErrorType::InvalidLabel, msg, None);
                (OTHER_LABEL, OTHER_LABEL)
            }
            DynamicLabelType::KeyOnly(_) => {
                // Count the number of distinct keys already recorded

                let dynamic_key = OTHER_LABEL;
                (dynamic_key, category)
            }
            DynamicLabelType::CategoryOnly(_) => {
                // Count the number of distinct categories already recorded

                let dynamic_category = OTHER_LABEL;
                (key, dynamic_category)
            }
            DynamicLabelType::KeyAndCategory(_) => {
                // Count the number of distinct keys and categories already recorded

                let dynamic_key = OTHER_LABEL;
                let dynamic_category = OTHER_LABEL;
                (dynamic_key, dynamic_category)
            }
        };

        // TODO need to define `label_count` and then do this for keys and or categories in the match statement
        // above.
        let error = if label_count >= MAX_LABELS {
            true
        } else if label.len() > MAX_LABEL_LENGTH {
            let msg = format!(
                "label length {} exceeds maximum of {}",
                label.len(),
                MAX_LABEL_LENGTH
            );
            record_error(glean, meta, ErrorType::InvalidLabel, msg, None);
            true
        } else if label.chars().any(|c| !c.is_ascii() || c.is_ascii_control()) {
            let msg = format!("label must be printable ascii, got '{}'", label);
            record_error(glean, meta, ErrorType::InvalidLabel, msg, None);
            true
        } else {
            false
        };

        if error {
            combine_base_identifier_and_labels(base_identifier, key, category)
        } else {
            full_metric_identifier
        }
    } else {
        record_error(
            glean,
            meta,
            ErrorType::InvalidLabel,
            "Invalid `DualLabeledCounter` label format, unable to determine key and/or category",
            None,
        );
        combine_base_identifier_and_labels(base_identifier, OTHER_LABEL, OTHER_LABEL)
    }
}

fn count_keys(meta: &CommonMetricDataInternal, glean: &Glean) -> usize {
    let prefix = &meta.base_identifier();
    let mut seen_keys: Vec<String> = Vec::new();
    let mut snapshotter = |_: &[u8], m: &Metric| {
        let keys: Vec<String> = m.as_json().as_object().unwrap().keys().cloned().collect();
        // TODO loop through the keys, which I'm not sure if they would be just keys or
        // (key, category), keeping track of the ones we see in the Vec

        if let Some(top_key) = keys.first() {
            if !seen_keys.contains(top_key) {
                seen_keys.push(top_key.clone());
            }
        }
    };

    let lifetime = meta.inner.lifetime;
    for store in &meta.inner.send_in_pings {
        glean
            .storage()
            .iter_store_from(lifetime, store, Some(prefix), &mut snapshotter);
    }
    seen_keys.len()
}

// TODO do I need this or can I reuse the count_keys fn when it is working?
// fn count_categories(storage: &Database) -> usize {
//     let mut label_count = 0;
//     let prefix = &full_metric_identifier[..=base_identifier.len()];
//     let mut snapshotter = |_: &[u8], _: &Metric| {
//         label_count += 1;
//     };

//     let lifetime = meta.inner.lifetime;
//     for store in &meta.inner.send_in_pings {
//         glean
//             .storage()
//             .iter_store_from(lifetime, store, Some(prefix), &mut snapshotter);
//     }
// }
