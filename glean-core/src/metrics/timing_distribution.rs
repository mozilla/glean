// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::time::Duration;

use serde::Serialize;

use crate::error_recording::{record_error, ErrorType};
use crate::histogram::{Histogram, Type};
use crate::metrics::time_unit::TimeUnit;
use crate::metrics::Metric;
use crate::metrics::MetricType;
use crate::storage::StorageManager;
use crate::CommonMetricData;
use crate::Glean;

/// Identifier for a running timer.
pub type TimerId = u64;

#[derive(Debug, Clone)]
struct Timings {
    next_id: TimerId,
    start_times: HashMap<TimerId, u64>,
}

/// Track different running timers, identified by a `TimerId`.
impl Timings {
    /// Create a new timing manager.
    fn new() -> Self {
        Self {
            next_id: 0,
            start_times: HashMap::new(),
        }
    }

    /// Start a new timer and set it to the `start_time`.
    ///
    /// Returns a new `TimerId` identifying the timer.
    fn set_start(&mut self, start_time: u64) -> TimerId {
        let id = self.next_id;
        self.next_id += 1;
        self.start_times.insert(id, start_time);
        id
    }

    /// Stop the timer and return the elapsed time.
    ///
    /// Returns an error if the `id` does not correspond to a running timer.
    /// Returns an error if the stop time is before the start time.
    ///
    /// ## Note
    ///
    /// This API exists to satisfy the FFI requirements, where the clock is handled on the
    /// application side and passed in as a timestamp.
    fn set_stop(&mut self, id: TimerId, stop_time: u64) -> Result<u64, &str> {
        let start_time = match self.start_times.remove(&id) {
            Some(start_time) => start_time,
            None => return Err("Timing not running"),
        };

        let duration = match stop_time.checked_sub(start_time) {
            Some(duration) => duration,
            None => return Err("Timer stopped with negative duration"),
        };

        Ok(duration)
    }

    /// Cancel and remove the timer.
    fn cancel(&mut self, id: TimerId) {
        self.start_times.remove(&id);
    }
}

/// A timing distribution metric.
///
/// Timing distributions are used to accumulate and store time measurement, for analyzing distributions of the timing data.
#[derive(Debug)]
pub struct TimingDistributionMetric {
    meta: CommonMetricData,
    time_unit: TimeUnit,
    timings: Timings,
}

/// A serializable representation of a snapshotted histogram with a time unit.
#[derive(Debug, Serialize)]
pub struct Snapshot {
    bucket_count: usize,
    range: [u32; 2],
    histogram_type: Type,
    values: HashMap<u32, u32>,
    sum: u32,
    time_unit: TimeUnit,
}

/// Create a snapshot of the histogram with a time unit.
///
/// The snapshot can be serialized into the payload format.
pub(crate) fn snapshot(hist: &Histogram, time_unit: TimeUnit) -> Snapshot {
    let values = hist
        .bucket_ranges()
        .iter()
        .cloned()
        .zip(hist.values().iter().cloned())
        .filter(|(_, v)| *v != 0)
        .collect();

    Snapshot {
        bucket_count: hist.bucket_count(),
        range: [hist.min(), hist.max()],
        histogram_type: hist.typ(),
        values,
        sum: hist.sum(),
        time_unit,
    }
}

impl MetricType for TimingDistributionMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}

impl TimingDistributionMetric {
    /// Create a new timing distribution metric.
    pub fn new(meta: CommonMetricData, time_unit: TimeUnit) -> Self {
        Self {
            meta,
            time_unit,
            timings: Timings::new(),
        }
    }

    /// Start tracking time for the provided metric.
    ///
    /// This records an error if it’s already tracking time (i.e. start was already
    /// called with no corresponding [stop]): in that case the original
    /// start time will be preserved.
    ///
    /// ## Arguments
    ///
    /// * `start_time` - Timestamp in nanoseconds.
    ///
    /// ## Return value
    ///
    /// Returns a unique `TimerId` for the new timer.
    pub fn set_start(&mut self, glean: &Glean, start_time: u64) -> TimerId {
        if !self.should_record(glean) {
            return 0;
        }

        self.timings.set_start(start_time)
    }

