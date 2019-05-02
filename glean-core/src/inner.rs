use std::fs;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;

use rkv::{Rkv, SingleStore, StoreOptions};

use crate::Lifetime;
use crate::metrics::Metric;

#[derive(Debug)]
pub struct Inner {
    initialized: bool,
    upload_enabled: bool,
    rkv: Option<Rkv>,
    app_data: BTreeMap<String, Metric>,
}

impl Inner {
    pub fn new() -> Self {
        log::info!("Creating new Inner glean");

        Self {
            initialized: false,
            upload_enabled: true,
            rkv: None,
            app_data: BTreeMap::new(),
        }
    }

    pub fn initialize(&mut self, data_path: &str) {
        self.rkv = Some(self.open_rkv(data_path));
        self.initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn set_upload_enabled(&mut self, flag: bool) {
        self.upload_enabled = flag;
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.upload_enabled
    }


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
        F: FnMut(&[u8], &Metric)
    {
        let len = iter_start.len();

        // Lifetime::Application data is not persisted to disk
        if lifetime == Lifetime::Application {
            for (key, value) in &self.app_data {
                if key.starts_with(iter_start) {
                    let key = &key[len..];
                    transaction_fn(key.as_bytes(), value);
                }

            }

            return;
        }

        let rkv = self.rkv.as_ref().unwrap();
        let store: SingleStore = rkv.open_single(lifetime.as_str(), StoreOptions::create()).unwrap();
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
        let store: SingleStore = rkv.open_single(store_name.as_str(), StoreOptions::create()).unwrap();
        let writer = rkv.write().unwrap();
        transaction_fn(writer, store);
    }

    pub fn record(&self, lifetime: Lifetime, ping_name: &str, key: &str, metric: &Metric) {
        let encoded = bincode::serialize(&metric).unwrap();
        let value = rkv::Value::Blob(&encoded);

        let final_key = format!("{}#{}", ping_name, key);
        let store_name = lifetime.as_str();
        let rkv = self.rkv.as_ref().unwrap();
        let store = rkv.open_single(store_name, StoreOptions::create()).unwrap();

        let mut writer = rkv.write().unwrap();
        store.put(&mut writer, final_key, &value).unwrap();
        let _ = writer.commit();
    }

    pub fn record_with<F>(&mut self, lifetime: Lifetime, ping_name: &str, key: &str, transform: F)
    where
        F: Fn(Option<Metric>) -> Metric,
    {
        let final_key = format!("{}#{}", ping_name, key);

        if lifetime == Lifetime::Application {
            let entry = self.app_data.entry(final_key);
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
