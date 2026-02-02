// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::borrow::Cow;
use std::char;
use std::collections::{HashMap, HashSet};
use std::mem;
use std::sync::{Arc, Mutex};

use rusqlite::{params, Transaction};

use crate::common_metric_data::{CommonMetricData, CommonMetricDataInternal, DynamicLabelType};
use crate::error_recording::{record_error, test_get_num_recorded_errors, ErrorType};
use crate::metrics::{CounterMetric, Metric, MetricType};
use crate::{Glean, TestGetValue};

const MAX_LABELS: usize = 16;
const OTHER_LABEL: &str = "__other__";
const MAX_LABEL_LENGTH: usize = 111;
pub(crate) const RECORD_SEPARATOR: char = '\x1E';

/// A dual labled metric
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
        let mut n = 0;
        n += self.keys.size_of(ops);
        n += self.categories.size_of(ops);
        n += self.counter.size_of(ops);

        // `MallocSizeOf` is not implemented for `Arc<CounterMetric>`,
        // so we reimplement counting the size of the hashmap ourselves.
        let map = self.dual_label_map.lock().unwrap();

        // Copy of `MallocShallowSizeOf` implementation for `HashMap<K, V>` in `wr_malloc_size_of`.
        // Note: An instantiated submetric is behind an `Arc`.
        // `size_of` should only be called from a single thread to avoid double-counting.
        let shallow_size = if ops.has_malloc_enclosing_size_of() {
            map.values()
                .next()
                .map_or(0, |v| unsafe { ops.malloc_enclosing_size_of(v) })
        } else {
            map.capacity()
                * (mem::size_of::<String>() // key
                    + mem::size_of::<Arc<CounterMetric>>() // allocation for the `Arc` value
                    + mem::size_of::<CounterMetric>() // allocation for the `CounterMetric` value
                                                      // within the `Arc`
                    + mem::size_of::<usize>())
        };

        let mut map_size = shallow_size;
        for (k, v) in map.iter() {
            map_size += k.size_of(ops);
            map_size += v.size_of(ops);
        }
        n += map_size;

        n
    }
}