    /// Stop tracking time for the provided metric and associated timer id.
    /// Add a count to the corresponding bucket in the timing distribution.
    /// This will record an error if no `start` was called.
    ///
    /// ## Arguments
    ///
    /// * `id` - The `TimerId` to associate with this timing. This allows
    ///   for concurrent timing of events associated with different ids to the
    ///   same timespan metric.
    /// * `stop_time` - Timestamp in nanoseconds.
    pub fn set_stop_and_accumulate(&mut self, glean: &Glean, id: TimerId, stop_time: u64) {
        let duration = match self.timings.set_stop(id, stop_time) {
            Err(error) => {
                record_error(glean, &self.meta, ErrorType::InvalidValue, error, None);
                return;
            }
            Ok(duration) => duration,
        };
        let duration = Duration::from_nanos(duration);
        let duration = self.time_unit.duration_convert(duration);

        self.accumulate_samples(glean, vec![duration as u32])
    }

    /// Abort a previous `set_start` call. No error is recorded if no `set_start`
    /// was called.
    ///
    /// ## Arguments
    ///
    /// * `id` - The `TimerId` to associate with this timing. This allows
    ///   for concurrent timing of events associated with different ids to the
    ///   same timing distribution metric.
    pub fn cancel(&mut self, id: TimerId) {
        self.timings.cancel(id);
    }

    /// Accumulates the provided samples in the metric.
    ///
    /// Please note that this assumes that the provided samples are already in the
    /// "unit" declared by the instance of the implementing metric type (e.g. if the
    /// implementing class is a [TimingDistributionMetricType] and the instance this
    /// method was called on is using [TimeUnit.Second], then `samples` are assumed
    /// to be in that unit).
    ///
    /// ## Arguments
    ///
    /// - `samples` - The vector holding the samples to be recorded by the metric.
    pub fn accumulate_samples(&mut self, glean: &Glean, samples: Vec<u32>) {
        if !self.should_record(glean) {
            return;
        }

        for sample in samples {
            glean
                .storage()
                .record_with(&self.meta, |old_value| match old_value {
                    Some(Metric::TimingDistribution(mut hist, time_unit)) => {
                        hist.accumulate(sample);
                        Metric::TimingDistribution(hist, time_unit)
                    }
                    _ => {
                        let mut hist = Histogram::default();
                        hist.accumulate(sample);
                        Metric::TimingDistribution(hist, self.time_unit)
                    }
                });
        }
    }

    /// Accumulates the provided signed samples in the metric.
    ///
    /// This is required so that the platform-specific code can provide us with
    /// 64 bit signed integers if no `u32` comparable type is available. This
    /// will take care of filtering and reporting errors for any provided negative
    /// sample.
    ///
    /// ## Arguments
    ///
    /// - `samples` - The vector holding the samples to be recorded by the metric.
    ///
    /// ## Notes
    ///
    /// Discards any negative value in `samples` and report an `ErrorType::InvalidValue`
    /// for each of them.
    pub fn accumulate_samples_signed(&mut self, glean: &Glean, samples: Vec<i64>) {
        let sample_count = samples.len();
        let positive_samples: Vec<u32> = samples
            .into_iter()
            .filter(|s| *s >= 0)
            .map(|s| s as u32)
            .collect();
        // Report if we find any negative value.
        let num_errors = sample_count - positive_samples.len();
        let msg = format!("Accumulated ${} negative samples", num_errors);
        record_error(glean, &self.meta, ErrorType::InvalidValue, msg, num_errors);
        // Finally, accumulate the values.
        self.accumulate_samples(glean, positive_samples);
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently stored value as an integer.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value(&self, glean: &Glean, storage_name: &str) -> Option<Histogram> {
        match StorageManager.snapshot_metric(glean.storage(), storage_name, &self.meta.identifier())
        {
            Some(Metric::TimingDistribution(hist, _)) => Some(hist),
            _ => None,
        }
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the currently-stored histogram as a JSON String of the serialized value.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value_as_json_string(
        &self,
        glean: &Glean,
        storage_name: &str,
    ) -> Option<String> {
        self.test_get_value(glean, storage_name)
            .map(|hist| serde_json::to_string(&snapshot(&hist, self.time_unit)).unwrap())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_snapshot() {
        use serde_json::json;

        let mut hist = Histogram::exponential(1, 500, 10);

        for i in 1..=10 {
            hist.accumulate(i);
        }

        let snap = snapshot(&hist, TimeUnit::Millisecond);

        let expected_json = json!({
            "bucket_count": 10,
            "histogram_type": "exponential",
            "range": [1, 500],
            "sum": 55,
            "time_unit": "millisecond",
            "values": {
                "1": 1,
                "2": 2,
                "4": 5,
                "9": 2,
            },
        });

        assert_eq!(expected_json, json!(snap));
    }
}
