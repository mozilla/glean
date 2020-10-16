// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

/// A description for the `UuidMetric` type.
///
/// When changing this trait, make sure all the operations are
/// implemented in the related type in `../metrics/`.
pub trait Uuid {
    /// Sets to the specified value.
    ///
    /// # Arguments
    ///
    /// * `value` - The UUID to set the metric to.
    fn set(&self, value: uuid::Uuid);

    /// Generates a new random UUID and set the metric to it.
    fn generate_and_set(&self) -> uuid::Uuid;

    /// **Exported for test purposes.**
    ///
    /// Gets the currently stored value as a string.
    ///
    /// This doesn't clear the stored value.
    fn test_get_value(&self, storage_name: &str) -> Option<String>;
}
