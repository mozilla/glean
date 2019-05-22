// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;

#[test]
fn write_ping_to_disk() {
    let (glean, temp) = new_glean();

    // We need to store a metric as an empty ping is not stored.
    let counter = CounterMetric::new(CommonMetricData {
        name: "counter".into(),
        category: "local".into(),
        send_in_pings: vec!["metrics".into()],
        ..Default::default()
    });
    counter.add(&glean, 1);

    assert!(glean.send_ping("metrics", false).unwrap());

    let path = temp.path().join("pings");

    let mut count = 0;
    for entry in std::fs::read_dir(path).unwrap() {
        assert!(entry.unwrap().path().is_file());
        count += 1;
    }
    assert_eq!(1, count);
}
