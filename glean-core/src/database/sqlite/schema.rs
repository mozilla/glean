// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The SQLite database schema.

use std::num::NonZeroU32;

use rusqlite::{config::DbConfig, Transaction};

use super::connection::ConnectionOpener;

/// The schema for a physical SQLite database that contains many
/// named logical databases.
#[derive(Debug)]
pub struct Schema;

impl ConnectionOpener for Schema {
    const MAX_SCHEMA_VERSION: u32 = 1;

    type Error = SchemaError;

    fn setup(conn: &mut rusqlite::Connection) -> Result<(), Self::Error> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA journal_size_limit = 512000; -- 512 KB.
             PRAGMA temp_store = MEMORY;
             PRAGMA auto_vacuum = INCREMENTAL;
            ",
        )?;

        // Set hardening flags.
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_DEFENSIVE, true)?;

        // Turn off misfeatures: double-quoted strings and untrusted schemas.
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_DQS_DML, false)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_DQS_DDL, false)?;
        conn.set_db_config(DbConfig::SQLITE_DBCONFIG_TRUSTED_SCHEMA, true)?;

        Ok(())
    }

    fn create(tx: &mut Transaction<'_>) -> Result<(), Self::Error> {
        tx.execute_batch(
            "CREATE TABLE telemetry(
               id TEXT NOT NULL,
               ping TEXT NOT NULL,
               lifetime TEXT NOT NULL,
               labels TEXT NOT NULL, -- can't be null or ON CONFLICT won't work
               value BLOB,
               UNIQUE(id, ping, labels)
             );",
        )?;
        Ok(())
    }

    fn upgrade(_: &mut Transaction<'_>, to_version: NonZeroU32) -> Result<(), Self::Error> {
        Err(SchemaError::UnsupportedSchemaVersion(to_version.get()))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SchemaError {
    #[error("unsupported schema version: {0}")]
    UnsupportedSchemaVersion(u32),
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
