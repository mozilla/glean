// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::num::NonZeroU64;
use std::path::Path;
use std::str;
use std::time::Duration;

use malloc_size_of::MallocSizeOf;
use rusqlite::params;
use rusqlite::types::FromSqlError;
use rusqlite::OptionalExtension;
use rusqlite::Transaction;
use rusqlite::{Error as SqlError, ErrorCode};

use connection::Connection;
use schema::Schema;
pub use schema::SchemaError;

use crate::common_metric_data::CommonMetricDataInternal;
use crate::database::migration;
use crate::metrics::dual_labeled_counter::RECORD_SEPARATOR;
use crate::metrics::Metric;
use crate::Error;
use crate::Glean;
use crate::Lifetime;
use crate::Result;

mod connection;
mod schema;

#[derive(Debug)]
pub enum LoadState {
    Ok,
    Err(Error),
}

pub struct Database {
    /// The database connection.
    pub(crate) conn: connection::Connection,

    /// Initial file size when opening the database.
    pub(crate) file_size: Option<NonZeroU64>,

    /// Load state
    load_state: LoadState,
}

impl MallocSizeOf for Database {
    fn size_of(&self, _ops: &mut malloc_size_of::MallocSizeOfOps) -> usize {
        // FIXME: Can we get the allocated size of the connection?
        0
    }
}

impl std::fmt::Debug for Database {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Database")
            .field("conn", &self.conn)
            .finish()
    }
}

const DEFAULT_DATABASE_FILE_NAME: &str = "glean.sqlite";