impl MetricType for DualLabeledCounterMetric {
    fn meta(&self) -> &CommonMetricDataInternal {
        self.counter.meta()
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

    /// Creates a new metric with a specific key and category, validating against
    /// the static or dynamic labels where needed.
    fn new_counter_metric(&self, key: &str, category: &str) -> CounterMetric {
        match (&self.keys, &self.categories) {
            (None, None) => self.counter.with_label(DynamicLabelType::KeyAndCategory(
                key.into(),
                category.into(),
            )),
            (None, _) => {
                let static_category = self.static_category(category);
                self.counter.with_label(DynamicLabelType::KeyOnly(
                    key.into(),
                    static_category.into(),
                ))
            }
            (_, None) => {
                let static_key = self.static_key(key);
                self.counter.with_label(DynamicLabelType::CategoryOnly(
                    static_key.into(),
                    category.into(),
                ))
            }
            (_, _) => {
                // Both labels are static and can be validated now
                let static_key = self.static_key(key);
                let static_category = self.static_category(category);
                let label = format!("{static_key}{RECORD_SEPARATOR}{static_category}");
                self.counter.with_label(DynamicLabelType::Static(label))
            }
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

impl TestGetValue for DualLabeledCounterMetric {
    type Output = HashMap<String, HashMap<String, i32>>;

    fn test_get_value(
        &self,
        ping_name: Option<String>,
    ) -> Option<HashMap<String, HashMap<String, i32>>> {
        let mut out: HashMap<String, HashMap<String, i32>> = HashMap::new();
        let map = self.dual_label_map.lock().unwrap();
        for ((key, category), metric) in map.iter() {
            if let Some(value) = metric.test_get_value(ping_name.clone()) {
                out.entry(key.clone())
                    .or_default()
                    .insert(category.clone(), value);
            }
        }
        Some(out)
    }
}

/// Combines a metric's base identifier and label
pub fn combine_base_identifier_and_labels(
    base_identifier: &str,
    key: &str,
    category: &str,
) -> String {
    format!(
        "{}{}",
        base_identifier,
        make_label_from_key_and_category(key, category)
    )
}

/// Separate label into key and category components.
/// Must validate the label format before calling this to ensure it doesn't contain
/// any ASCII record separator characters aside from the one's we put there.
pub fn separate_label_into_key_and_category(label: &str) -> Option<(&str, &str)> {
    label
        .strip_prefix(RECORD_SEPARATOR)
        .unwrap_or(label)
        .split_once(RECORD_SEPARATOR)
}

/// Construct and return a label from a given key and category with the RECORD_SEPARATOR
/// characters in the format: `<RS><key><RS><category>`
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
    panic!("not validating dual labeled like this anymore");
}

pub fn validate_dual_label_sqlite(
    tx: &mut Transaction,
    base_identifier: &str,
    key: &str,
    category: &str,
    send_in_pings: &[String],
) -> Option<String> {
    let existing_labels_sql = "SELECT DISTINCT labels FROM telemetry WHERE id = ?1";

    let mut existing_keys = HashSet::new();
    let mut existing_categories = HashSet::new();
    'checkdb: loop {
        let Ok(mut stmt) = tx.prepare(existing_labels_sql) else {
            // If we can't fetch from the database, assume the label is ok to use
            break 'checkdb;
        };

        let Ok(mut rows) = stmt.query(params![base_identifier]) else {
            // If we can't fetch from the database, assume the label is ok to use
            break 'checkdb;
        };

        while let Ok(Some(row)) = rows.next() {
            let existing_labels: String = row.get(0).unwrap();
            let Some((existing_key, existing_category)) =
                existing_labels.split_once(RECORD_SEPARATOR)
            else {
                log::debug!("Database contains invalid dual-label: {existing_labels:?}");
                continue;
            };

            existing_keys.insert(existing_key.to_string());
            existing_categories.insert(existing_category.to_string());
        }

        break 'checkdb;
    }

    let new_key = if existing_keys.contains(key) || existing_keys.len() < MAX_LABELS {
        key
    } else {
        OTHER_LABEL
    };

    let new_category =
        if existing_categories.contains(category) || existing_categories.len() < MAX_LABELS {
            category
        } else {
            OTHER_LABEL
        };

    Some(format!("{new_key}{RECORD_SEPARATOR}{new_category}"))
}

fn label_is_valid(label: &str, glean: &Glean, meta: &CommonMetricDataInternal) -> bool {
    if label.len() > MAX_LABEL_LENGTH {
        let msg = format!(
            "label length {} exceeds maximum of {}",
            label.len(),
            MAX_LABEL_LENGTH
        );
        record_error(glean, meta, ErrorType::InvalidLabel, msg, None);
        false
    } else {
        true
    }
}

fn get_seen_keys_and_categories(
    meta: &CommonMetricDataInternal,
    glean: &Glean,
) -> (HashSet<String>, HashSet<String>) {
    let base_identifier = &meta.base_identifier();
    let prefix = format!("{base_identifier}{RECORD_SEPARATOR}");
    let mut seen_keys: HashSet<String> = HashSet::new();
    let mut seen_categories: HashSet<String> = HashSet::new();
    let mut snapshotter = |metric_id: &[u8], _: &Metric| {
        let metric_id_str = String::from_utf8_lossy(metric_id);

        // Split full identifier on the ASCII Record Separator (\x1e)
        let parts: Vec<&str> = metric_id_str.split(RECORD_SEPARATOR).collect();

        if parts.len() == 2 {
            seen_keys.insert(parts[0].into());
            seen_categories.insert(parts[1].into());
        } else {
            record_error(
                glean,
                meta,
                ErrorType::InvalidLabel,
                "Dual Labeled Counter label doesn't contain exactly 2 parts".to_string(),
                None,
            );
        }
    };

    let lifetime = meta.inner.lifetime;
    for store in &meta.inner.send_in_pings {
        glean
            .storage()
            .iter_store_from(lifetime, store, &prefix, &mut snapshotter);
    }

    (seen_keys, seen_categories)
}
