// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[cfg(feature = "sqlite")]
mod conn_ext;

#[cfg(feature = "sqlite")]
pub mod migration;
#[cfg(feature = "sqlite")]
pub mod sqlite;

use crate::{JsonValue, Result};
use chrono::{DateTime, Utc};
#[cfg(feature = "sqlite")]
pub use conn_ext::ConnExt;

#[cfg(not(feature = "sqlite"))]
mod rkv;

#[cfg(feature = "sqlite")]
pub use sqlite::Database;

#[cfg(not(feature = "sqlite"))]
pub use rkv::Database;

/// A trait defining the methods for a database to handle storing submitted pings.
pub trait StoredSubmittedPingHandler {
    /// Gets all pings in the `submitted_pings` table.
    fn get_all_submitted_pings(&self) -> Vec<crate::SubmittedPing>;

    /// Returns all submitted pings in the `submitted_pings` table that match a supplied ping name.
    ///
    /// # Arguments
    ///
    /// * `ping` - The name of the pings to return.
    fn get_submitted_pings_by_name(&self, ping: &str) -> Vec<crate::SubmittedPing>;

    /// Marks a particular ping as uploaded.
    ///
    /// # Arguments
    ///
    /// * `document_id` - The ping to mark as uploaded.
    /// * `date_uploaded` - The UTC date/time the ping was uploaded.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of rows updated.
    fn mark_ping_as_uploaded(&self, document_id: &str, date_uploaded: DateTime<Utc>) -> usize;

    /// Marks a particular ping as upload failed.
    ///
    /// # Arguments
    ///
    /// * `document_id` - The ping to mark as upload failed.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of rows updated.
    fn mark_ping_as_upload_failed(&self, document_id: &str) -> usize;

    /// Stores a submitted ping into the `submitted_pings` table.
    ///
    /// # Arguments
    ///
    /// * `document_id` - The unique identifier for the ping.
    /// * `ping` - The name of the ping.
    /// * `date_submitted` - The UTC date/time the ping was submitted.
    /// * `date_uploaded` - An optional UTC date/time the ping was uploaded.
    /// * `payload` - A JSON representation of the content of the ping.
    ///
    /// # Returns
    ///
    /// An empty `Result`.
    fn store_submitted_ping(
        &self,
        document_id: &str,
        ping: &str,
        date_submitted: DateTime<Utc>,
        date_uploaded: Option<DateTime<Utc>>,
        upload_failed: Option<DateTime<Utc>>,
        payload: JsonValue,
    ) -> Result<()>;

    /// Remove stored submitted pings that are older than `before_time` (or 30 days if not specified)
    ///
    /// # Arguments
    ///
    /// * `before_time` - An optional date – when supplied uses that date as the oldest date_submitted we should keep.
    ///     Defaults to 30 days if `None` is supplied.
    ///
    /// # Returns
    ///
    /// An empty `Result`.
    fn cleanup_submitted_pings(&self, before_time: Option<DateTime<Utc>>) -> Result<()>;
}