/// Calculate the database size from all the files in the directory.
///
///  # Arguments
///
///  *`path` - The path to the directory
///
///  # Returns
///
/// Returns the non-zero combined size of all files in a directory,
/// or `None` on error or if the size is `0`.
fn database_size(dir: &Path) -> Option<NonZeroU64> {
    let mut total_size = 0;
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    let path = entry.path();
                    if let Ok(metadata) = fs::metadata(path) {
                        total_size += metadata.len();
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    NonZeroU64::new(total_size)
}

pub fn sqlite_open(path: &Path) -> std::result::Result<(Connection, LoadState), Error> {
    // TODO: Make this more robust, use the correct errors and see how we can test all the branches
    // properly.
    match Connection::new::<Schema>(path) {
        Err(e @ SchemaError::UnsupportedSchemaVersion(_)) => Err(e.into()),
        Err(e @ SchemaError::Sqlite(SqlError::SqliteFailure(err, _))) => {
            match err.code {
                ErrorCode::PermissionDenied => Err(e.into()),
                ErrorCode::NotADatabase => {
                    log::debug!("sqlite failed: not a database. starting from scratch.");
                    fs::remove_file(path).map_err(|_| rkv::StoreError::FileInvalid)?;
                    // Now try again, we only handle that error once.
                    let conn = Connection::new::<Schema>(path)?;
                    Ok((conn, LoadState::Err(e.into())))
                }
                ErrorCode::CannotOpen => {
                    log::debug!("sqlite failed: cannot open. starting from scratch.");
                    fs::remove_file(path).map_err(|_| rkv::StoreError::FileInvalid)?;
                    // Now try again, we only handle that error once.
                    let conn = Connection::new::<Schema>(path)?;
                    Ok((conn, LoadState::Err(e.into())))
                }
                _ => Err(e.into()),
            }
        }
        Err(err @ SchemaError::Sqlite(SqlError::SqlInputError { .. })) => {
            log::debug!("sqlite failed: schema migration failed. starting from scratch.");
            fs::remove_file(path).map_err(|_| rkv::StoreError::FileInvalid)?;
            // Now try again, we only handle that error once.
            let conn = Connection::new::<Schema>(path)?;
            Ok((conn, LoadState::Err(err.into())))
        }
        other => {
            let conn = other?;
            Ok((conn, LoadState::Ok))
        }
    }
}

impl Database {
    /// Initializes the data store.
    ///
    /// This opens the underlying SQLite store and creates
    /// the underlying directory structure.
    pub fn new(
        data_path: &Path,
        _delay_ping_lifetime_io: bool,
        _ping_lifetime_threshold: usize,
        _ping_lifetime_max_time: Duration,
    ) -> Result<Self> {
        let path = data_path.join("db");
        log::debug!("Database path: {:?}", path.display());
        let file_size = database_size(&path);

        fs::create_dir_all(&path)?;
        let store_path = path.join(DEFAULT_DATABASE_FILE_NAME);
        let sqlite_exists = store_path.exists();
        let (conn, load_state) = sqlite_open(&store_path)?;

        let db = Self {
            conn,
            file_size,
            load_state,
        };

        if sqlite_exists {
            log::debug!("SQLite database already exists. Not trying to migrate Rkv");
        } else {
            _ = migration::try_migrate(&path, &db);
        }

        Ok(db)
    }

    /// Get the initial database file size.
    pub fn file_size(&self) -> Option<NonZeroU64> {
        self.file_size
    }

    /// Get the load state.
    pub fn load_state(&self) -> Option<String> {
        if let LoadState::Err(e) = &self.load_state {
            Some(e.to_string())
        } else {
            None
        }
    }

    /// Iterates with the provided transaction function
    /// over the requested data from the given storage.
    ///
    /// * If the storage is unavailable, the transaction function is never invoked.
    /// * If the read data cannot be deserialized it will be silently skipped.
    ///
    /// # Arguments
    ///
    /// * `lifetime` - The metric lifetime to iterate over.
    /// * `storage_name` - The storage name to iterate over.
    /// * `transaction_fn` - Called for each entry being iterated over. It is
    ///   passed two arguments: `(metric_id: &[u8], metric: &Metric)`.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn iter_store<F>(&self, lifetime: Lifetime, storage_name: &str, mut transaction_fn: F)
    where
        F: FnMut(&[u8], &[&str], &Metric),
    {
        let iter_sql = r#"
        SELECT
            id,
            value,
            labels
        FROM telemetry
        WHERE
            lifetime = ?1
            AND ping = ?2
        "#;

        self.conn
            .read(|conn| {
                let mut stmt = conn.prepare_cached(iter_sql).unwrap();
                let rows = stmt
                    .query_map(
                        params![lifetime.as_str().to_string(), storage_name],
                        |row| {
                            let id: String = row.get(0)?;
                            let blob: Vec<u8> = row.get(1)?;
                            let labels: String = row.get(2)?;
                            let blob: Metric = rmp_serde::from_slice(&blob)
                                .map_err(|_| FromSqlError::InvalidType)?;
                            Ok((id, labels, blob))
                        },
                    )
                    .unwrap();

                for row in rows {
                    let Ok((metric_id, labels, metric)) = row else {
                        continue;
                    };
                    let labels = labels.split(RECORD_SEPARATOR).collect::<Vec<_>>();
                    transaction_fn(metric_id.as_bytes(), &labels, &metric);
                }

                Result::<(), ()>::Ok(())
            })
            .unwrap()
    }

    /// TODO
    pub fn get_metric(
        &self,
        data: &CommonMetricDataInternal,
        storage_name: &str,
    ) -> Option<Metric> {
        let get_metric_sql = r#"
        SELECT
            value
        FROM telemetry
        WHERE
            id = ?1
            AND ping = ?2
            AND labels = ?3
        LIMIT 1
        "#;

        let metric_identifier = &data.base_identifier();

        self.conn
            .read(|tx| {
                let labels = data.check_labels(tx);

                let mut stmt = tx.prepare_cached(get_metric_sql)?;
                stmt.query_one([metric_identifier, storage_name, labels.label()], |row| {
                    let blob: Vec<u8> = row.get(0)?;
                    let blob: Metric =
                        rmp_serde::from_slice(&blob).map_err(|_| FromSqlError::InvalidType)?;
                    Ok(blob)
                })
                .optional()
            })
            .unwrap_or(None) // TODO: Should we handle the error here properly?
    }

    /// Determines if the storage has the given metric.
    ///
    /// If data cannot be read it is assumed that the storage does not have the metric.
    ///
    /// # Arguments
    ///
    /// * `lifetime` - The lifetime of the metric.
    /// * `storage_name` - The storage name to look in.
    /// * `metric_identifier` - The metric identifier.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn has_metric(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        metric_identifier: &str,
    ) -> bool {
        let has_metric_sql = r#"
        SELECT id
        FROM telemetry
        WHERE
            lifetime = ?1
            AND ping = ?2
            AND id = ?3
        "#;

        self.conn
            .read(|conn| {
                let Ok(mut stmt) = conn.prepare_cached(has_metric_sql) else {
                    return Ok(false);
                };
                let Ok(mut metric_iter) =
                    stmt.query([lifetime.as_str(), storage_name, metric_identifier])
                else {
                    return Ok(false);
                };

                Result::<bool, ()>::Ok(metric_iter.next().map(|m| m.is_some()).unwrap_or(false))
            })
            .unwrap_or(false)
    }

    /// Records a metric in the underlying storage system.
    pub fn record(&self, glean: &Glean, data: &CommonMetricDataInternal, value: &Metric) {
        let name = data.base_identifier();

        _ = self.conn.write(|tx| {
            let labels = data.check_labels(tx);
            labels.record_error(glean, tx, &name, data.storage_names());

            for ping_name in data.storage_names() {
                if glean.is_ping_enabled(ping_name) {
                    if let Err(e) = self.record_per_lifetime(
                        tx,
                        data.inner.lifetime,
                        ping_name,
                        &name,
                        labels.label(),
                        value,
                    ) {
                        log::error!(
                            "Failed to record metric '{}' into {}: {:?}",
                            data.base_identifier(),
                            ping_name,
                            e
                        );
                    }
                }
            }

            Ok::<(), rusqlite::Error>(())
        });
    }

    /// Records a metric in the underlying storage system, for a single lifetime.
    ///
    /// # Returns
    ///
    /// If the storage is unavailable or the write fails, no data will be stored and an error will be returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    pub(crate) fn record_per_lifetime(
        &self,
        tx: &mut Transaction,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        labels: &str,
        metric: &Metric,
    ) -> Result<()> {
        let insert_sql = r#"
        INSERT INTO
            telemetry (id, ping, lifetime, labels, value)
        VALUES
            (?1, ?2, ?3, ?4,  ?5)
        ON CONFLICT(id, ping, labels) DO UPDATE SET
            lifetime = excluded.lifetime,
            value = excluded.value
        "#;

        let mut stmt = tx.prepare_cached(insert_sql)?;
        let encoded = rmp_serde::to_vec(&metric).expect("IMPOSSIBLE: Serializing metric failed");
        stmt.execute(params![
            key,
            storage_name,
            lifetime.as_str(),
            labels,
            encoded
        ])?;

        Ok(())
    }

    /// Records the provided value, with the given lifetime,
    /// after applying a transformation function.
    pub fn record_with<F>(&self, glean: &Glean, data: &CommonMetricDataInternal, transform: F)
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        _ = self
            .conn
            .write(|tx| self.record_with_transaction(glean, tx, data, transform));
    }

    pub fn record_with_transaction<F>(
        &self,
        glean: &Glean,
        tx: &mut Transaction,
        data: &CommonMetricDataInternal,
        mut transform: F,
    ) -> Result<()>
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let name = data.base_identifier();

        let labels = data.check_labels(tx);
        labels.record_error(glean, tx, &name, data.storage_names());

        for ping_name in data.storage_names() {
            if glean.is_ping_enabled(ping_name) {
                if let Err(e) = self.record_per_lifetime_with(
                    tx,
                    data.inner.lifetime,
                    ping_name,
                    &name,
                    labels.label(),
                    &mut transform,
                ) {
                    log::error!(
                        "Failed to record metric '{}' into {}: {:?}",
                        data.base_identifier(),
                        ping_name,
                        e
                    );
                }
            }
        }

        Ok(())
    }

    /// Records a metric in the underlying storage system,
    /// after applying the given transformation function, for a single lifetime.
    ///
    /// # Returns
    ///
    /// If the storage is unavailable or the write fails, no data will be stored and an error will be returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    fn record_per_lifetime_with<F>(
        &self,
        tx: &mut Transaction,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        labels: &str,
        mut transform: F,
    ) -> Result<()>
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let value_sql = r#"
        SELECT value
        FROM telemetry
        WHERE
            id = ?1
            AND ping = ?2
            AND lifetime = ?3
            AND labels = ?4
        LIMIT 1
        "#;

        let new_value = {
            let mut stmt = tx.prepare_cached(value_sql)?;
            let mut rows = stmt.query(params![
                key,
                storage_name,
                lifetime.as_str().to_string(),
                labels
            ])?;

            if let Ok(Some(row)) = rows.next() {
                let blob: Vec<u8> = row.get(0)?;
                let old_value = rmp_serde::from_slice(&blob).ok();
                transform(old_value)
            } else {
                transform(None)
            }
        };

        let insert_sql = r#"
                    INSERT INTO
                        telemetry (id, ping, lifetime, labels, value)
                    VALUES
                        (?1, ?2, ?3, ?4, ?5)
                    ON CONFLICT(id, ping, labels) DO UPDATE SET
                        lifetime = excluded.lifetime,
                        value = excluded.value
                    "#;

        {
            let mut stmt = tx.prepare_cached(insert_sql)?;
            let encoded =
                rmp_serde::to_vec(&new_value).expect("IMPOSSIBLE: Serializing metric failed");
            stmt.execute(params![
                key,
                storage_name,
                lifetime.as_str(),
                labels,
                encoded
            ])?;
        }

        Ok(())
    }

    /// Clears a storage (only Ping Lifetime).
    ///
    /// # Returns
    ///
    /// * If the storage is unavailable an error is returned.
    /// * If any individual delete fails, an error is returned, but other deletions might have
    ///   happened.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn clear_ping_lifetime_storage(&self, storage_name: &str) -> Result<()> {
        let clear_sql = "DELETE FROM telemetry WHERE lifetime = ?1 AND ping = ?2";
        self.conn.write(|tx| {
            let mut stmt = tx.prepare_cached(clear_sql)?;
            stmt.execute([Lifetime::Ping.as_str(), storage_name])?;
            Ok(())
        })
    }

    pub fn clear_lifetime_storage(&self, lifetime: Lifetime, storage_name: &str) -> Result<()> {
        let clear_sql = "DELETE FROM telemetry WHERE lifetime = ?1 AND ping = ?2";
        self.conn.write(|tx| {
            let mut stmt = tx.prepare_cached(clear_sql)?;
            stmt.execute([lifetime.as_str(), storage_name])?;
            Ok(())
        })
    }

    /// Removes a single metric from the storage.
    ///
    /// # Arguments
    ///
    /// * `lifetime` - the lifetime of the storage in which to look for the metric.
    /// * `storage_name` - the name of the storage to store/fetch data from.
    /// * `metric_id` - the metric category + name.
    ///
    /// # Returns
    ///
    /// * If the storage is unavailable an error is returned.
    /// * If the metric could not be deleted, an error is returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// # Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn remove_single_metric(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        metric_id: &str,
    ) -> Result<()> {
        let clear_sql = "DELETE FROM telemetry WHERE lifetime = ?1 AND ping = ?2 AND id = ?3";
        self.conn.write(|tx| {
            let mut stmt = tx.prepare_cached(clear_sql)?;
            stmt.execute([lifetime.as_str(), storage_name, metric_id])?;
            Ok(())
        })
    }

    /// Clears all the metrics in the database, for the provided lifetime.
    ///
    /// Errors are logged.
    ///
    /// # Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn clear_lifetime(&self, lifetime: Lifetime) {
        let clear_sql = "DELETE FROM telemetry WHERE lifetime = ?1";
        _ = self.conn.write(|tx| {
            let mut stmt = tx.prepare_cached(clear_sql)?;
            let res = stmt.execute([lifetime.as_str()]);

            if let Err(e) = res {
                log::warn!("Could not clear store for lifetime {:?}: {:?}", lifetime, e);
            }
            Ok::<(), rusqlite::Error>(())
        });
    }

    /// Clears all metrics in the database.
    ///
    /// Errors are logged.
    ///
    /// # Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn clear_all(&self) {
        let lifetimes = &[
            Lifetime::User.as_str(),
            Lifetime::Ping.as_str(),
            Lifetime::Application.as_str(),
        ];
        let clear_sql =
            "DELETE FROM telemetry WHERE lifetime = ?1 OR lifetime = ?2 OR lifetime = ?3";
        _ = self.conn.write(|tx| {
            let mut stmt = tx.prepare_cached(clear_sql)?;
            let res = stmt.execute(lifetimes);

            if let Err(e) = res {
                log::warn!("Could not clear store for all lifetimes: {:?}", e);
            }
            Ok::<(), rusqlite::Error>(())
        });
    }

    /// Persists ping_lifetime_data to disk.
    ///
    /// Does nothing in case there is nothing to persist.
    ///
    /// # Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn persist_ping_lifetime_data(&self) -> Result<()> {
        Ok(())
    }
}
