// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::num::NonZeroU64;
use std::path::Path;
use std::str;

use rusqlite::params;
use rusqlite::types::FromSqlError;
use rusqlite::Transaction;

use connection::Connection;
use schema::Schema;

use crate::common_metric_data::CommonMetricDataInternal;
use crate::metrics::Metric;
use crate::Glean;
use crate::Lifetime;
use crate::Result;

mod connection;
mod schema;

pub struct Database {
    /// The database connection.
    conn: connection::Connection,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Database")
            .field("conn", &self.conn)
            .finish()
    }
}

const DEFAULT_DATABASE_FILE_NAME: &str = "glean.sqlite";

impl Database {
    /// Initializes the data store.
    ///
    /// This opens the underlying SQLite store and creates
    /// the underlying directory structure.
    pub fn new(data_path: &Path, _delay_ping_lifetime_io: bool) -> Result<Self> {
        let path = data_path.join("db");
        log::debug!("Database path: {:?}", path.display());

        fs::create_dir_all(&path)?;
        let store_path = path.join(DEFAULT_DATABASE_FILE_NAME);
        let conn = Connection::new::<Schema>(&store_path).unwrap();

        let db = Self { conn };

        Ok(db)
    }

    /// Get the initial database file size.
    pub fn file_size(&self) -> Option<NonZeroU64> {
        None
    }

    /// Get the rkv load state.
    pub fn rkv_load_state(&self) -> Option<String> {
        None
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
        F: FnMut(&[u8], &Metric),
    {
        let iter_sql = r#"
        SELECT id, value
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
                            let blob: Metric = bincode::deserialize(&blob)
                                .map_err(|_| FromSqlError::InvalidType)?;
                            Ok((id, blob))
                        },
                    )
                    .unwrap();

                for row in rows {
                    let Ok((metric_id, metric)) = row else {
                        continue;
                    };
                    transaction_fn(metric_id.as_bytes(), &metric);
                }

                Result::<(), ()>::Ok(())
            })
            .unwrap()
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
        // If upload is disabled we don't want to record.
        if !glean.is_upload_enabled() {
            return;
        }

        let name = data.identifier(glean);

        _ = self.conn.write(|tx| {
            for ping_name in data.storage_names() {
                if let Err(e) =
                    self.record_per_lifetime(tx, data.inner.lifetime, ping_name, &name, value)
                {
                    log::error!(
                        "Failed to record metric '{}' into {}: {:?}",
                        data.base_identifier(),
                        ping_name,
                        e
                    );
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
    fn record_per_lifetime(
        &self,
        tx: &mut Transaction,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        metric: &Metric,
    ) -> Result<()> {
        let insert_sql = r#"
        INSERT INTO
            telemetry (id, ping, lifetime, value)
        VALUES
            (?1, ?2, ?3, ?4)
        ON CONFLICT(id, ping) DO UPDATE SET
            lifetime = excluded.lifetime,
            value = excluded.value
        "#;

        let mut stmt = tx.prepare_cached(insert_sql)?;
        let encoded = bincode::serialize(&metric).expect("IMPOSSIBLE: Serializing metric failed");
        stmt.execute(params![key, storage_name, lifetime.as_str(), encoded])?;

        Ok(())
    }

    /// Records the provided value, with the given lifetime,
    /// after applying a transformation function.
    pub fn record_with<F>(&self, glean: &Glean, data: &CommonMetricDataInternal, mut transform: F)
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        // If upload is disabled we don't want to record.
        if !glean.is_upload_enabled() {
            return;
        }

        _ = self.conn.write(|tx| {
            let name = data.identifier(glean);
            for ping_name in data.storage_names() {
                if let Err(e) = self.record_per_lifetime_with(
                    tx,
                    data.inner.lifetime,
                    ping_name,
                    &name,
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

            Result::<(), rusqlite::Error>::Ok(())
        });
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
        mut transform: F,
    ) -> Result<()>
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let find_sql = r#"
        SELECT value
        FROM telemetry
        WHERE
            lifetime = ?1
            AND ping = ?2
            AND id = ?3
        LIMIT 1
        "#;

        let new_value = {
            let mut stmt = tx.prepare_cached(&find_sql)?;
            let mut rows = stmt.query(params![lifetime.as_str().to_string(), storage_name, key])?;

            if let Ok(Some(row)) = rows.next() {
                let blob: Vec<u8> = row.get(0)?;
                let old_value = bincode::deserialize(&blob).ok();
                transform(old_value)
            } else {
                transform(None)
            }
        };

        let insert_sql = r#"
                    INSERT INTO
                        telemetry (id, ping, lifetime, value)
                    VALUES
                        (?1, ?2, ?3, ?4)
                    ON CONFLICT(id, ping) DO UPDATE SET
                        lifetime = excluded.lifetime,
                        value = excluded.value
                    "#;

        {
            let mut stmt = tx.prepare_cached(insert_sql)?;
            let encoded =
                bincode::serialize(&new_value).expect("IMPOSSIBLE: Serializing metric failed");
            stmt.execute(params![key, storage_name, lifetime.as_str(), encoded])?;
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
