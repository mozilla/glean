// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;

#[test]
fn write_ping_to_disk() {
    let (mut glean, _temp) = new_glean();

    let ping = PingType::new("metrics", true);
    glean.register_ping_type(&ping);

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["metrics".into()],
        ..Default::default()
    });
    counter.add(&glean, 1);

    assert!(ping.send(&glean).unwrap());

    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());
}

#[test]
fn disabling_upload_clears_pending_pings() {
    let (mut glean, _) = new_glean();

    let ping = PingType::new("metrics", true);
    glean.register_ping_type(&ping);

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["metrics".into()],
        ..Default::default()
    });

    counter.add(&glean, 1);
    assert!(ping.send(&glean).unwrap());
    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());

    glean.set_upload_enabled(false);
    assert_eq!(0, get_queued_pings(glean.get_data_path()).unwrap().len());

    glean.set_upload_enabled(true);
    assert_eq!(0, get_queued_pings(glean.get_data_path()).unwrap().len());

    counter.add(&glean, 1);
    assert!(ping.send(&glean).unwrap());
    assert_eq!(1, get_queued_pings(glean.get_data_path()).unwrap().len());
}
