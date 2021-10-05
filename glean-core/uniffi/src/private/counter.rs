// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{CommonMetricData, Glean};
use std::sync::atomic::{AtomicI32, Ordering};

#[derive(Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
    count: AtomicI32,
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
    pub(crate) fn add_sync(&self, _glean: &Glean, amount: i32) {
        log::info!("Counter({:?}).add({})", self.meta, amount);
        self.count.fetch_add(amount, Ordering::SeqCst);
    }

    pub fn add(&self, amount: i32) {
        crate::core::with_glean(|glean| self.add_sync(glean, amount))
    }

    pub fn test_get_value(&self, _ping_name: Option<String>) -> Option<i32> {
        let val = self.count.load(Ordering::SeqCst);
        if val > 0 {
            Some(val)
        } else {
            None
        }
    }
}
