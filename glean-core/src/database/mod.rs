// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fs;
use std::str;
use std::sync::RwLock;

use chrono::{Local, NaiveDate};
use lazy_static::lazy_static;
use rkv::{Rkv, SingleStore, StoreOptions};

use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Glean;
use crate::Lifetime;
use crate::Result;

lazy_static! {
    // A map of all the known 'user' lifetime metrics along with their expiration
    // dates. See bug 1604854 for more context on why this is needed.
    static ref USER_LIFETIME_EXPIRATION_MAP: HashMap<&'static str, NaiveDate> = vec![
        ("glean_internal_test.user_metric_expired", NaiveDate::from_ymd(2015, 3, 14)),
    ].into_iter().collect();

    // Get the current date at runtime. While we should use the build-date to be
    // consistent with other metric types, given the edge case this already is,
    // it might be enough to go with a run-time date.
    static ref STARTUP_LOCAL_DATE: NaiveDate = Local::now().naive_utc().date();
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

    /// If the `delay_ping_lifetime_io` Glean config option is `true`,
    /// we will save metrics with 'ping' lifetime data in a map temporarily
    /// so as to persist them to disk using rkv in bulk on demand.
    ping_lifetime_data: Option<RwLock<BTreeMap<String, Metric>>>,
}

impl std::fmt::Debug for Database {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("Database")
            .field("rkv", &self.rkv)
            .field("user_store", &"SingleStore")
            .field("ping_store", &"SingleStore")
            .field("application_store", &"SingleStore")
            .field("ping_lifetime_data", &self.ping_lifetime_data)
            .finish()
    }
}

impl Database {
    /// Initialize the data store.
    ///
    /// This opens the underlying rkv store and creates
    /// the underlying directory structure.
    ///
    /// It also loads any Lifetime::Ping data that might be
    /// persisted, in case `delay_ping_lifetime_io` is set.
    pub fn new(data_path: &str, delay_ping_lifetime_io: bool) -> Result<Self> {
        let rkv = Self::open_rkv(data_path)?;
        let user_store = rkv.open_single(Lifetime::User.as_str(), StoreOptions::create())?;
        let ping_store = rkv.open_single(Lifetime::Ping.as_str(), StoreOptions::create())?;
        let application_store =
            rkv.open_single(Lifetime::Application.as_str(), StoreOptions::create())?;
        let ping_lifetime_data = if delay_ping_lifetime_io {
            Some(RwLock::new(BTreeMap::new()))
        } else {
            None
        };

        let db = Self {
            rkv,
            user_store,
            ping_store,
            application_store,
            ping_lifetime_data,
        };

        // Force the startup date to be initialized.
        log::debug!("Startup UTC date is: {:?}", *STARTUP_LOCAL_DATE);

        db.load_ping_lifetime_data();

        Ok(db)
    }

    fn get_store(&self, lifetime: Lifetime) -> &SingleStore {
        match lifetime {
            Lifetime::User => &self.user_store,
            Lifetime::Ping => &self.ping_store,
            Lifetime::Application => &self.application_store,
        }
    }

    /// Creates the storage directories and inits rkv.
    fn open_rkv(path: &str) -> Result<Rkv> {
        let path = std::path::Path::new(path).join("db");
        log::debug!("Database path: {:?}", path.display());
        fs::create_dir_all(&path)?;

        let rkv = Rkv::new(&path)?;
        log::info!("Database initialized");
        Ok(rkv)
    }

    /// Build the key of the final location of the data in the database.
    /// Such location is built using the storage name and the metric
    /// key/name (if available).
    ///
    /// ## Arguments
    ///
    /// * `storage_name` - the name of the storage to store/fetch data from.
    /// * `metric_key` - the optional metric key/name.
    ///
    /// ## Return value
    ///
    /// Returns a String representing the location, in the database, data must
    /// be written or read from.
    fn get_storage_key(storage_name: &str, metric_key: Option<&str>) -> String {
        match metric_key {
            Some(k) => format!("{}#{}", storage_name, k),
            None => format!("{}#", storage_name),
        }
    }

