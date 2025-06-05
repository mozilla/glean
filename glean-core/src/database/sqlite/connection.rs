// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Lower-level, generic SQLite connection management.
//!
//! This module is inspired by, and borrows concepts from, the
//! Application Services `sql-support` crate.

use std::{fmt::Debug, num::NonZeroU32, path::Path, sync::Mutex};

use rusqlite::{OpenFlags, Transaction, TransactionBehavior};

/// Sets up an SQLite database connection, and either
/// initializes an empty physical database with the latest schema, or
/// upgrades an existing physical database to the latest schema.
pub trait ConnectionOpener {
    /// The highest schema version that we support.
    const MAX_SCHEMA_VERSION: u32;

    type Error: From<rusqlite::Error>;

    /// Sets up an opened connection for use. This is a good place to
    /// set pragmas and configuration options, register functions, and
    /// load extensions.
    fn setup(_conn: &mut rusqlite::Connection) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Initializes an empty physical database with the latest schema.
    fn create(tx: &mut Transaction<'_>) -> Result<(), Self::Error>;

    /// Upgrades an existing physical database to the schema with
    /// the given version.
    fn upgrade(tx: &mut Transaction<'_>, to_version: NonZeroU32) -> Result<(), Self::Error>;
}

/// A thread-safe wrapper around a connection to a physical SQLite database.
pub struct Connection {
    /// The inner connection.
    conn: Mutex<rusqlite::Connection>,
}

impl Connection {
    /// Opens a connection to a physical database at the given path.
    pub fn new<O>(path: &Path) -> Result<Self, O::Error>
    where
        O: ConnectionOpener,
    {
        let flags = OpenFlags::SQLITE_OPEN_NO_MUTEX // Send/Sync is guaranteed by Rust already
            | OpenFlags::SQLITE_OPEN_EXRESCODE      // Extended result codes
            | OpenFlags::SQLITE_OPEN_CREATE         // Create if it doesn't exist
            | OpenFlags::SQLITE_OPEN_READ_WRITE; // opened for reading and writing

        let mut conn = rusqlite::Connection::open_with_flags(path, flags)?;
        O::setup(&mut conn)?;

        // On open upgrade the schema to the latest version.
        let mut tx = conn.transaction_with_behavior(TransactionBehavior::Exclusive)?;
        match tx.query_row_and_then("PRAGMA user_version", [], |row| row.get(0)) {
            Ok(mut version @ 1..) => {
                while version < O::MAX_SCHEMA_VERSION {
                    O::upgrade(&mut tx, NonZeroU32::new(version + 1).unwrap())?;
                    version += 1;
                }
            }
            Ok(0) => O::create(&mut tx)?,
            Err(err) => Err(err)?,
        }
        // Set the schema version to the highest that we support.
        // If the current version is higher than ours, downgrade it,
        // so that upgrading to it again in the future can fix up any
        // invariants that our version might not uphold.
        tx.execute_batch(&format!("PRAGMA user_version = {}", O::MAX_SCHEMA_VERSION))?;
        tx.commit()?;
        Ok(Self::with_connection(conn))
    }

    fn with_connection(conn: rusqlite::Connection) -> Self {
        Self {
            conn: Mutex::new(conn),
        }
    }

    /// Accesses the database for reading.
    pub fn read<T, E>(
        &self,
        f: impl FnOnce(&rusqlite::Connection) -> Result<T, E>,
    ) -> Result<T, E> {
        let conn = self.conn.lock().unwrap();
        f(&*conn)
    }

    /// Accesses the database in a transaction for reading and writing.
    pub fn write<T, E>(&self, f: impl FnOnce(&mut Transaction<'_>) -> Result<T, E>) -> Result<T, E>
    where
        E: From<rusqlite::Error>,
    {
        let mut conn = self.conn.lock().unwrap();
        let mut tx = conn.transaction_with_behavior(TransactionBehavior::Immediate)?;
        let result = f(&mut tx)?;
        tx.commit()?;
        Ok(result)
    }
}

impl Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Connection { .. }")
    }
}
