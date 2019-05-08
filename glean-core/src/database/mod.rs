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

#[derive(Debug)]
pub struct Database {
    rkv: Option<Rkv>,
    // Metrics with 'application' lifetime only live as long
    // as the application lives: they don't need to be persisted
    // to disk using rkv. Store them in a map.
    app_lifetime_data: RwLock<BTreeMap<String, Metric>>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            rkv: None,
            app_lifetime_data: RwLock::new(BTreeMap::new()),
        }
    }

    /// Initialize the data store.
    ///
    /// This opens the underlying rkv store and creates
    /// the underlying directory structure.
    pub fn initialize(&mut self, data_path: &str) {
        self.rkv = Some(self.open_rkv(data_path));
    }

    /// Creates the storage directories and inits rkv.
    fn open_rkv(&mut self, path: &str) -> Rkv {
        let path = std::path::Path::new(path);
        log::info!("Path is: {:?}", path.display());
        if let Err(e) = fs::create_dir_all(&path) {
            log::info!(
                "Failed to create data dir. LETS CRASH!!!1! (error: {:?})",
                e
            );
            panic!("WAAAAAH!!!1!");
        }
        log::info!("path created. creating rkv.");
        let rkv = match Rkv::new(path) {
            Ok(rkv) => rkv,
            Err(e) => {
                log::info!("Failed to create rkv. LETS CRASH!!!1! (error: {:?})", e);
                panic!("WAAAAAH!!!1!");
            }
        };
        log::info!("Rkv done. We are initialized!");
        rkv
    }

    pub fn iter_store_from<F>(&self, lifetime: Lifetime, iter_start: &str, mut transaction_fn: F)
    where
        F: FnMut(&[u8], &Metric),
    {
        let len = iter_start.len();

        // Lifetime::Application data is not persisted to disk
        if lifetime == Lifetime::Application {
            let data = self.app_lifetime_data.read().unwrap();
            for (key, value) in data.iter() {
                if key.starts_with(iter_start) {
                    let key = &key[len..];
                    transaction_fn(key.as_bytes(), value);
                }
            }

            return;
        }

        let rkv = self.rkv.as_ref().unwrap();
        let store: SingleStore = rkv
            .open_single(lifetime.as_str(), StoreOptions::create())
            .unwrap();
        let reader = rkv.read().unwrap();
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

        let rkv = self.rkv.as_ref().unwrap();
        let store: SingleStore = rkv
            .open_single(store_name.as_str(), StoreOptions::create())
            .unwrap();
        let writer = rkv.write().unwrap();
        transaction_fn(writer, store);
    }

    /// Records a metric in the underlying storage system.
    pub fn record(&self, data: &CommonMetricData, value: &Metric) {
        let name = data.identifier();

        for ping_name in data.storage_names() {
            self.record_per_lifetime(data.lifetime, ping_name, &name, value);
        }
    }

    fn record_per_lifetime(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        metric: &Metric,
    ) {
        let encoded = bincode::serialize(&metric).unwrap();
        let value = rkv::Value::Blob(&encoded);

        let final_key = format!("{}#{}", storage_name, key);
        let store_name = lifetime.as_str();
        let rkv = self.rkv.as_ref().unwrap();
        let store = rkv.open_single(store_name, StoreOptions::create()).unwrap();

        let mut writer = rkv.write().unwrap();
        store.put(&mut writer, final_key, &value).unwrap();
        let _ = writer.commit();
    }

    /// Records the provided value, with the given lifetime, after
    /// applying a transformation function.
    pub fn record_with<F>(&self, data: &CommonMetricData, transform: F)
    where
        F: Fn(Option<Metric>) -> Metric,
    {
        let name = data.identifier();
        for ping_name in data.storage_names() {
            self.record_per_lifetime_with(data.lifetime, ping_name, &name, &transform);
        }
    }

    pub fn record_per_lifetime_with<F>(
        &self,
        lifetime: Lifetime,
        storage_name: &str,
        key: &str,
        transform: F,
    ) where
        F: Fn(Option<Metric>) -> Metric,
    {
        let final_key = format!("{}#{}", storage_name, key);

        if lifetime == Lifetime::Application {
            let mut data = self.app_lifetime_data.write().unwrap();
            let entry = data.entry(final_key);
            match entry {
                Entry::Vacant(entry) => {
                    entry.insert(transform(None));
                }
                Entry::Occupied(mut entry) => {
                    let old_value = entry.get();
                    entry.insert(transform(Some(old_value.clone())));
                }
            }
            return;
        }

        let store_name = lifetime.as_str();
        let rkv = self.rkv.as_ref().unwrap();
        let store = rkv.open_single(store_name, StoreOptions::create()).unwrap();

        let mut writer = rkv.write().unwrap();
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
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    #[should_panic]
    fn test_panicks_if_fails_dir_creation() {
        let mut db = Database::new();
        db.initialize("")
    }

    #[test]
    fn test_data_dir_rkv_inits() {
        let dir = tempdir().unwrap();
        let str_dir = dir.path().display().to_string();

        let mut db = Database::new();
        db.initialize(&str_dir);

        assert!(dir.path().exists());
        assert!(db.rkv.is_some());
    }
}
