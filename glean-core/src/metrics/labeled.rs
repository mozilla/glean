// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::marker::PhantomData;

use crate::metrics::MetricType;
use crate::CommonMetricData;
use crate::Glean;

/// A labeled metric.
///
/// Labeled metrics allow to record multiple sub-metrics of the same type under different string labels.
#[derive(Debug)]
pub struct LabeledMetric<T> {
    meta: CommonMetricData,
    labels: Option<Vec<String>>,
    // Type of the underlying metric
    // We don't hold on to any of these, but we need to be able to create them.
    typ: PhantomData<T>,
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
            typ: PhantomData,
        }
    }

    fn new_metric_with_name(&self, name: String) -> T {
        let meta = CommonMetricData {
            name,
            ..self.meta.clone()
        };

        T::with_meta(meta)
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
    pub fn get(&self, _glean: &Glean, label: &str) -> T {
        self.new_metric_with_name(format!("{}/{}", self.meta.name, label))
    }
}
