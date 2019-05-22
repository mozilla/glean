// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::{CommonMetricData, Glean, storage::StorageManager};

#[test]
fn thread_safety() {
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    let (_t, tmpname) = tempdir();
    let glean = Glean::new(&tmpname, GLOBAL_APPLICATION_ID).unwrap();
    let glean = Arc::new(Mutex::new(glean));

    let threadsafe_metric = CounterMetric::new(CommonMetricData {
        name: "threadsafe".into(),
        category: "global".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        ..Default::default()
    });
    let threadsafe_metric = Arc::new(threadsafe_metric);

    let barrier = Arc::new(Barrier::new(2));
    let c = barrier.clone();
    let threadsafe_metric_clone = threadsafe_metric.clone();
    let glean_clone = glean.clone();
    let child = thread::spawn(move || {
        threadsafe_metric_clone.add(&*glean_clone.lock().unwrap(), 1);
        c.wait();
        threadsafe_metric_clone.add(&*glean_clone.lock().unwrap(), 1);
    });

    threadsafe_metric.add(&*glean.lock().unwrap(), 1);
    barrier.wait();
    threadsafe_metric.add(&*glean.lock().unwrap(), 1);

    child.join().unwrap();

    let snapshot = StorageManager
        .snapshot_as_json(glean.lock().unwrap().storage(), "core", true)
        .unwrap();
    assert_eq!(json!({"counter": { "global.threadsafe": 4 }}), snapshot);
}
