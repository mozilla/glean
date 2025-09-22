// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{ErrorType, TestGetValue};

/// A description for the [`CounterMetric`](crate::metrics::CounterMetric) type.
///
/// When changing this trait, make sure all the operations are
/// implemented in the related type in `../metrics/`.
pub trait Counter: TestGetValue<Output = i32> {
    /// Increases the counter by `amount`.
    ///
    /// # Arguments
    ///
    /// * `amount` - The amount to increase by. Should be positive.
    ///
    /// ## Notes
    ///
    /// Logs an error if the `amount` is 0 or negative.
    fn add(&self, amount: i32);

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
