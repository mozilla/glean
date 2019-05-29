// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use crate::metrics::{Metric, MetricType};
use crate::Glean;
use crate::Lifetime;

const MAX_LABELS: usize = 16;
const OTHER_LABEL: &str = "__other__";
const MAX_LABEL_LENGTH: usize = 30;
lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new("^[a-z_][a-z0-9_]{0,29}$").unwrap();
}

/// A labeled metric.
///
/// Labeled metrics allow to record multiple sub-metrics of the same type under different string labels.
#[derive(Clone, Debug)]
pub struct LabeledMetric<T> {
    labels: Option<Vec<String>>,
    /// Type of the underlying metric
    /// We hold on to an instance of it, which is cloned to create new modified instances.
    submetric: T,

    seen_labels: HashSet<String>,
}

impl<T> LabeledMetric<T>
where
    T: MetricType + Clone,
{
    /// Create a new labeled metric from the given meta data and optional list of labels.
    ///
    /// See [`get`](#method.get) for information how static or dynamic labels are handled.
    pub fn new(submetric: T, labels: Option<Vec<String>>) -> LabeledMetric<T> {
        LabeledMetric {
            labels,
            submetric,
            seen_labels: HashSet::new(),
        }
    }

    fn new_metric_with_name(&self, name: String) -> T {
        let mut t = self.submetric.clone();
        t.meta_mut().name = name;
        t
    }

    fn static_label<'a>(&mut self, label: &'a str) -> &'a str {
        let labels = self.labels.as_ref().unwrap();
        if labels.iter().any(|l| l == label) {
            label
        } else {
            OTHER_LABEL
        }
    }

    fn dynamic_label<'a>(&mut self, glean: &Glean, label: &'a str) -> &'a str {
        if self.seen_labels.is_empty() && self.submetric.meta().lifetime != Lifetime::Application {
            // Fetch all labels that are already stored by iterating through existing data.

            let prefix = format!("{}/", self.submetric.meta().identifier());
            let seen_labels = &mut self.seen_labels;
            let mut snapshotter = |metric_name: &[u8], _: &Metric| {
                let metric_name = String::from_utf8_lossy(metric_name);
                if metric_name.starts_with(&prefix) {
                    let label = metric_name.splitn(2, '/').nth(1).unwrap(); // safe unwrap, we know it contains a slash
                    seen_labels.insert(label.into());
                }
            };

            let lifetime = self.submetric.meta().lifetime;
            for store in &self.submetric.meta().send_in_pings {
                glean
                    .storage()
                    .iter_store_from(lifetime, store, &mut snapshotter);
            }
        }

        if !self.seen_labels.contains(label) {
            if self.seen_labels.len() >= MAX_LABELS {
                return OTHER_LABEL;
            } else {
                if label.len() > MAX_LABEL_LENGTH {
                    log::error!(
                        "label length {} exceeds maximum of {}",
                        label.len(),
                        MAX_LABEL_LENGTH
                    );
                    return OTHER_LABEL;
                }

                if !LABEL_REGEX.is_match(label) {
                    log::error!("label must be snake_case, got '{}'", label,);
                    return OTHER_LABEL;
                }

                self.seen_labels.insert(label.into());
            }
        }

        label
    }

    /// Get a specific metric for a given label.
    ///
    /// If a set of acceptable labels were specified in the `metrics.yaml` file,
    /// and the given label is not in the set, it will be recorded under the special `OTHER_LABEL` label.
    ///
    /// If a set of acceptable labels was not specified in the `metrics.yaml` file,
    /// only the first 16 unique labels will be used.
    /// After that, any additional labels will be recorded under the special `OTHER_LABEL` label.
    ///
    /// Labels must be `snake_case` and less than 30 characters.
    /// If an invalid label is used, the metric will be recorded in the special `OTHER_LABEL` label.
    pub fn get(&mut self, glean: &Glean, label: &str) -> T {
        let label = match self.labels {
            Some(_) => self.static_label(label),
            None => self.dynamic_label(glean, label),
        };
        let label = format!("{}/{}", self.submetric.meta().name, label);

        self.new_metric_with_name(label)
    }
}
