// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs;
use std::path::Path;
use std::str;

use crate::metrics::Metric;
use crate::Error;
use crate::Lifetime;
use crate::Result;

use rkv::StoreOptions;
use rusqlite::Transaction;

use super::sqlite;

pub type Rkv = rkv::Rkv<rkv::backend::SafeModeEnvironment>;
pub type SingleStore = rkv::SingleStore<rkv::backend::SafeModeDatabase>;

pub(crate) const RECORD_SEPARATOR: char = '\x1E';

pub fn rkv_new(path: &Path) -> std::result::Result<Rkv, rkv::StoreError> {
    match Rkv::new::<rkv::backend::SafeMode>(path) {
        // An invalid file can mean:
        // 1. An empty file.
        // 2. A corrupted file.
        //
        // In both instances there's not much we can do.
        // Drop the data by removing the file.
        Err(rkv::StoreError::FileInvalid) => {
            log::debug!("rkv failed: invalid file. starting from scratch.");
            let safebin = path.join("data.safe.bin");
            fs::remove_file(safebin).map_err(|_| rkv::StoreError::FileInvalid)?;
            Err(rkv::StoreError::FileInvalid)
        }
        Err(rkv::StoreError::DatabaseCorrupted) => {
            log::debug!("rkv failed: database corrupted. starting from scratch.");
            let safebin = path.join("data.safe.bin");
            fs::remove_file(safebin).map_err(|_| rkv::StoreError::DatabaseCorrupted)?;
            Err(rkv::StoreError::DatabaseCorrupted)
        }
        other => {
            let rkv = other?;
            Ok(rkv)
        }
    }
}

pub struct Database {
    /// Handle to the database environment.
    rkv: Rkv,

    /// Handles to the "lifetime" stores.
    ///
    /// A "store" is a handle to the underlying database.
    /// We keep them open for fast and frequent access.
    user_store: SingleStore,
    ping_store: SingleStore,
    application_store: SingleStore,
}

impl Database {
    /// Open the Rkv database and the embbedded stores.
    pub fn new(data_path: &Path) -> Result<Self> {
        log::debug!("Rkv database path: {:?}", data_path.display());

        let rkv = Self::open_rkv(data_path)?;
        let user_store = rkv.open_single(Lifetime::User.as_str(), StoreOptions::create())?;
        let ping_store = rkv.open_single(Lifetime::Ping.as_str(), StoreOptions::create())?;
        let application_store =
            rkv.open_single(Lifetime::Application.as_str(), StoreOptions::create())?;

        let db = Self {
            rkv,
            user_store,
            ping_store,
            application_store,
        };

        Ok(db)
    }

    fn open_rkv(path: &Path) -> Result<Rkv> {
        let rkv = rkv_new(path)?;
        Ok(rkv)
    }

