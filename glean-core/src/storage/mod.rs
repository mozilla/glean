#![allow(non_upper_case_globals)]

use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use crate::metrics::Metric;
use crate::Glean;
use crate::Lifetime;

mod generic;

pub use generic::GenericStorage;

pub struct StorageManager;

impl StorageManager {
    pub fn snapshot(&self, store_name: &str, clear_store: bool) -> String {
        let data = self.snapshot_as_json(store_name, clear_store);
        ::serde_json::to_string_pretty(&data).unwrap()
    }

    pub fn snapshot_as_json(&self, store_name: &str, clear_store: bool) -> JsonValue {
        let mut snapshot: HashMap<&str, HashMap<String, JsonValue>> = HashMap::new();

        let store_iter = format!("{}#", store_name);
        let len = store_iter.len();

        let mut snapshotter = |reader: rkv::Reader, store: rkv::SingleStore| {
            let mut iter = store.iter_from(&reader, &store_iter).unwrap();
            while let Some(Ok((metric_name, value))) = iter.next() {
                if !metric_name.starts_with(store_iter.as_bytes()) {
                    break;
                }

                let metric_name = &metric_name[len..];
                let metric: Metric = match value.unwrap() {
                    rkv::Value::Blob(blob) => bincode::deserialize(blob).unwrap(),
                    _ => continue,
                };

                let map = snapshot
                    .entry(metric.category())
                    .or_insert_with(HashMap::new);
                let metric_name = String::from_utf8_lossy(metric_name).into_owned();
                map.insert(metric_name, metric.as_json());
            }
        };

        Glean::singleton().read_with_store(Lifetime::Ping.as_str(), &mut snapshotter);
        Glean::singleton().read_with_store(Lifetime::Application.as_str(), &mut snapshotter);
        Glean::singleton().read_with_store(Lifetime::User.as_str(), &mut snapshotter);

        if clear_store {
            Glean::singleton().write_with_store(Lifetime::Ping.as_str(), |mut writer, store| {
                let mut metrics = Vec::new();
                {
                    let mut iter = store.iter_from(&writer, &store_iter).unwrap();
                    while let Some(Ok((metric_name, _))) = iter.next() {
                        if let Ok(metric_name) = std::str::from_utf8(metric_name) {
                            if !metric_name.starts_with(&store_iter) {
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

        json!(snapshot)
    }
}
