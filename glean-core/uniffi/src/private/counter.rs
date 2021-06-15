// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{Glean, CommonMetricData};

#[derive(Clone, Debug)]
pub struct CounterMetric {
    meta: CommonMetricData,
}

impl CounterMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    /// Internal only, synchronous API for incremeting a counter
    pub(crate) fn add_sync(&self, _glean: &Glean, amount: i32) {
        log::info!("Counter({:?}).add({})", self.meta, amount)
    }

    pub fn add(&self, amount: i32) {
        crate::core::with_glean(|glean| self.add_sync(glean, amount))
    }
}