    /// Loads Lifetime::Ping data from rkv to memory,
    /// if `delay_ping_lifetime_io` is set to true.
    ///
    /// Does nothing if it isn't or if there is not data to load.
    fn load_ping_lifetime_data(&self) {
        if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
            let mut data = ping_lifetime_data
                .write()
                .expect("Can't read ping lifetime data");

            let reader = unwrap_or!(self.rkv.read(), return);
            let store = self.get_store(Lifetime::Ping);
            let mut iter = unwrap_or!(store.iter_start(&reader), return);

            while let Some(Ok((metric_name, value))) = iter.next() {
                let metric_name = match str::from_utf8(metric_name) {
                    Ok(metric_name) => metric_name.to_string(),
                    _ => continue,
                };
                let metric: Metric = match value.expect("Value missing in iteration") {
                    rkv::Value::Blob(blob) => unwrap_or!(bincode::deserialize(blob), continue),
                    _ => continue,
                };

                data.insert(metric_name, metric);
            }
        }
    }

    /// Checks if the 'user' lifetime metric is expired and, if so, removes it.
    ///
    /// ## Arguments
    ///
    /// * `storage_name`: The storage name to iterate over.
    /// * `metric_id_slice`: The metric id to check.
    ///
    /// ## Return value
    ///
    /// Returns `true` if the metric is expired and should not be reported,
    /// `false` otherwise.
    ///
    /// ## Panics
    ///
    /// This function will **not** panic on database errors.
    fn check_and_remove_expired_user_lifetime(
        &self,
        storage_name: &str,
        metric_id_slice: &[u8],
    ) -> bool {
        let metric_name = match str::from_utf8(metric_id_slice) {
            Ok(metric_name) => metric_name.to_string(),
            _ => return false,
        };

        match USER_LIFETIME_EXPIRATION_MAP.get(metric_name.as_str()) {
            Some(&expiration) => {
                let is_expired = *STARTUP_LOCAL_DATE > expiration;
                if is_expired {
                    // This should be fine to delete now, even though we may be iterating
                    // through the database entries.
                    if let Err(e) =
                        self.remove_single_metric(Lifetime::User, storage_name, &metric_name)
                    {
                        log::error!(
                            "Failed to remove  expired {} from {}: {:?}",
                            metric_name,
                            storage_name,
                            e
                        );
                    } else {
                        log::debug!("Removed expired {} from {}", metric_name, storage_name);
                    }
                }
                // While we might not be able to remove it from the database, we can still
                // make sure to not report it.
                true
            }
            // If we can't find the expiration in our table, that's fine. We might
            // not know about it, or it's really not meant to expire.
            _ => false,
        }
    }

    /// Iterates with the provided transaction function over the requested data
    /// from the given storage.
    ///
    /// * If the storage is unavailable, the transaction function is never invoked.
    /// * If the read data cannot be deserialized it will be silently skipped.
    ///
    /// ## Arguments
    ///
    /// * `lifetime`: The metric lifetime to iterate over.
    /// * `storage_name`: The storage name to iterate over.
    /// * `metric_key`: The metric key to iterate over. All metrics iterated over
    ///   will have this prefix. For example, if `metric_key` is of the form `{category}.`,
    ///   it will iterate over all metrics in the given category. If the `metric_key` is of the
    ///   form `{category}.{name}/`, the iterator will iterate over all specific metrics for
    ///   a given labeled metric. If not provided, the entire storage for the given lifetime
    ///   will be iterated over.
    /// * `transaction_fn`: Called for each entry being iterated over. It is
    ///   passed two arguments: `(metric_name: &[u8], metric: &Metric)`.
    ///
    /// ## Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn iter_store_from<F>(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        metric_key: Option<&str>,
        mut transaction_fn: F,
    ) where
        F: FnMut(&[u8], &Metric),
    {
        let iter_start = Self::get_storage_key(storage_name, metric_key);
        let len = iter_start.len();

        // Lifetime::Ping data is not immediately persisted to disk if
        // Glean has `delay_ping_lifetime_io` set to true
        if lifetime == Lifetime::Ping {
            if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
                let data = ping_lifetime_data
                    .read()
                    .expect("Can't read ping lifetime data");
                for (key, value) in data.iter() {
                    if key.starts_with(&iter_start) {
                        let key = &key[len..];
                        transaction_fn(key.as_bytes(), value);
                    }
                }
                return;
            }
        }

        let reader = unwrap_or!(self.rkv.read(), return);
        let mut iter = unwrap_or!(
            self.get_store(lifetime).iter_from(&reader, &iter_start),
            return
        );

        while let Some(Ok((metric_name, value))) = iter.next() {
            if !metric_name.starts_with(iter_start.as_bytes()) {
                break;
            }

            let metric_name = &metric_name[len..];

            // Don't report the 'user' lifetiem metric if it expired and additionally
            // mark it for removal. We need to do it now as we don't know the storage
            // names beforehand.
            if lifetime == Lifetime::User
                && self.check_and_remove_expired_user_lifetime(storage_name, metric_name)
            {
                continue;
            }

            let metric: Metric = match value.expect("Value missing in iteration") {
                rkv::Value::Blob(blob) => unwrap_or!(bincode::deserialize(blob), continue),
                _ => continue,
            };
            transaction_fn(metric_name, &metric);
        }
    }

    /// Determine if the storage has the given metric.
    ///
    /// If data cannot be read it is assumed that the storage does not have the metric.
    ///
    /// ## Arguments
    ///
    /// * `lifetime`: The lifetime of the metric.
    /// * `storage_name`: The storage name to look in.
    /// * `metric_identifier`: The metric identifier.
    ///
    /// ## Panics
    ///
    /// This function will **not** panic on database errors.
    pub fn has_metric(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        metric_identifier: &str,
    ) -> bool {
        let key = Self::get_storage_key(storage_name, Some(metric_identifier));

        // Lifetime::Ping data is not persisted to disk if
        // Glean has `delay_ping_lifetime_io` set to true
        if lifetime == Lifetime::Ping {
            if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
                return ping_lifetime_data
                    .read()
                    .map(|data| data.contains_key(&key))
                    .unwrap_or(false);
            }
        }

        let reader = unwrap_or!(self.rkv.read(), return false);
        self.get_store(lifetime)
            .get(&reader, &key)
            .unwrap_or(None)
            .is_some()
    }

    /// Write to the specified storage with the provided transaction function.
    ///
    /// If the storage is unavailable, it will return an error.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn write_with_store<F>(&self, store_name: Lifetime, mut transaction_fn: F) -> Result<()>
    where
        F: FnMut(rkv::Writer, &SingleStore) -> Result<()>,
    {
        let writer = self.rkv.write().unwrap();
        let store = self.get_store(store_name);
        transaction_fn(writer, store)
    }

    /// Records a metric in the underlying storage system.
    pub fn record(&self, glean: &Glean, data: &CommonMetricData, value: &Metric) {
        let name = data.identifier(glean);

        for ping_name in data.storage_names() {
            if let Err(e) = self.record_per_lifetime(data.lifetime, ping_name, &name, value) {
                log::error!("Failed to record metric into {}: {:?}", ping_name, e);
            }
        }
    }

    /// Records a metric in the underlying storage system, for a single lifetime.
    ///
    /// ## Return value
    ///
    /// If the storage is unavailable or the write fails, no data will be stored and an error will be returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    fn record_per_lifetime(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        metric: &Metric,
    ) -> Result<()> {
        let final_key = Self::get_storage_key(storage_name, Some(key));

        // Lifetime::Ping data is not immediately persisted to disk if
        // Glean has `delay_ping_lifetime_io` set to true
        if lifetime == Lifetime::Ping {
            if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
                let mut data = ping_lifetime_data
                    .write()
                    .expect("Can't read ping lifetime data");
                data.insert(final_key, metric.clone());
                return Ok(());
            }
        }

        let encoded = bincode::serialize(&metric).expect("IMPOSSIBLE: Serializing metric failed");
        let value = rkv::Value::Blob(&encoded);

        let mut writer = self.rkv.write()?;
        self.get_store(lifetime)
            .put(&mut writer, final_key, &value)?;
        writer.commit()?;
        Ok(())
    }

    /// Records the provided value, with the given lifetime, after
    /// applying a transformation function.
    pub fn record_with<F>(&self, glean: &Glean, data: &CommonMetricData, mut transform: F)
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let name = data.identifier(glean);
        for ping_name in data.storage_names() {
            if let Err(e) =
                self.record_per_lifetime_with(data.lifetime, ping_name, &name, &mut transform)
            {
                log::error!("Failed to record metric into {}: {:?}", ping_name, e);
            }
        }
    }

    /// Records a metric in the underlying storage system, after applying the
    /// given transformation function, for a single lifetime.
    ///
    /// ## Return value
    ///
    /// If the storage is unavailable or the write fails, no data will be stored and an error will be returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn record_per_lifetime_with<F>(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        mut transform: F,
    ) -> Result<()>
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let final_key = Self::get_storage_key(storage_name, Some(key));

        // Lifetime::Ping data is not persisted to disk if
        // Glean has `delay_ping_lifetime_io` set to true
        if lifetime == Lifetime::Ping {
            if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
                let mut data = ping_lifetime_data
                    .write()
                    .expect("Can't access ping lifetime data as writable");
                let entry = data.entry(final_key);
                match entry {
                    Entry::Vacant(entry) => {
                        entry.insert(transform(None));
                    }
                    Entry::Occupied(mut entry) => {
                        let old_value = entry.get().clone();
                        entry.insert(transform(Some(old_value)));
                    }
                }
                return Ok(());
            }
        }

        let mut writer = self.rkv.write()?;
        let store = self.get_store(lifetime);
        let new_value: Metric = {
            let old_value = store.get(&writer, &final_key)?;

            match old_value {
                Some(rkv::Value::Blob(blob)) => {
                    let old_value = bincode::deserialize(blob).ok();
                    transform(old_value)
                }
                _ => transform(None),
            }
        };

        let encoded =
            bincode::serialize(&new_value).expect("IMPOSSIBLE: Serializing metric failed");
        let value = rkv::Value::Blob(&encoded);
        store.put(&mut writer, final_key, &value)?;
        writer.commit()?;
        Ok(())
    }

    /// Clears a storage (only Ping Lifetime).
    ///
    /// ## Return value
    ///
    /// * If the storage is unavailable an error is returned.
    /// * If any individual delete fails, an error is returned, but other deletions might have
    ///   happened.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn clear_ping_lifetime_storage(&self, storage_name: &str) -> Result<()> {
        // Lifetime::Ping data will be saved to `ping_lifetime_data`
        // in case `delay_ping_lifetime_io` is set to true
        if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
            ping_lifetime_data
                .write()
                .expect("Can't access ping lifetime data as writable")
                .clear();
        }

        self.write_with_store(Lifetime::Ping, |mut writer, store| {
            let mut metrics = Vec::new();
            {
                let mut iter = store.iter_from(&writer, &storage_name)?;
                while let Some(Ok((metric_name, _))) = iter.next() {
                    if let Ok(metric_name) = std::str::from_utf8(metric_name) {
                        if !metric_name.starts_with(&storage_name) {
                            break;
                        }
                        metrics.push(metric_name.to_owned());
                    }
                }
            }

            let mut res = Ok(());
            for to_delete in metrics {
                if let Err(e) = store.delete(&mut writer, to_delete) {
                    log::error!("Can't delete from store: {:?}", e);
                    res = Err(e);
                }
            }

            writer.commit()?;
            Ok(res?)
        })
    }

    /// Removes a single metric from the storage.
    ///
    /// ## Arguments
    ///
    /// * `lifetime` - the lifetime of the storage in which to look for the metric.
    /// * `storage_name` - the name of the storage to store/fetch data from.
    /// * `metric_key` - the metric key/name.
    ///
    /// ## Return value
    ///
    /// * If the storage is unavailable an error is returned.
    /// * If the metric could not be deleted, an error is returned.
    ///
    /// Otherwise `Ok(())` is returned.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn remove_single_metric(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        metric_name: &str,
    ) -> Result<()> {
        let final_key = Self::get_storage_key(storage_name, Some(metric_name));

        // Lifetime::Ping data is not persisted to disk if
        // Glean has `delay_ping_lifetime_io` set to true
        if lifetime == Lifetime::Ping {
            if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
                let mut data = ping_lifetime_data
                    .write()
                    .expect("Can't access app lifetime data as writable");
                data.remove(&final_key);
            }
        }

        self.write_with_store(lifetime, |mut writer, store| {
            if let Err(e) = store.delete(&mut writer, final_key.clone()) {
                if self.ping_lifetime_data.is_some() {
                    // If ping_lifetime_data exists, it might be
                    // that data is in memory, but not yet in rkv.
                    return Ok(());
                }
                return Err(e.into());
            }
            writer.commit()?;
            Ok(())
        })
    }

    /// Clears all the metrics in the database, for the provided lifetime.
    ///
    /// Errors are logged.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn clear_lifetime(&self, lifetime: Lifetime) {
        let res = self.write_with_store(lifetime, |mut writer, store| {
            store.clear(&mut writer)?;
            writer.commit()?;
            Ok(())
        });
        if let Err(e) = res {
            log::error!("Could not clear store for lifetime {:?}: {:?}", lifetime, e);
        }
    }

    /// Clears all metrics in the database.
    ///
    /// Errors are logged.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn clear_all(&self) {
        if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
            ping_lifetime_data
                .write()
                .expect("Can't access ping lifetime data as writable")
                .clear();
        }

        for lifetime in [Lifetime::User, Lifetime::Ping, Lifetime::Application].iter() {
            self.clear_lifetime(*lifetime);
        }
    }

    /// Persist ping_lifetime_data to disk.
    ///
    /// Does nothing in case there is nothing to persist.
    ///
    /// ## Panics
    ///
    /// * This function will **not** panic on database errors.
    pub fn persist_ping_lifetime_data(&self) -> Result<()> {
        if let Some(ping_lifetime_data) = &self.ping_lifetime_data {
            let data = ping_lifetime_data
                .read()
                .expect("Can't read ping lifetime data");

            self.write_with_store(Lifetime::Ping, |mut writer, store| {
                for (key, value) in data.iter() {
                    let encoded =
                        bincode::serialize(&value).expect("IMPOSSIBLE: Serializing metric failed");
                    // There is no need for `get_storage_key` here because
                    // the key is already formatted from when it was saved
                    // to ping_lifetime_data.
                    store.put(&mut writer, &key, &rkv::Value::Blob(&encoded))?;
                }
                writer.commit()?;
                Ok(())
            })?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_panicks_if_fails_dir_creation() {
        assert!(Database::new("/!#\"'@#°ç", false).is_err());
    }

    #[test]
    fn test_data_dir_rkv_inits() {
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();

        Database::new(&str_dir, false).unwrap();

        assert!(dir.path().exists());
    }

    #[test]
    fn test_ping_lifetime_metric_recorded() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        assert!(db.ping_lifetime_data.is_none());

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        )
        .unwrap();

        // Verify that the data is correctly recorded.
        let mut found_metrics = 0;
        let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
            found_metrics += 1;
            let metric_id = String::from_utf8_lossy(metric_name).into_owned();
            assert_eq!(test_metric_id, metric_id);
            match metric {
                Metric::String(s) => assert_eq!(test_value, s),
                _ => panic!("Unexpected data found"),
            }
        };

        db.iter_store_from(Lifetime::Ping, test_storage, None, &mut snapshotter);
        assert_eq!(1, found_metrics, "We only expect 1 Lifetime.Ping metric.");
    }

    #[test]
    fn test_application_lifetime_metric_recorded() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage1";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::Application,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        )
        .unwrap();

        // Verify that the data is correctly recorded.
        let mut found_metrics = 0;
        let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
            found_metrics += 1;
            let metric_id = String::from_utf8_lossy(metric_name).into_owned();
            assert_eq!(test_metric_id, metric_id);
            match metric {
                Metric::String(s) => assert_eq!(test_value, s),
                _ => panic!("Unexpected data found"),
            }
        };

        db.iter_store_from(Lifetime::Application, test_storage, None, &mut snapshotter);
        assert_eq!(
            1, found_metrics,
            "We only expect 1 Lifetime.Application metric."
        );
    }

    #[test]
    fn test_user_lifetime_metric_recorded() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage2";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::User,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        )
        .unwrap();

        // Verify that the data is correctly recorded.
        let mut found_metrics = 0;
        let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
            found_metrics += 1;
            let metric_id = String::from_utf8_lossy(metric_name).into_owned();
            assert_eq!(test_metric_id, metric_id);
            match metric {
                Metric::String(s) => assert_eq!(test_value, s),
                _ => panic!("Unexpected data found"),
            }
        };

        db.iter_store_from(Lifetime::User, test_storage, None, &mut snapshotter);
        assert_eq!(1, found_metrics, "We only expect 1 Lifetime.User metric.");
    }

    #[test]
    fn test_clear_ping_storage() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        // Attempt to record a known value for every single lifetime.
        let test_storage = "test-storage";
        db.record_per_lifetime(
            Lifetime::User,
            test_storage,
            "telemetry_test.test_name_user",
            &Metric::String("test-value-user".to_string()),
        )
        .unwrap();
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            "telemetry_test.test_name_ping",
            &Metric::String("test-value-ping".to_string()),
        )
        .unwrap();
        db.record_per_lifetime(
            Lifetime::Application,
            test_storage,
            "telemetry_test.test_name_application",
            &Metric::String("test-value-application".to_string()),
        )
        .unwrap();

        // Take a snapshot for the data, all the lifetimes.
        {
            let mut snapshot: HashMap<String, String> = HashMap::new();
            let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
                let metric_name = String::from_utf8_lossy(metric_name).into_owned();
                match metric {
                    Metric::String(s) => snapshot.insert(metric_name, s.to_string()),
                    _ => panic!("Unexpected data found"),
                };
            };

            db.iter_store_from(Lifetime::User, test_storage, None, &mut snapshotter);
            db.iter_store_from(Lifetime::Ping, test_storage, None, &mut snapshotter);
            db.iter_store_from(Lifetime::Application, test_storage, None, &mut snapshotter);

            assert_eq!(3, snapshot.len(), "We expect all lifetimes to be present.");
            assert!(snapshot.contains_key("telemetry_test.test_name_user"));
            assert!(snapshot.contains_key("telemetry_test.test_name_ping"));
            assert!(snapshot.contains_key("telemetry_test.test_name_application"));
        }

        // Clear the Ping lifetime.
        db.clear_ping_lifetime_storage(test_storage).unwrap();

        // Take a snapshot again and check that we're only clearing the Ping lifetime.
        {
            let mut snapshot: HashMap<String, String> = HashMap::new();
            let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
                let metric_name = String::from_utf8_lossy(metric_name).into_owned();
                match metric {
                    Metric::String(s) => snapshot.insert(metric_name, s.to_string()),
                    _ => panic!("Unexpected data found"),
                };
            };

            db.iter_store_from(Lifetime::User, test_storage, None, &mut snapshotter);
            db.iter_store_from(Lifetime::Ping, test_storage, None, &mut snapshotter);
            db.iter_store_from(Lifetime::Application, test_storage, None, &mut snapshotter);

            assert_eq!(2, snapshot.len(), "We only expect 2 metrics to be left.");
            assert!(snapshot.contains_key("telemetry_test.test_name_user"));
            assert!(snapshot.contains_key("telemetry_test.test_name_application"));
        }
    }

    #[test]
    fn test_remove_single_metric() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        let test_storage = "test-storage-single-lifetime";
        let metric_id_pattern = "telemetry_test.single_metric";

        // Write sample metrics to the database.
        let lifetimes = vec![Lifetime::User, Lifetime::Ping, Lifetime::Application];

        for lifetime in lifetimes.iter() {
            for value in &["retain", "delete"] {
                db.record_per_lifetime(
                    *lifetime,
                    test_storage,
                    &format!("{}_{}", metric_id_pattern, value),
                    &Metric::String((*value).to_string()),
                )
                .unwrap();
            }
        }

        // Remove "telemetry_test.single_metric_delete" from each lifetime.
        for lifetime in lifetimes.iter() {
            db.remove_single_metric(
                *lifetime,
                test_storage,
                &format!("{}_delete", metric_id_pattern),
            )
            .unwrap();
        }

        // Verify that "telemetry_test.single_metric_retain" is still around for all lifetimes.
        for lifetime in lifetimes.iter() {
            let mut found_metrics = 0;
            let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
                found_metrics += 1;
                let metric_id = String::from_utf8_lossy(metric_name).into_owned();
                assert_eq!(format!("{}_retain", metric_id_pattern), metric_id);
                match metric {
                    Metric::String(s) => assert_eq!("retain", s),
                    _ => panic!("Unexpected data found"),
                }
            };

            // Check the User lifetime.
            db.iter_store_from(*lifetime, test_storage, None, &mut snapshotter);
            assert_eq!(
                1, found_metrics,
                "We only expect 1 metric for this lifetime."
            );
        }
    }

    #[test]
    fn test_delayed_ping_lifetime_persistence() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, true).unwrap();
        let test_storage = "test-storage";

        assert!(db.ping_lifetime_data.is_some());

        // Attempt to record a known value.
        let test_value1 = "test-value1";
        let test_metric_id1 = "telemetry_test.test_name1";
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            test_metric_id1,
            &Metric::String(test_value1.to_string()),
        )
        .unwrap();

        // Attempt to persist data.
        db.persist_ping_lifetime_data().unwrap();

        // Attempt to record another known value.
        let test_value2 = "test-value2";
        let test_metric_id2 = "telemetry_test.test_name2";
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            test_metric_id2,
            &Metric::String(test_value2.to_string()),
        )
        .unwrap();

        {
            // At this stage we expect `test_value1` to be persisted and in memory,
            // since it was recorded before calling `persist_ping_lifetime_data`,
            // and `test_value2` to be only in memory, since it was recorded after.
            let store: SingleStore = db
                .rkv
                .open_single(Lifetime::Ping.as_str(), StoreOptions::create())
                .unwrap();
            let reader = db.rkv.read().unwrap();

            // Verify that test_value1 is in rkv.
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id1))
                .unwrap_or(None)
                .is_some());
            // Verifiy that test_value2 is **not** in rkv.
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id2))
                .unwrap_or(None)
                .is_none());

            let data = match &db.ping_lifetime_data {
                Some(ping_lifetime_data) => ping_lifetime_data,
                None => panic!("Expected `ping_lifetime_data` to exist here!"),
            };
            let data = data.read().unwrap();
            // Verify that test_value1 is also in memory.
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id1))
                .is_some());
            // Verify that test_value2 is in memory.
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id2))
                .is_some());
        }

        // Attempt to persist data again.
        db.persist_ping_lifetime_data().unwrap();

        {
            // At this stage we expect `test_value1` and `test_value2` to
            // be persisted, since both were created before a call to
            // `persist_ping_lifetime_data`.
            let store: SingleStore = db
                .rkv
                .open_single(Lifetime::Ping.as_str(), StoreOptions::create())
                .unwrap();
            let reader = db.rkv.read().unwrap();

            // Verify that test_value1 is in rkv.
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id1))
                .unwrap_or(None)
                .is_some());
            // Verifiy that test_value2 is also in rkv.
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id2))
                .unwrap_or(None)
                .is_some());

            let data = match &db.ping_lifetime_data {
                Some(ping_lifetime_data) => ping_lifetime_data,
                None => panic!("Expected `ping_lifetime_data` to exist here!"),
            };
            let data = data.read().unwrap();
            // Verify that test_value1 is also in memory.
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id1))
                .is_some());
            // Verify that test_value2 is also in memory.
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id2))
                .is_some());
        }
    }

    #[test]
    fn test_load_ping_lifetime_data_from_memory() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();

        let test_storage = "test-storage";
        let test_value = "test-value";
        let test_metric_id = "telemetry_test.test_name";

        {
            let db = Database::new(&str_dir, true).unwrap();

            // Attempt to record a known value.
            db.record_per_lifetime(
                Lifetime::Ping,
                test_storage,
                test_metric_id,
                &Metric::String(test_value.to_string()),
            )
            .unwrap();

            // Verify that test_value is in memory.
            let data = match &db.ping_lifetime_data {
                Some(ping_lifetime_data) => ping_lifetime_data,
                None => panic!("Expected `ping_lifetime_data` to exist here!"),
            };
            let data = data.read().unwrap();
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id))
                .is_some());

            // Attempt to persist data.
            db.persist_ping_lifetime_data().unwrap();

            // Verify that test_value is now in rkv.
            let store: SingleStore = db
                .rkv
                .open_single(Lifetime::Ping.as_str(), StoreOptions::create())
                .unwrap();
            let reader = db.rkv.read().unwrap();
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id))
                .unwrap_or(None)
                .is_some());
        }

        // Now create a new instace of the db and check if data was
        // correctly loaded from rkv to memory.
        {
            let db = Database::new(&str_dir, true).unwrap();

            // Verify that test_value is in memory.
            let data = match &db.ping_lifetime_data {
                Some(ping_lifetime_data) => ping_lifetime_data,
                None => panic!("Expected `ping_lifetime_data` to exist here!"),
            };
            let data = data.read().unwrap();
            assert!(data
                .get(&format!("{}#{}", test_storage, test_metric_id))
                .is_some());

            // Verify that test_value is also in rkv.
            let store: SingleStore = db
                .rkv
                .open_single(Lifetime::Ping.as_str(), StoreOptions::create())
                .unwrap();
            let reader = db.rkv.read().unwrap();
            assert!(store
                .get(&reader, format!("{}#{}", test_storage, test_metric_id))
                .unwrap_or(None)
                .is_some());
        }
    }

    #[test]
    fn test_purge_expired_user_lifetime_metrics() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir, false).unwrap();

        let test_storage = "test-storage-expired-user";
        let metric_id_pattern = "glean_internal_test.user_metric";

        // Write sample metrics to the database. We write all lifetimes to
        // make sure we don't mess with anything else that's already stored.
        let lifetimes = vec![Lifetime::User, Lifetime::Ping, Lifetime::Application];

        for lifetime in lifetimes.iter() {
            db.record_per_lifetime(
                *lifetime,
                test_storage,
                &format!("{}_retain", metric_id_pattern),
                &Metric::String("retain".to_string()),
            )
            .unwrap();
        }

        // Save a 'user'-lifetime metric that will be expired.
        db.record_per_lifetime(
            Lifetime::User,
            test_storage,
            &format!("{}_expired", metric_id_pattern),
            &Metric::String("expired".to_string()),
        )
        .unwrap();

        // Verify that "telemetry_test.single_metric_retain" is still around for all lifetimes.
        let mut total_found_metrics = 0;
        for lifetime in lifetimes.iter() {
            let mut found_metrics = 0;
            let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
                found_metrics += 1;
                let metric_id = String::from_utf8_lossy(metric_name).into_owned();
                assert_eq!(format!("{}_retain", metric_id_pattern), metric_id);
                match metric {
                    Metric::String(s) => assert_eq!("retain", s),
                    _ => panic!("Unexpected data found"),
                }
            };

            // Check the count of the metrics that were found for this lifetime.
            db.iter_store_from(*lifetime, test_storage, None, &mut snapshotter);
            assert_eq!(
                1, found_metrics,
                "We only expect 1 metric for this lifetime."
            );
            // Increment the total count as well.
            total_found_metrics += found_metrics;
        }

        assert_eq!(
            3, total_found_metrics,
            "We only expect 3 metrics to be reported across all lifetimes."
        );
    }
}
