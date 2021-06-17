// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::atomic::{AtomicI32, Ordering};

use crate::error_recording::{record_error, ErrorType};
use crate::private::{Metric, MetricType};
use crate::{CommonMetricData, Glean};

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
    count: AtomicI32,
}

impl MetricType for CounterMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl CounterMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: CommonMetricData) -> Self {
        Self {
            meta,
            count: AtomicI32::new(0),
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
        crate::core::with_glean(|glean| self.add_sync(glean, amount))
    }

    pub(crate) fn get_value(&self, _glean: &Glean, _ping_name: Option<&str>) -> Option<i32> {
        let val = self.count.load(Ordering::SeqCst);
        if val > 0 {
            Some(val)
        } else {
            None
        }
    }

    pub fn test_get_value(&self, ping_name: Option<String>) -> Option<i32> {
        crate::core::with_glean(|glean| self.get_value(glean, ping_name.as_deref()))
    }
}
