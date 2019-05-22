// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use serde_json::json;

use glean_core::metrics::*;
use glean_core::CommonMetricData;

#[test]
fn list() {
    let (mut glean, _t) = new_glean();

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    list.add(&glean, "first");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));

    list.add(&glean, "second");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));
    assert!(snapshot.contains(r#""second""#));

    list.set(&glean, vec!["third".into()]);
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(!snapshot.contains(r#""first""#));
    assert!(!snapshot.contains(r#""second""#));
    assert!(snapshot.contains(r#""third""#));
}
