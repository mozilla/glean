#![allow(non_upper_case_globals)]

use std::sync::RwLock;
use std::collections::HashMap;

use lazy_static::lazy_static;
use serde_json::{json, Value as JsonValue};

pub struct CommonMetricData {
    pub name: String,
}

pub struct BooleanMetric {
    meta: CommonMetricData,
}

pub struct StringMetric {
    meta: CommonMetricData,
}

pub trait StorageDump {
    fn dump(&self) -> JsonValue;
}

impl BooleanMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set(&self, value: bool) {
        let mut lock = BooleanStorage.write().unwrap();
        lock.record(&self.meta, value)
    }
}

impl StringMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }

    pub fn set<S: Into<String>>(&self, value: S) {
        let mut lock = StringStorage.write().unwrap();
        lock.record(&self.meta, value.into())
    }
}

lazy_static! {
    pub static ref BooleanStorage : RwLock<BooleanStorageImpl> = RwLock::new(BooleanStorageImpl::new());
    pub static ref StringStorage : RwLock<StringStorageImpl> = RwLock::new(StringStorageImpl::new());
}
pub struct BooleanStorageImpl {
    store: HashMap<String, bool>,
}

impl BooleanStorageImpl {
    fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    fn record(&mut self, data: &CommonMetricData, value: bool) {
        let name = data.name.clone();
        self.store.insert(name, value);
    }
}

impl StorageDump for BooleanStorageImpl {
    fn dump(&self) -> JsonValue {
        json!(self.store)
    }
}

impl StorageDump for StringStorageImpl {
    fn dump(&self) -> JsonValue {
        json!(self.store)
    }
}

pub struct StringStorageImpl {
    store: HashMap<String, String>,
}

impl StringStorageImpl {
    fn new() -> Self {
        Self {
            store: HashMap::new()
        }
    }

    fn record(&mut self, data: &CommonMetricData, value: String) {
        let name = data.name.clone();
        self.store.insert(name, value);
    }

    fn dump(&self) -> JsonValue {
        json!(self.store)
    }
}


pub struct StorageManager;

macro_rules! dump_storages {
    ($(($name:expr, $storage:tt),)+) => {{
        let data = json!({
            $(
                $name: $storage.read().unwrap().dump(),
            )+
        });
        data
    }}
}

impl StorageManager {
    pub fn dump(&self) -> String {
        let data = dump_storages!(
            ("bool", BooleanStorage),
            ("string", StringStorage),
        );

        ::serde_json::to_string_pretty(&data).unwrap()
    }
}

pub mod flags {
    use super::{CommonMetricData, BooleanMetric};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref a11yEnabled : BooleanMetric = BooleanMetric::new(CommonMetricData { name: "flags.a11yEnabled".into() });
    }
}

pub mod app {
    use super::{CommonMetricData, StringMetric};
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref clientId : StringMetric = StringMetric::new(CommonMetricData { name: "app.clientId".into() });
    }
}
