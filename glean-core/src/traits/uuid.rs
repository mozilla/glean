// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::{ErrorType, TestGetValue};

/// A description for the [`UuidMetric`](crate::metrics::UuidMetric) type.
///
/// When changing this trait, make sure all the operations are
/// implemented in the related type in `../metrics/`.
pub trait Uuid: TestGetValue<Output = uuid::Uuid> {
    /// Sets to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The [`Uuid`](uuid::Uuid) to set the metric to.
    fn set(&self, value: uuid::Uuid);

    /// Generates a new random [`Uuid`](uuid::Uuid) and set the metric to it.
    fn generate_and_set(&self) -> uuid::Uuid;

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
