// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fs;
use std::sync::RwLock;

use rkv::{Rkv, SingleStore, StoreOptions};

use crate::metrics::Metric;
use crate::CommonMetricData;
use crate::Lifetime;
use crate::Result;

#[derive(Debug)]
pub struct Database {
    rkv: Rkv,
    // Metrics with 'application' lifetime only live as long
    // as the application lives: they don't need to be persisted
    // to disk using rkv. Store them in a map.
    app_lifetime_data: RwLock<BTreeMap<String, Metric>>,
}

impl Database {
    /// Initialize the data store.
    ///
    /// This opens the underlying rkv store and creates
    /// the underlying directory structure.
    pub fn new(data_path: &str) -> Result<Self> {
        Ok(Self {
            rkv: Self::open_rkv(data_path)?,
            app_lifetime_data: RwLock::new(BTreeMap::new()),
        })
    }

    /// Creates the storage directories and inits rkv.
    fn open_rkv(path: &str) -> Result<Rkv> {
        let path = std::path::Path::new(path);
        log::info!("Path is: {:?}", path.display());
        fs::create_dir_all(&path)?;

        let rkv = Rkv::new(path)?;
        log::info!("Rkv done. We are initialized!");
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

    /// Iterates with the provided transaction function on the data from
    /// the given storage.
    pub fn iter_store_from<F>(&self, lifetime: Lifetime, storage_name: &str, mut transaction_fn: F)
    where
        F: FnMut(&[u8], &Metric),
    {
        let iter_start = Self::get_storage_key(storage_name, None);
        let len = iter_start.len();

        // Lifetime::Application data is not persisted to disk
        if lifetime == Lifetime::Application {
            let data = self.app_lifetime_data.read().unwrap();
            for (key, value) in data.iter() {
                if key.starts_with(&iter_start) {
                    let key = &key[len..];
                    transaction_fn(key.as_bytes(), value);
                }
            }

            return;
        }

        let store: SingleStore = self
            .rkv
            .open_single(lifetime.as_str(), StoreOptions::create())
            .unwrap();
        let reader = self.rkv.read().unwrap();
        let mut iter = store.iter_from(&reader, &iter_start).unwrap();

        while let Some(Ok((metric_name, value))) = iter.next() {
            if !metric_name.starts_with(iter_start.as_bytes()) {
                break;
            }

            let metric_name = &metric_name[len..];
            let metric: Metric = match value.unwrap() {
                rkv::Value::Blob(blob) => bincode::deserialize(blob).unwrap(),
                _ => continue,
            };
            transaction_fn(metric_name, &metric);
        }
    }

    pub fn write_with_store<F>(&self, store_name: Lifetime, mut transaction_fn: F)
    where
        F: FnMut(rkv::Writer, SingleStore),
    {
        if store_name == Lifetime::Application {
            panic!("Can't write with store for application-lifetime data");
        }

        let store: SingleStore = self
            .rkv
            .open_single(store_name.as_str(), StoreOptions::create())
            .unwrap();
        let writer = self.rkv.write().unwrap();
        transaction_fn(writer, store);
    }

    /// Records a metric in the underlying storage system.
    pub fn record(&self, data: &CommonMetricData, value: &Metric) {
        let name = data.identifier();

        for ping_name in data.storage_names() {
            self.record_per_lifetime(data.lifetime, ping_name, &name, value);
        }
    }

    /// Records a metric in the underlying storage system, for
    /// a single lifetime.
    fn record_per_lifetime(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        metric: &Metric,
    ) {
        let final_key = Self::get_storage_key(storage_name, Some(key));

        if lifetime == Lifetime::Application {
            let mut data = self.app_lifetime_data.write().unwrap();
            data.insert(final_key, metric.clone());
            return;
        }

        let encoded = bincode::serialize(&metric).unwrap();
        let value = rkv::Value::Blob(&encoded);

        let store_name = lifetime.as_str();
        let store = self
            .rkv
            .open_single(store_name, StoreOptions::create())
            .unwrap();

        let mut writer = self.rkv.write().unwrap();
        store.put(&mut writer, final_key, &value).unwrap();
        let _ = writer.commit();
    }

    /// Records the provided value, with the given lifetime, after
    /// applying a transformation function.
    pub fn record_with<F>(&self, data: &CommonMetricData, mut transform: F)
    where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let name = data.identifier();
        for ping_name in data.storage_names() {
            self.record_per_lifetime_with(data.lifetime, ping_name, &name, &mut transform);
        }
    }

    /// Records a metric in the underlying storage system, after applying the
    /// given transformation function, for a single lifetime.
    pub fn record_per_lifetime_with<F>(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        mut transform: F,
    ) where
        F: FnMut(Option<Metric>) -> Metric,
    {
        let final_key = Self::get_storage_key(storage_name, Some(key));

        if lifetime == Lifetime::Application {
            let mut data = self.app_lifetime_data.write().unwrap();
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
            return;
        }

        let store_name = lifetime.as_str();
        let store = self
            .rkv
            .open_single(store_name, StoreOptions::create())
            .unwrap();

        let mut writer = self.rkv.write().unwrap();
        let new_value: Metric = {
            let old_value = store.get(&writer, &final_key).unwrap();

            match old_value {
                Some(rkv::Value::Blob(blob)) => {
                    let old_value = bincode::deserialize(blob).ok();
                    transform(old_value)
                }
                _ => transform(None),
            }
        };

        let encoded = bincode::serialize(&new_value).unwrap();
        let value = rkv::Value::Blob(&encoded);
        store.put(&mut writer, final_key, &value).unwrap();
        let _ = writer.commit();
    }

