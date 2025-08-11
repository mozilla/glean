// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{DistributionData, ErrorType, TestGetValue};

/// A description for the
/// [`CustomDistributionMetric`](crate::metrics::CustomDistributionMetric) type.
///
/// When changing this trait, make sure all the operations are
/// implemented in the related type in `../metrics/`.
pub trait CustomDistribution: TestGetValue<DistributionData> {
    /// Accumulates the provided signed samples in the metric.
    ///
    /// This is required so that the platform-specific code can provide us with
    /// 64 bit signed integers if no `u64` comparable type is available. This
    /// will take care of filtering and reporting errors for any provided negative
    /// sample.
    ///
    /// # Arguments
    ///
    /// - `samples` - The vector holding the samples to be recorded by the metric.
    ///
    /// ## Notes
    ///
    /// Discards any negative value in `samples` and report an
    /// [`ErrorType::InvalidValue`](crate::ErrorType::InvalidValue) for each of
    /// them.
    fn accumulate_samples_signed(&self, samples: Vec<i64>);

    /// Accumulates precisely one signed sample in the metric.
    ///
    /// This is required so that the platform-specific code can provide us with a
    /// 64 bit signed integer if no `u64` comparable type is available. This
    /// will take care of filtering and reporting errors.
    ///
    /// # Arguments
    ///
    /// - `sample` - The singular sample to be recorded by the metric.
    ///
    /// ## Notes
    ///
    /// Discards any negative value of `sample` and reports an
    /// [`ErrorType::InvalidValue`](crate::ErrorType::InvalidValue).
    fn accumulate_single_sample_signed(&self, sample: i64);

    /// **Exported for test purposes.**
    ///
    /// Gets the number of recorded errors for the given error type.
    ///
    /// # Arguments
    ///
    /// * `error` - The type of error
    ///
    /// # Returns
    ///
    /// The number of errors recorded.
    fn test_get_num_recorded_errors(&self, error: ErrorType) -> i32;
}
