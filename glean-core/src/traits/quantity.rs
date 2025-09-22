// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{ErrorType, TestGetValue};

/// A description for the [`QuantityMetric`](crate::metrics::QuantityMetric) type.
///
/// When changing this trait, make sure all the operations are
/// implemented in the related type in `../metrics/`.
pub trait Quantity: TestGetValue<Output = i64> {
    /// Sets the value. Must be non-negative.
    ///
    /// # Arguments
    ///
    /// * `value` - The value. Must be non-negative.
    ///
    /// ## Notes
    ///
    /// Logs an error if the `value` is negative.
    fn set(&self, value: i64);

    /// **Exported for test purposes.**
    ///
    /// Gets the number of recorded errors for the given metric and error type.
    ///
    /// # Arguments
    ///
    /// * `error` - The type of error
    ///
    /// # Returns
    ///
    /// The number of errors reported.
    fn test_get_num_recorded_errors(&self, error: ErrorType) -> i32;
}
