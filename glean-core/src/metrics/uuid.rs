// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::CommonMetricData;
use crate::Glean;

/// An UUID metric.
///
/// Stores UUID v4 (randomly generated) values.
#[derive(Clone, Debug)]
pub struct UuidMetric {
    meta: CommonMetricData,
}

impl MetricType for UuidMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl UuidMetric {
    /// Create a new UUID metric
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    /// Set to the specified value.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    /// * `value` - The UUID to set the metric to.
    pub fn set(&self, glean: &Glean, value: uuid::Uuid) {
        if !self.should_record(glean) {
            return;
        }

        let s = value.to_string();
        let value = Metric::Uuid(s);
        glean.storage().record(&self.meta, &value)
    }

    /// Generate a new random UUID and set the metric to it.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    pub fn generate(&self, storage: &Glean) -> uuid::Uuid {
        let uuid = uuid::Uuid::new_v4();
        self.set(storage, uuid);
        uuid
    }

    /// Generate a new random UUID if none is stored yet.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean instance this metric belongs to.
    pub fn generate_if_missing(&self, glean: &Glean) {
        if !self.should_record(glean) {
            return;
        }

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::Uuid(old_value)) => Metric::Uuid(old_value),
                _ => {
                    let uuid = uuid::Uuid::new_v4();
                    let new_value = uuid.to_string();
                    Metric::Uuid(new_value)
                }
            })
    }
}
