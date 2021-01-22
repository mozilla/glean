// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use glean_core::{ErrorType, Glean};
use inherent::inherent;

use crate::{dispatcher, new_metric};

/// Counter metric wrapper around the FFI implementation
#[derive(Clone)]
pub struct CounterMetric(u64);

impl CounterMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: glean_core::CommonMetricData) -> Self {
        let metric = new_metric!(glean_new_counter_metric, meta);
        Self(metric)
    }

    /// Internal only, synchronous API for incremeting a counter
    pub(crate) fn add_sync(&self, _glean: &Glean, amount: i32) {
        let id = self.0;
        crate::sys::with_glean(|glean| unsafe { glean.glean_counter_add(id, amount) });
    }
}

#[inherent(pub)]
impl glean_core::traits::Counter for CounterMetric {
    fn add(&self, amount: i32) {
        let id = self.0;
        dispatcher::launch(move || {
            crate::sys::with_glean(|glean| unsafe { glean.glean_counter_add(id, amount) });
        });
    }

    fn test_get_value<'a, S: Into<Option<&'a str>>>(&self, _ping_name: S) -> Option<i32> {
        dispatcher::block_on_queue();
        None
    }

    fn test_get_num_recorded_errors<'a, S: Into<Option<&'a str>>>(
        &self,
        _error: ErrorType,
        _ping_name: S,
    ) -> i32 {
        dispatcher::block_on_queue();
        0
    }
}
