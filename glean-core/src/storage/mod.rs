// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]

//! Storage snapshotting.

use std::collections::HashMap;

use serde_json::{json, Value as JsonValue};

use crate::database::Database;
use crate::metrics::Metric;
use crate::Lifetime;

/// Snapshot metrics from the underlying database.
pub struct StorageManager;

/// Labeled metrics are stored as `<metric name>/<label>`.
/// They need to go into a nested object in the final snapshot.
///
/// We therefore extract the metric name and the label from the key and construct the new object or
/// add to it.
fn snapshot_labeled_metrics(
    snapshot: &mut HashMap<String, HashMap<String, JsonValue>>,
    metric_name: &str,
    metric: &Metric,
) {
    let category = format!("labeled_{}", metric.category());
    let map = snapshot.entry(category).or_insert_with(HashMap::new);

    let mut s = metric_name.splitn(2, '/');
    let metric_name = s.next().unwrap(); // Safe unwrap, the function is only called when the name does contain a '/'
    let label = s.next().unwrap(); // Safe unwrap, the function is only called when the name does contain a '/'

    let obj = map.entry(metric_name.into()).or_insert_with(|| json!({}));
    let obj = obj.as_object_mut().unwrap(); // safe unwrap, we constructed the object above
    obj.insert(label.into(), metric.as_json());
}

impl StorageManager {
    /// Snapshot the given store and optionally clear it.
    ///
    /// ## Arguments
    ///
    /// * `storage` - the database to read from.
    /// * `store_name` - the store to snapshot.
    /// * `clear_store` - whether to clear the data after snapshotting.
    ///
    /// ## Return value
    ///
    /// Returns the stored data in a string encoded as JSON.
    /// Returns `None` if no data for the store exists.
    pub fn snapshot(
        &self,
        storage: &Database,
        store_name: &str,
        clear_store: bool,
    ) -> Option<String> {
        self.snapshot_as_json(storage, store_name, clear_store)
            .map(|data| ::serde_json::to_string_pretty(&data).unwrap())
    }

    /// Snapshot the given store and optionally clear it.
    ///
    /// ## Arguments
    ///
    /// * `storage` - the database to read from.
    /// * `store_name` - the store to snapshot.
    /// * `clear_store` - whether to clear the data after snapshotting.
    ///
    /// ## Return value
    ///
    /// Returns a JSON representation of the stored data.
    /// Returns `None` if no data for the store exists.
    pub fn snapshot_as_json(
        &self,
        storage: &Database,
        store_name: &str,
        clear_store: bool,
    ) -> Option<JsonValue> {
        let mut snapshot: HashMap<String, HashMap<String, JsonValue>> = HashMap::new();

        let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
            let metric_name = String::from_utf8_lossy(metric_name).into_owned();
            if metric_name.contains('/') {
                snapshot_labeled_metrics(&mut snapshot, &metric_name, &metric);
            } else {
                let map = snapshot
                    .entry(metric.category().into())
                    .or_insert_with(HashMap::new);
                map.insert(metric_name, metric.as_json());
            }
        };

        storage.iter_store_from(Lifetime::Ping, &store_name, &mut snapshotter);
        storage.iter_store_from(Lifetime::Application, &store_name, &mut snapshotter);
        storage.iter_store_from(Lifetime::User, &store_name, &mut snapshotter);

        if clear_store {
            storage.clear_ping_lifetime_storage(store_name);
        }

        if snapshot.is_empty() {
            None
        } else {
            Some(json!(snapshot))
        }
    }

    /// Get the current value of a single metric identified by name.
    ///
    /// This look for a value in stores for all lifetimes.
    ///
    /// ## Arguments:
    ///
    /// * `storage` - The database to get data from.
    /// * `store_name` - The store name to look into.
    /// * `metric_id` - The full metric identifier.
    ///
    /// ## Return value:
    ///
    /// Returns the decoded metric or `None` if no data is found.
    pub fn snapshot_metric(
        &self,
        storage: &Database,
        store_name: &str,
        metric_id: &str,
    ) -> Option<Metric> {
        let mut snapshot: Option<Metric> = None;

        let mut snapshotter = |metric_name: &[u8], metric: &Metric| {
            let metric_name = String::from_utf8_lossy(metric_name).into_owned();
            if metric_name == metric_id {
                snapshot = Some(metric.clone())
            }
        };

        storage.iter_store_from(Lifetime::Ping, &store_name, &mut snapshotter);
        storage.iter_store_from(Lifetime::Application, &store_name, &mut snapshotter);
        storage.iter_store_from(Lifetime::User, &store_name, &mut snapshotter);

        snapshot
    }
}
