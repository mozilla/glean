// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::atomic::{AtomicU64, Ordering::SeqCst};
use std::sync::Arc;

use glean_core::{ErrorType, metrics::TimeUnit};
use inherent::inherent;

use crate::{dispatcher, new_metric};

/// Timespan metric wrapper around the FFI implementation
#[derive(Clone)]
pub struct TimespanMetric(Arc<AtomicU64>);

impl TimespanMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: glean_core::CommonMetricData, time_unit: TimeUnit) -> Self {
        let metric = Arc::new(AtomicU64::new(0));

        let deferred = Arc::clone(&metric);
        dispatcher::launch(move || {
            let metric = new_metric!(glean_new_timespan_metric, meta, time_unit as i32);
            deferred.store(metric, SeqCst);
        });

        Self(metric)
    }
}

#[inherent(pub)]
impl glean_core::traits::Timespan for TimespanMetric {
    fn start(&self) {
        let start_time = time::precise_time_ns();

        let id = Arc::clone(&self.0);
        dispatcher::launch(move || {
            let id = id.load(SeqCst);
            crate::sys::with_glean(|glean| unsafe { glean.glean_timespan_set_start(id, start_time) });
        });
    }

    fn stop(&self) {
        let stop_time = time::precise_time_ns();

        let id = Arc::clone(&self.0);
        dispatcher::launch(move || {
            let id = id.load(SeqCst);
            crate::sys::with_glean(|glean| unsafe { glean.glean_timespan_set_stop(id, stop_time) });
        });
    }

    fn cancel(&self) {
        let id = Arc::clone(&self.0);
        dispatcher::launch(move || {
            let id = id.load(SeqCst);
            crate::sys::with_glean(|glean| unsafe { glean.glean_timespan_cancel(id) });
        });
    }

    fn test_get_value<'a, S: Into<Option<&'a str>>>(&self, _ping_name: S) -> Option<u64> {
        crate::block_on_dispatcher();
        None
    }

    fn test_get_num_recorded_errors<'a, S: Into<Option<&'a str>>>(
        &self,
        _error: ErrorType,
        _ping_name: S,
    ) -> i32 {
        crate::block_on_dispatcher();
        0
    }
}
