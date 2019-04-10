#![allow(non_upper_case_globals)]

use std::sync::RwLock;

use lazy_static::lazy_static;
use serde_json::{json, Value as JsonValue};

mod boolean;
mod string;

use boolean::BooleanStorageImpl;
use string::StringStorageImpl;

lazy_static! {
    pub static ref BooleanStorage: RwLock<BooleanStorageImpl> =
        RwLock::new(BooleanStorageImpl::new());
    pub static ref StringStorage: RwLock<StringStorageImpl> = RwLock::new(StringStorageImpl::new());
}

pub trait StorageDump {
    fn dump(&self, store_name: &str) -> Option<JsonValue>;
}

pub struct StorageManager;

macro_rules! dump_storages {
    ($store_name:expr => $(( $name:expr, $storage:tt),)+) => {{
        let data = json!({
            $(
                $name: $storage.read().unwrap().dump($store_name),
            )+
        });
        data
    }}
}

impl StorageManager {
    pub fn dump(&self, store_name: &str) -> String {
        let data = dump_storages!(store_name => ("bool", BooleanStorage), ("string", StringStorage),);

        ::serde_json::to_string_pretty(&data).unwrap()
    }
}
