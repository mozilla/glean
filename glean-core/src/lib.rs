use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::fs;
use lazy_static::lazy_static;
use rkv::{Rkv, SingleStore, StoreOptions};
use tempfile::Builder;

mod common_metric_data;
mod internal_metrics;
pub mod metrics;
pub mod storage;

pub use common_metric_data::{CommonMetricData, Lifetime};
use metrics::Metric;

lazy_static! {
    static ref GLEAN_SINGLETON: Glean = Glean::new();
}

#[derive(Debug)]
pub struct Glean {
    inner: RwLock<Inner>,
}

impl Glean {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(Inner::new()),
        }
    }

    pub fn singleton() -> &'static Self {
        &*GLEAN_SINGLETON
    }

    pub fn initialize(&self) {
        {
            let mut inner = self.inner.write().unwrap();
            inner.initialized = true;

            // drop lock before we call any metric setters
        }

        internal_metrics::clientId.set("glean.rs-sample")
    }

    fn read(&self) -> RwLockReadGuard<Inner> {
        self.inner.read().unwrap()
    }

    fn write(&self) -> RwLockWriteGuard<Inner> {
        self.inner.write().unwrap()
    }

    pub fn is_initialized(&self) -> bool {
        self.read().initialized
    }

    pub fn set_upload_enabled(&self, flag: bool) {
        self.write().upload_enabled = flag;
    }

    pub fn is_upload_enabled(&self) -> bool {
        self.read().upload_enabled
    }

    pub fn read_with_store<F>(&self, store_name: &str, mut transaction_fn: F) where F: FnMut(rkv::Reader, SingleStore) {
        let inner = self.write();
        let store: SingleStore = inner.rkv.open_single(store_name, StoreOptions::create()).unwrap();
        let reader = inner.rkv.read().unwrap();
        transaction_fn(reader, store);
    }

    pub fn write_with_store<F>(&self, store_name: &str, mut transaction_fn: F) where F: FnMut(rkv::Writer, SingleStore) {
        let inner = self.write();
        let store: SingleStore = inner.rkv.open_single(store_name, StoreOptions::create()).unwrap();
        let writer = inner.rkv.write().unwrap();
        transaction_fn(writer, store);
    }

    pub fn record(&self, lifetime: Lifetime, ping_name: &str, key: &str, value: &rkv::Value) {
        let inner = self.write();
        let final_key = format!("{}#{}", ping_name, key);
        let store_name = lifetime.as_str();
        let store = inner.rkv.open_single(store_name, StoreOptions::create()).unwrap();

        let mut writer = inner.rkv.write().unwrap();
        store.put(&mut writer, final_key, value).unwrap();
        let _ = writer.commit();
    }

    pub fn record_with<F>(&self, lifetime: Lifetime, ping_name: &str, key: &str, transform: F) where F: Fn(Option<Metric>) -> Metric {
        let inner = self.write();
        let final_key = format!("{}#{}", ping_name, key);
        let store_name = lifetime.as_str();
        let store = inner.rkv.open_single(store_name, StoreOptions::create()).unwrap();

        let mut writer = inner.rkv.write().unwrap();
        let new_value : Metric = {
            let old_value = store.get(&writer, &final_key).unwrap();

            match old_value {
                Some(rkv::Value::Blob(blob)) => {
                    let old_value = bincode::deserialize(blob).ok();
                    transform(old_value)
                }
                _ => transform(None)
            }
        };

        let encoded = bincode::serialize(&new_value).unwrap();
        let value = rkv::Value::Blob(&encoded);
        store.put(&mut writer, final_key, &value).unwrap();
        let _ = writer.commit();
    }
}

#[derive(Debug)]
struct Inner {
    initialized: bool,
    upload_enabled: bool,
    rkv: Rkv,
}

impl Inner {
    fn new() -> Self {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        fs::create_dir_all(root.path()).unwrap();
        let path = root.path();
        let rkv = Rkv::new(path).unwrap();

        Self {
            initialized: true,
            upload_enabled: true,
            rkv: rkv,
        }
    }
}
