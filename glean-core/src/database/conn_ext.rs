// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use rusqlite::Connection;

/// This trait exists so that we can use these helpers on `rusqlite::{Transaction, Connection}`.
/// Note that you must import ConnExt in order to call these methods on anything.
pub trait ConnExt {
    /// The method you need to implement to opt in to all of this.
    fn conn(&self) -> &Connection;

    /// Execute a single statement.
    fn execute_one(&self, stmt: &str) -> Result<(), rusqlite::Error> {
        match self.conn().execute(stmt, []) {
            Ok(_) => Ok(()),
            // Ignore ExecuteReturnedResults error because they're pointless
            // and annoying.
            Err(rusqlite::Error::ExecuteReturnedResults) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

impl ConnExt for Connection {
    #[inline]
    fn conn(&self) -> &Connection {
        self
    }
}