    fn get_store(&self, lifetime: Lifetime) -> &SingleStore {
        match lifetime {
            Lifetime::User => &self.user_store,
            Lifetime::Ping => &self.ping_store,
            Lifetime::Application => &self.application_store,
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
    /// * `transaction_fn` - Called for each entry being iterated over.
    ///                      It is passed two arguments: `(key: String, metric: &Metric)`.
    pub fn iter_store<F>(&self, lifetime: Lifetime, mut transaction_fn: F)
    where
        F: FnMut(String, &Metric),
    {
        let Ok(reader) = self.rkv.read() else { return };
        let Ok(mut iter) = self.get_store(lifetime).iter_start(&reader) else {
            log::debug!("No store for {lifetime:?}");
            return;
        };

        while let Some(Ok((key, value))) = iter.next() {
            let Ok(key) = String::from_utf8(key.to_vec()) else {
                log::debug!("Key is not valid UTF-8: {key:?}");
                continue;
            };
            let metric: Metric = match value {
                rkv::Value::Blob(blob) => {
                    let Ok(value) = bincode::deserialize(blob) else {
                        log::debug!("Value for key {key:?} could not be deserialized");
                        continue;
                    };
                    value
                }
                _ => {
                    log::debug!("Blob for key {key:?} is not a valid blob");
                    continue;
                }
            };
            transaction_fn(key, &metric);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MetricKey<'a> {
    ping: &'a str,
    id: &'a str,
    label: Option<String>,
}

/// Split a database key into its metric key parts.
fn split_key(key: &str) -> Option<MetricKey<'_>> {
    let (ping, rest) = key.split_once('#')?;
    if ping.is_empty() || rest.is_empty() {
        return None;
    }

    let (id, labels) = match rest.split_once(|c| ['/', RECORD_SEPARATOR].contains(&c)) {
        Some((id, labels)) => {
            if labels.is_empty() {
                return None;
            }
            (id, labels)
        }
        _ => (rest, ""),
    };
    if id.is_empty() {
        return None;
    }

    let label = if labels.is_empty() {
        // No label at all
        None
    } else if labels.contains(RECORD_SEPARATOR) {
        // Label separated by
        let (key, category) = labels.split_once(RECORD_SEPARATOR)?;

        if key.is_empty() || category.is_empty() {
            return None;
        }

        Some(String::from(labels))
    } else {
        Some(String::from(labels))
    };

    Some(MetricKey { ping, id, label })
}

fn migrate(rkv: &Database, sql_db: &sqlite::Database, tx: &mut Transaction) -> usize {
    let mut migrated_metrics = 0;
    let mut migrate_metric = |lifetime: Lifetime, key: String, metric: &Metric| {
        let Some(metric_id) = split_key(&key) else {
            log::debug!("Invalid metric key: {key:?}");
            return;
        };
        let label = metric_id.label.as_deref().unwrap_or("");
        _ = sql_db.record_per_lifetime(tx, lifetime, metric_id.ping, metric_id.id, label, metric);
        migrated_metrics += 1;
    };

    let snapshotter_user =
        |key: String, metric: &Metric| migrate_metric(Lifetime::User, key, metric);
    rkv.iter_store(Lifetime::User, snapshotter_user);

    let snapshotter_app =
        |key: String, metric: &Metric| migrate_metric(Lifetime::Application, key, metric);
    rkv.iter_store(Lifetime::Application, snapshotter_app);

    let snapshotter_ping =
        |key: String, metric: &Metric| migrate_metric(Lifetime::Ping, key, metric);
    rkv.iter_store(Lifetime::Ping, snapshotter_ping);

    migrated_metrics
}

pub fn try_migrate(data_path: &Path, db: &sqlite::Database) -> Result<()> {
    use super::migration::{self, Database as RkvDatabase};

    let rkv_file = data_path.join("data.safe.bin");
    log::debug!(
        "Trying to migrate. Data path: {}, expected file: {}",
        data_path.display(),
        rkv_file.display()
    );

    if !rkv_file.exists() {
        log::debug!("No rkv file. No migration.");
        return Ok(());
    }

    let Ok(rkv) = RkvDatabase::new(data_path) else {
        log::debug!("Can't open rkv database. No migration.");
        return Ok(());
    };
    let count = db.conn.write(|tx| {
        let count = migration::migrate(&rkv, db, tx);
        Ok::<_, Error>(count)
    })?;

    log::info!("{count} metrics migrated to sqlite");

    log::debug!(
        "Data migrated. Would be removing Rkv database at {}",
        rkv_file.display()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    impl<'a> MetricKey<'a> {
        fn new<'b>(ping: &'a str, id: &'a str, label: impl Into<Option<&'b str>>) -> Self {
            Self {
                ping,
                id,
                label: label.into().map(|s| s.to_string()),
            }
        }
    }

    #[test]
    fn splitting_key() {
        let matches = &[
            (MetricKey::new("metrics", "name", None), "metrics#name"),
            (
                MetricKey::new("metrics", "cat.name", None),
                "metrics#cat.name",
            ),
            (
                MetricKey::new("metrics", "cat1.cat2.name", None),
                "metrics#cat1.cat2.name",
            ),
            (
                MetricKey::new("metrics", "cat1.cat2.name", "label"),
                "metrics#cat1.cat2.name/label",
            ),
            (
                // This currently works. We do allow slashes in labels.
                // Maybe we shouldn't have.
                MetricKey::new("metrics", "cat1.cat2.name", "label1/label2"),
                "metrics#cat1.cat2.name/label1/label2",
            ),
            (
                // This currently works. We do allow slashes in labels.
                // Maybe we shouldn't have.
                MetricKey::new("metrics", "cat.name", "label//"),
                "metrics#cat.name/label//",
            ),
            (
                MetricKey::new("metrics", "cat1.cat2.name", "label1\x1Elabel2"),
                "metrics#cat1.cat2.name\x1elabel1\x1elabel2",
            ),
            (
                MetricKey::new("glean_internal_info", "baseline#sequence", None),
                "glean_internal_info#baseline#sequence",
            ),
        ];

        for (exp, key) in matches {
            let m = split_key(key).unwrap_or_else(|| panic!("{key:?} should be splittable"));
            assert_eq!(*exp, m, "did not split correctly: {key:?}");
        }
    }

    #[test]
    fn splitting_key_fails() {
        let matches = &[
            "",
            "metrics",
            "metrics#",
            "#cat",
            "#cat.name",
            "metrics#/",
            "metrics#//",
            "metrics#/label",
            "metrics#cat.name/",
            "metrics#cat.name\x1e",
            "metrics#cat.name\x1e\x1e",
            "metrics#cat.name\x1elabel1\x1e",
            "metrics#cat.name\x1e\x1elabel2",
        ];

        for key in matches {
            assert_eq!(None, split_key(key), "should not split: {key:?}");
        }
    }
}
