// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::Arc;

use crate::error_recording::{record_error, ErrorType};
use crate::private::{Metric, MetricType};
use crate::storage;
use crate::{CommonMetricData, Glean};

#[derive(Debug, Clone)]
pub struct CounterMetric {
    meta: Arc<CommonMetricData>,
}

impl MetricType for CounterMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn with_name(&self, name: String) -> Self {
        let mut meta = (*self.meta).clone();
        meta.name = name;
        Self {
            meta: Arc::new(meta),
        }
    }

    fn with_dynamic_label(&self, label: String) -> Self {
        let mut meta = (*self.meta).clone();
        meta.dynamic_label = Some(label);
        Self {
            meta: Arc::new(meta),
        }
    }
}

impl CounterMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: CommonMetricData) -> Self {
        Self {
            meta: Arc::new(meta),
        }
    }

    /// Internal only, synchronous API for incremeting a counter
    pub(crate) fn add_sync(&self, glean: &Glean, amount: i32) {
        log::info!("Counter({:?}).add({})", self.meta, amount);

        if !self.should_record(glean) {
            return;
        }

        if amount <= 0 {
            record_error(
                glean,
                &self.meta,
                ErrorType::InvalidValue,
                format!("Added negative or zero value {}", amount),
                None,
            );
            return;
        }

        glean
            .storage()
            .record_with(glean, &self.meta, |old_value| match old_value {
                Some(Metric::Counter(old_value)) => {
                    Metric::Counter(old_value.saturating_add(amount))
                }
                _ => Metric::Counter(amount),
            })
    }

    pub fn add(&self, amount: i32) {
        let metric = self.clone();
        crate::launch_with_glean(move |glean| metric.add_sync(glean, amount))
    }

    pub(crate) fn get_value(&self, glean: &Glean, ping_name: Option<&str>) -> Option<i32> {
        let queried_ping_name = ping_name.unwrap_or_else(|| &self.meta().send_in_pings[0]);

        match storage::snapshot_metric_for_test(
            glean.storage(),
            queried_ping_name,
            &self.meta.identifier(glean),
            self.meta.lifetime,
        ) {
            Some(Metric::Counter(i)) => Some(i),
            _ => None,
        }
    }

    pub fn test_get_value(&self, ping_name: Option<String>) -> Option<i32> {
        crate::block_on_dispatcher();
        crate::core::with_glean(|glean| self.get_value(glean, ping_name.as_deref()))
    }
}
