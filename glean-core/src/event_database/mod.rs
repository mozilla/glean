// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::fs;
use std::fs::{create_dir_all, OpenOptions};
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use serde_json::{json, Map as JsonMap, Value as JsonValue};

use crate::CommonMetricData;
use crate::Glean;
use crate::Result;

const TIMESTAMP_FIELD: &str = "timestamp";
const CATEGORY_FIELD: &str = "category";
const NAME_FIELD: &str = "name";
const EXTRA_FIELD: &str = "extra";

#[derive(Debug, Clone)]
pub struct RecordedEventData {
    pub timestamp: u64,
    pub category: String,
    pub name: String,
    pub extra: Option<HashMap<String, String>>,
}

impl RecordedEventData {
    /// Serialize an event to JSON
    pub fn serialize(&self) -> JsonValue {
        RecordedEventData::serialize_parts(self.timestamp, &self.category, &self.name, &self.extra)
    }

    /// Serialize an event to JSON, adjusting its timestamp relative to a base timestamp
    pub fn serialize_relative(&self, timestamp_offset: u64) -> JsonValue {
        RecordedEventData::serialize_parts(
            self.timestamp - timestamp_offset,
            &self.category,
            &self.name,
            &self.extra,
        )
    }

    /// Internal function to perform the serialization to JSON
    fn serialize_parts(
        timestamp: u64,
        category: &str,
        name: &str,
        extra: &Option<HashMap<String, String>>,
    ) -> JsonValue {
        let mut result = JsonMap::new();
        result.insert(TIMESTAMP_FIELD.to_string(), json!(timestamp));
        result.insert(CATEGORY_FIELD.to_string(), json!(category));
        result.insert(NAME_FIELD.to_string(), json!(name));
        if let Some(extra) = extra {
            result.insert(
                EXTRA_FIELD.to_string(),
                JsonValue::Object(
                    extra
                        .iter()
                        .map(|(k, v)| (k.clone(), json!(v)))
                        .collect::<JsonMap<String, JsonValue>>(),
                ),
            );
        }
        JsonValue::Object(result)
    }

    /// Deserialize an event from JSON
    pub fn deserialize(value: &JsonValue) -> Option<RecordedEventData> {
        let extra = value
            .get(EXTRA_FIELD.to_string())
            .and_then(|extra| extra.as_object())
            .and_then(|extra| {
                Some(
                    extra
                        .iter()
                        .filter(|(_, v)| v.as_str().is_some())
                        .map(|(k, v)| (k.to_string(), v.as_str().unwrap().to_string()))
                        .collect(),
                )
            });
        Some(RecordedEventData {
            timestamp: value.get(TIMESTAMP_FIELD.to_string())?.as_u64()?,
            category: value.get(CATEGORY_FIELD.to_string())?.as_str()?.to_string(),
            name: value.get(NAME_FIELD.to_string())?.as_str()?.to_string(),
            extra,
        })
    }
}

/// This struct handles the in-memory and on-disk storage logic for events.
///
/// So that the data survives shutting down of the application, events are stored
/// in an append-only file on disk, in addition to the store in memory. Each line
/// of this file records a single event in JSON, exactly as it will be sent in the
/// ping.  There is one file per store.
///
/// When restarting the application, these on-disk files are checked, and if any are
/// found, they are loaded, queued for sending and flushed immediately before any
/// further events are collected.  This is because the timestamps for these events
/// may have come from a previous boot of the device, and therefore will not be
/// compatible with any newly-collected events.
#[derive(Debug)]
pub struct EventDatabase {
    pub path: PathBuf,
    // The in-memory list of events
    event_stores: RwLock<HashMap<String, Vec<RecordedEventData>>>,
}

impl EventDatabase {
    /// Create a new event database.
    ///
    /// # Arguments
    ///
    /// - `data_path`: The directory to store events in. A new directory
    ///   `events` will be created inside of this directory.
    pub fn new(data_path: &str) -> Result<Self> {
        let path = Path::new(data_path).join("events");
        create_dir_all(&path)?;

        Ok(Self {
            path,
            event_stores: RwLock::new(HashMap::new()),
        })
    }

    /// Initialize events storage. This must be called once on application startup,
    /// e.g. from [Glean.initialize], but after we are ready to send pings, since
    /// this could potentially collect and send pings.
    ///
    /// If there are any events queued on disk, it loads them into memory so
    /// that the memory and disk representations are in sync.
    ///
    /// Secondly, if this is the first time the application has been run since
    /// rebooting, any pings containing events are assembled into pings and cleared
    /// immediately, since their timestamps won't be compatible with the timestamps
    /// we would create during this boot of the device.
    ///
    /// # Arguments
    ///
    /// * `glean`:  The Glean instance.
    pub fn on_ready_to_send_pings(&self, glean: &Glean) {
        match self.load_events_from_disk() {
            Ok(_) => self.send_all_events(glean),
            Err(err) => log::error!("Error loading pings from disk: {}", err),
        }
    }

