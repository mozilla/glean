// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The SQLite database schema.

use std::num::NonZeroU32;

use rusqlite::{config::DbConfig, OptionalExtension, Transaction};

use super::connection::ConnectionOpener;

/// The schema for a physical SQLite database that contains many
/// named logical databases.
#[derive(Debug)]
pub struct Schema;

impl ConnectionOpener for Schema {
    const MAX_SCHEMA_VERSION: u32 = 2;

    type Error = SchemaError;

    fn setup(conn: &mut rusqlite::Connection) -> Result<(), Self::Error> {
        conn.execute_batch(
            "
             -- we unconditionally want write-ahead-logging mode
             PRAGMA journal_mode = WAL;
             -- Sync at the most criticial moments, but not with every write
             PRAGMA synchronous = NORMAL;
             -- limit size of the journal. TODO(bug 2049290): value currently arbitrary.
             -- needs refinement.
             PRAGMA journal_size_limit = 512000; -- 512 KB.
             -- We don't care about temp tables being persisted to disk
             PRAGMA temp_store = MEMORY;
             -- allows adding incremental cleanup later
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
            "
             CREATE TABLE telemetry(
               id TEXT NOT NULL,
               ping TEXT NOT NULL,
               lifetime TEXT NOT NULL,
               labels TEXT NOT NULL, -- can't be null or ON CONFLICT won't work
               value BLOB,
               UNIQUE(id, ping, labels)
             );
             CREATE TABLE migration(id INTEGER PRIMARY KEY, state TEXT NOT NULL);
            ",
        )?;
        Ok(())
    }

    fn upgrade(tx: &mut Transaction<'_>, to_version: NonZeroU32) -> Result<(), Self::Error> {
        match to_version.get() {
            2 => {
                log::info!("Upgrading user_version to 2");
                // Clients upgrading to schema 2 don't have the table.
                // But they did run through the migration.
                tx.execute_batch(
                    "CREATE TABLE migration(
                        id INTEGER PRIMARY KEY,
                        state TEXT NOT NULL
                    );",
                )?;
                let cid_exists: Option<i32> = tx
                    .query_row(
                        "SELECT 1 FROM telemetry WHERE id = 'client_id'",
                        [],
                        |row| row.get(0),
                    )
                    .optional()?;
                if cid_exists.is_some() {
                    log::info!("Client ID already exists. Marking migration as done.");
                    tx.execute("INSERT INTO migration (id, state) VALUES (1, 'done') ON CONFLICT(id) DO UPDATE SET state = excluded.state", [])?;
                }
                Ok(())
            }
            to_version => Err(SchemaError::UnsupportedSchemaVersion(to_version)),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SchemaError {
    #[error("unsupported schema version: {0}")]
    UnsupportedSchemaVersion(u32),
    #[error("sqlite: {0}")]
    Sqlite(#[from] rusqlite::Error),
}
