// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use inherent::inherent;
use std::sync::Arc;

use glean_core::metrics::MetricType;
pub use glean_core::metrics::{Datetime, TimeUnit};
use glean_core::ErrorType;

// We need to wrap the glean-core type: otherwise if we try to implement
// the trait for the metric in `glean_core::metrics` we hit error[E0117]:
// only traits defined in the current crate can be implemented for arbitrary
// types.

/// Developer-facing API for recording datetime metrics.
///
/// Instances of this class type are automatically generated by the parsers
/// at build time, allowing developers to record values that were previously
/// registered in the metrics.yaml file.
#[derive(Clone)]
pub struct DatetimeMetric(pub(crate) Arc<glean_core::metrics::DatetimeMetric>);

impl DatetimeMetric {
    /// The public constructor used by automatically generated metrics.
    pub fn new(meta: glean_core::CommonMetricData, time_unit: TimeUnit) -> Self {
        Self(Arc::new(glean_core::metrics::DatetimeMetric::new(
            meta, time_unit,
        )))
    }
}

#[inherent(pub)]
impl glean_core::traits::Datetime for DatetimeMetric {
    fn set(&self, value: Option<Datetime>) {
        let metric = Arc::clone(&self.0);
        crate::launch_with_glean(move |glean| metric.set(glean, value));
    }

    fn test_get_value<'a, S: Into<Option<&'a str>>>(&self, ping_name: S) -> Option<Datetime> {
        crate::block_on_dispatcher();

        let queried_ping_name = ping_name
            .into()
            .unwrap_or_else(|| &self.0.meta().send_in_pings[0]);

        crate::with_glean(|glean| self.0.test_get_value(glean, queried_ping_name))
    }

    fn test_get_num_recorded_errors<'a, S: Into<Option<&'a str>>>(
        &self,
        error: ErrorType,
        ping_name: S,
    ) -> i32 {
        crate::block_on_dispatcher();

        crate::with_glean_mut(|glean| {
            glean_core::test_get_num_recorded_errors(glean, self.0.meta(), error, ping_name.into())
                .unwrap_or(0)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::common_test::{lock_test, new_glean};
    use crate::CommonMetricData;
    use chrono::prelude::*;

    #[test]
    fn datetime_convenient_api() {
        let _lock = lock_test();
        let _t = new_glean(None, true);

        let metric: DatetimeMetric = DatetimeMetric::new(
            CommonMetricData {
                name: "datetime".into(),
                category: "test".into(),
                send_in_pings: vec!["test1".into()],
                ..Default::default()
            },
            TimeUnit::Day,
        );

        // Record a date: it will get truncated to Day resolution.
        let sample_date = FixedOffset::east(0).ymd(2018, 2, 25).and_hms(11, 5, 0);
        metric.set(Some(sample_date));

        // Check that the value has the correct resolution.
        let date = metric.test_get_value(None).unwrap();
        assert_eq!(date, FixedOffset::east(0).ymd(2018, 2, 25).and_hms(0, 0, 0));

        // Ensure no error was recorded.
        assert_eq!(
            metric.test_get_num_recorded_errors(ErrorType::InvalidValue, None),
            0
        )
    }
}