    fn load_events_from_disk(&self) -> Result<()> {
        let mut db = self.event_stores.write().unwrap();
        for entry in fs::read_dir(&self.path)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                let store_name = entry.file_name().into_string()?;
                let file = BufReader::new(OpenOptions::new().read(true).open(entry.path())?);
                db.insert(
                    store_name,
                    file.lines()
                        .filter_map(|line| line.ok())
                        .filter_map(|line| {
                            println!("{}", line);
                            serde_json::from_str::<JsonValue>(&line).ok()
                        })
                        .filter_map(|json| RecordedEventData::deserialize(&json))
                        .collect(),
                );
            }
        }
        Ok(())
    }

    fn send_all_events(&self, glean: &Glean) {
        let store_names = {
            let db = self.event_stores.read().unwrap();
            db.keys().cloned().collect::<Vec<String>>()
        };

        for store_name in store_names {
            if let Err(err) = glean.send_ping_by_name(&store_name, false) {
                log::error!(
                    "Error flushing existing events to the {} ping: {}",
                    store_name,
                    err
                );
            }
        }
    }

    /// Record an event in the desired stores.
    ///
    /// # Arguments
    ///
    /// - `glean`: The Glean instance.
    /// - `meta`: The metadata about the event metric. Used to get the category,
    ///   name and stores for the metric.
    /// - `timestamp`: The timestamp of the event, in nanoseconds. Must use a
    ///   monotonically increasing timer (this value is obtained on the
    ///   platform-specific side).
    /// - `extra`: Extra data values, mapping strings to strings.
    pub fn record(
        &self,
        glean: &Glean,
        meta: &CommonMetricData,
        timestamp: u64,
        extra: Option<HashMap<String, String>>,
    ) {
        let event = RecordedEventData {
            timestamp,
            category: meta.category.to_string(),
            name: meta.name.to_string(),
            extra,
        };
        let event_json = event.serialize().to_string();
        let mut stores_to_send: Vec<&str> = Vec::new();

        {
            let mut db = self.event_stores.write().unwrap();
            for store_name in meta.send_in_pings.iter() {
                let store = db
                    .entry(store_name.to_string())
                    .or_insert_with(|| Vec::new());
                store.push(event.clone());
                self.write_event_to_disk(store_name, &event_json);
                if store.len() == glean.get_max_events() {
                    stores_to_send.push(&store_name);
                }
            }
        }

        for store_name in stores_to_send {
            if let Err(err) = glean.send_ping_by_name(store_name, false) {
                log::error!(
                    "Got more than {} events, but could not send {} ping: {}",
                    glean.get_max_events(),
                    store_name,
                    err
                );
            }
        }
    }

    /// Writes an event to a single store on disk.
    ///
    /// This assumes that the write lock in `self.event_stores` is already held.
    ///
    /// # Arguments
    ///
    /// - `store_name`: The name of the store.
    /// - `event_json`: The event content, as a single-line JSON-encoded string.
    fn write_event_to_disk(&self, store_name: &str, event_json: &str) {
        if let Err(err) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path.join(store_name))
            .and_then(|mut file| writeln!(file, "{}", event_json))
        {
            log::error!("Error writing event to store {}: {}", store_name, err);
        }
    }

    /// Get a snapshot of the stored event data as a JsonValue.
    ///
    /// # Arguments
    ///
    /// - `store_name` The name of the desired store.
    /// - `clear_store` Whether to clear the store afterward.
    ///
    /// # Returns
    ///
    /// The JsonValue as an array of events, if any.
    pub fn snapshot_as_json(&self, store_name: &str, clear_store: bool) -> Option<JsonValue> {
        let result = {
            let db = self.event_stores.read().unwrap();
            db.get(&store_name.to_string()).and_then(|store| {
                if !store.is_empty() {
                    let first_timestamp = store[0].timestamp;
                    Some(json!(store
                        .iter()
                        .map(|e| e.serialize_relative(first_timestamp))
                        .collect::<Vec<JsonValue>>()))
                } else {
                    None
                }
            })
        };

        if clear_store {
            let mut db = self.event_stores.write().unwrap();
            db.remove(&store_name.to_string());

            if let Err(err) = fs::remove_file(self.path.join(store_name)) {
                log::error!("Error removing events queue file {}: {}", store_name, err);
            }
        }

        result
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Return whether there are any events currently stored for the given even
    /// metric.
    ///
    /// This doesn't clear the stored value.
    pub fn test_has_value<'a>(&'a self, meta: &'a CommonMetricData, store_name: &str) -> bool {
        self.event_stores
            .read()
            .unwrap()
            .get(&store_name.to_string())
            .into_iter()
            .flatten()
            .any(|event| event.name == meta.name && event.category == meta.category)
    }

    /// **Test-only API (exported for FFI purposes).**
    ///
    /// Get the vector of currently stored events for the given event metric in
    /// the given store.
    ///
    /// This doesn't clear the stored value.
    pub fn test_get_value<'a>(
        &'a self,
        meta: &'a CommonMetricData,
        store_name: &str,
    ) -> Option<Vec<RecordedEventData>> {
        let value: Vec<RecordedEventData> = self
            .event_stores
            .read()
            .unwrap()
            .get(&store_name.to_string())
            .into_iter()
            .flatten()
            .filter(|event| event.name == meta.name && event.category == meta.category)
            .cloned()
            .collect();
        if !value.is_empty() {
            Some(value)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn handle_truncated_events_on_disk() {
        let t = tempfile::tempdir().unwrap();

        {
            let db = EventDatabase::new(&t.path().display().to_string()).unwrap();
            db.write_event_to_disk("events", "{\"timestamp\": 500");
            db.write_event_to_disk("events", "{\"timestamp\"");
            db.write_event_to_disk(
                "events",
                "{\"timestamp\": 501, \"category\": \"ui\", \"name\": \"click\"}",
            );
        }

        {
            let db = EventDatabase::new(&t.path().display().to_string()).unwrap();
            db.load_events_from_disk().unwrap();
            let events = &db.event_stores.read().unwrap()["events"];
            assert_eq!(1, events.len());
        }
    }
}
