// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;

#[test]
fn uuid_is_generated_and_stored() {
    let (mut glean, _t) = new_glean();

    let uuid: UuidMetric = UuidMetric::new(CommonMetricData {
        name: "uuid".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    uuid.generate(&glean);
    let snapshot = glean.snapshot("core", false);
    assert!(
        snapshot.contains(r#""local.uuid": ""#),
        format!("Snapshot 1: {}", snapshot)
    );

    uuid.generate(&glean);
    let snapshot = glean.snapshot("core", false);
    assert!(
        snapshot.contains(r#""local.uuid": ""#),
        format!("Snapshot 2: {}", snapshot)
    );
}