    /// Clears a storage (only Ping Lifetime).
    pub fn clear_ping_lifetime_storage(&self, storage_name: &str) {
        self.write_with_store(Lifetime::Ping, |mut writer, store| {
            let mut metrics = Vec::new();
            {
                let mut iter = store.iter_from(&writer, &storage_name).unwrap();
                while let Some(Ok((metric_name, _))) = iter.next() {
                    if let Ok(metric_name) = std::str::from_utf8(metric_name) {
                        if !metric_name.starts_with(&storage_name) {
                            break;
                        }
                        metrics.push(metric_name.to_owned());
                    }
                }
            }

            for to_delete in metrics {
                store.delete(&mut writer, to_delete).unwrap();
            }

            writer.commit().unwrap();
        });
    }

    /// Removes a single metric from the storage.
    ///
    /// ## Arguments
    ///
    /// * `lifetime` - the lifetime of the storage in which to look for the metric.
    /// * `storage_name` - the name of the storage to store/fetch data from.
    /// * `metric_key` - the metric key/name.
    pub fn remove_single_metric(&self, lifetime: Lifetime, storage_name: &str, metric_name: &str) {
        let final_key = Self::get_storage_key(storage_name, Some(metric_name));

        if lifetime == Lifetime::Application {
            let mut data = self.app_lifetime_data.write().unwrap();
            data.remove(&final_key);
            return;
        }

        self.write_with_store(lifetime, |mut writer, store| {
            store.delete(&mut writer, final_key.clone()).unwrap();

            writer.commit().unwrap();
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_panicks_if_fails_dir_creation() {
        assert!(Database::new("").is_err());
    }

    #[test]
    fn test_data_dir_rkv_inits() {
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();

        Database::new(&str_dir).unwrap();

        assert!(dir.path().exists());
    }

    #[test]
    fn test_ping_lifetime_metric_recorded() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir).unwrap();

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        );

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

        db.iter_store_from(Lifetime::Ping, test_storage, &mut snapshotter);
        assert_eq!(1, found_metrics, "We only expect 1 Lifetime.Ping metric.");
    }

    #[test]
    fn test_application_lifetime_metric_recorded() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir).unwrap();

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage1";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::Application,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        );

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

        db.iter_store_from(Lifetime::Application, test_storage, &mut snapshotter);
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
        let db = Database::new(&str_dir).unwrap();

        // Attempt to record a known value.
        let test_value = "test-value";
        let test_storage = "test-storage2";
        let test_metric_id = "telemetry_test.test_name";
        db.record_per_lifetime(
            Lifetime::User,
            test_storage,
            test_metric_id,
            &Metric::String(test_value.to_string()),
        );

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

        db.iter_store_from(Lifetime::User, test_storage, &mut snapshotter);
        assert_eq!(1, found_metrics, "We only expect 1 Lifetime.User metric.");
    }

    #[test]
    fn test_clear_ping_storage() {
        // Init the database in a temporary directory.
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();
        let db = Database::new(&str_dir).unwrap();

        // Attempt to record a known value for every single lifetime.
        let test_storage = "test-storage";
        db.record_per_lifetime(
            Lifetime::User,
            test_storage,
            "telemetry_test.test_name_user",
            &Metric::String("test-value-user".to_string()),
        );
        db.record_per_lifetime(
            Lifetime::Ping,
            test_storage,
            "telemetry_test.test_name_ping",
            &Metric::String("test-value-ping".to_string()),
        );
        db.record_per_lifetime(
            Lifetime::Application,
            test_storage,
            "telemetry_test.test_name_application",
            &Metric::String("test-value-application".to_string()),
        );

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

            db.iter_store_from(Lifetime::User, test_storage, &mut snapshotter);
            db.iter_store_from(Lifetime::Ping, test_storage, &mut snapshotter);
            db.iter_store_from(Lifetime::Application, test_storage, &mut snapshotter);

            assert_eq!(3, snapshot.len(), "We expect all lifetimes to be present.");
            assert!(snapshot.contains_key("telemetry_test.test_name_user"));
            assert!(snapshot.contains_key("telemetry_test.test_name_ping"));
            assert!(snapshot.contains_key("telemetry_test.test_name_application"));
        }

        // Clear the Ping lifetime.
        db.clear_ping_lifetime_storage(test_storage);

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

            db.iter_store_from(Lifetime::User, test_storage, &mut snapshotter);
            db.iter_store_from(Lifetime::Ping, test_storage, &mut snapshotter);
            db.iter_store_from(Lifetime::Application, test_storage, &mut snapshotter);

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
        let db = Database::new(&str_dir).unwrap();

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
                    &Metric::String(value.to_string()),
                );
            }
        }

        // Remove "telemetry_test.single_metric_delete" from each lifetime.
        for lifetime in lifetimes.iter() {
            db.remove_single_metric(
                *lifetime,
                test_storage,
                &format!("{}_delete", metric_id_pattern),
            );
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
            db.iter_store_from(*lifetime, test_storage, &mut snapshotter);
            assert_eq!(
                1, found_metrics,
                "We only expect 1 metric for this lifetime."
            );
        }
    }
}
