// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashSet;
use std::marker::PhantomData;

use crate::metrics::MetricType;
use crate::CommonMetricData;
use crate::Glean;

const MAX_LABELS: usize = 16;
const OTHER_LABEL: &str = "__other__";
const MAX_LABEL_LENGTH: usize = 30;

/// A labeled metric.
///
/// Labeled metrics allow to record multiple sub-metrics of the same type under different string labels.
#[derive(Debug)]
pub struct LabeledMetric<T> {
    meta: CommonMetricData,
    labels: Option<Vec<String>>,
    /// Type of the underlying metric
    /// We don't hold on to any of these, but we need to be able to create them.
    submetric: PhantomData<T>,

    seen_labels: HashSet<String>,
}

impl<T> LabeledMetric<T>
where
    T: MetricType,
{
    /// Create a new labeled metric from the given meta data and optional list of labels.
    ///
    /// See [`get`](#method.get) for information how static or dynamic labels are handled.
    pub fn new(meta: CommonMetricData, labels: Option<Vec<String>>) -> LabeledMetric<T> {
        LabeledMetric {
            meta,
            labels,
            submetric: PhantomData,
            seen_labels: HashSet::new(),
        }
    }

    fn new_metric_with_name(&self, name: String) -> T {
        let meta = CommonMetricData {
            name,
            ..self.meta.clone()
        };

        T::with_meta(meta)
    }

    fn static_label<'a>(&mut self, label: &'a str) -> &'a str {
        let labels = self.labels.as_ref().unwrap();
        if labels.iter().any(|l| l == label) {
            label
        } else {
            OTHER_LABEL
        }
    }

    fn dynamic_label<'a>(&mut self, label: &'a str) -> &'a str {
        // TODO(bug 1554970): Fetch seen_labels from the database if empty

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

                // TODO: Regex check

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
    pub fn get(&mut self, _glean: &Glean, label: &str) -> T {
        let label = match self.labels {
            Some(_) => self.static_label(label),
            None => self.dynamic_label(label),
        };
        let label = format!("{}/{}", self.meta.name, label);

        self.new_metric_with_name(label)
    }
}
