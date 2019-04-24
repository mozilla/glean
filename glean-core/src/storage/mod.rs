#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use crate::Glean;
use crate::Lifetime;

mod generic;

pub use generic::GenericStorage;

pub trait StorageDump {
    fn snapshot(&mut self, store_name: &str, clear_store: bool) -> Option<JsonValue>;
}

pub struct StorageManager;

impl StorageManager {
    pub fn snapshot(&self, store_name: &str, clear_store: bool) -> String {
        let metric_types = ["bool", "counter", "string"];
        let mut snapshot : HashMap<&str, HashMap<String, JsonValue>> = HashMap::with_capacity(metric_types.len());

        for typ in &metric_types {
            let store_iter = format!("{}#{}#", typ, store_name);
            let len = store_iter.len();
            let mut map = HashMap::new();

            let mut snapshotter = |reader: &rkv::Reader, store: rkv::SingleStore| {
                let mut iter = store.iter_from(reader, &store_iter).unwrap();
                while let Some(Ok((metric_name, value))) = iter.next() {
                    if metric_name.len() < len || !metric_name.starts_with(store_iter.as_bytes()) {
                        break;
                    }

                    let metric_name = &metric_name[len..];
                    let data = match value.unwrap() {
                        rkv::Value::Str(s) => json!(s),
                        rkv::Value::U64(s) => json!(s),
                        rkv::Value::I64(s) => json!(s),
                        rkv::Value::F64(s) => json!(s.into_inner()),
                        rkv::Value::Bool(s) => json!(s),
                        rkv::Value::Json(s) => json!(s),
                        rkv::Value::Blob(s) => json!(s),
                        rkv::Value::Instant(_s) => unimplemented!(),
                        rkv::Value::Uuid(_s) => unimplemented!(),
                    };
                    let metric_name = String::from_utf8_lossy(metric_name).into_owned();
                    map.insert(metric_name, data);
                }
            };

            Glean::singleton().read_with_store(Lifetime::Ping.as_str(), &mut snapshotter);
            Glean::singleton().read_with_store(Lifetime::Application.as_str(), &mut snapshotter);
            Glean::singleton().read_with_store(Lifetime::User.as_str(), &mut snapshotter);

            snapshot.insert(typ, map);
        }

        ::serde_json::to_string_pretty(&snapshot).unwrap()
    }
}
