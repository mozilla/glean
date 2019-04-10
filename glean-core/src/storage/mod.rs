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
    fn snapshot(&mut self, store_name: &str, clear_store: bool) -> Option<JsonValue>;
}

pub struct StorageManager;

macro_rules! dump_storages {
    ($store_name:expr, $clear:expr => $(( $name:expr, $storage:tt),)+) => {{
        let data = json!({
            $(
                $name: $storage.write().unwrap().snapshot($store_name, $clear),
            )+
        });
        data
    }}
}

impl StorageManager {
    pub fn snapshot(&self, store_name: &str, clear_store: bool) -> String {
        let data = dump_storages!(store_name, clear_store => ("bool", BooleanStorage), ("string", StringStorage),);

        ::serde_json::to_string_pretty(&data).unwrap()
    }
}
