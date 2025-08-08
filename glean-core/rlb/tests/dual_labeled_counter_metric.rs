// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;

use glean::ConfigurationBuilder;
use glean_core::TestGetValue;

/// Some user metrics.
mod metrics {
    use glean::Lifetime;
    use glean_core::{CommonMetricData, DualLabeledCounterMetric};
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static dual_labeled_counter: Lazy<DualLabeledCounterMetric> = Lazy::new(|| {
        DualLabeledCounterMetric::new(
            CommonMetricData {
                name: "labeled_boolean".into(),
                category: "sample".into(),
                send_in_pings: vec!["validation".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                ..Default::default()
            },
            None,
            None,
        )
    });
}

mod pings {
    use super::*;
    use glean::private::PingType;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static validation: Lazy<PingType> = Lazy::new(|| {
        common::PingBuilder::new("validation")
            .with_send_if_empty(true)
            .build()
    });
}

#[test]
fn test_dual_labeled_metrics_test_get_value_functions_appropriately() {
    common::enable_test_logging();

    metrics::dual_labeled_counter
        .get("label1", "category1")
        .add(10);

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    _ = &*pings::validation;
    let cfg = ConfigurationBuilder::new(true, tmpname, "firefox-desktop")
        .with_server_endpoint("invalid-test-host")
        .build();
    common::initialize(cfg);

    let map = metrics::dual_labeled_counter.test_get_value(None).unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("label1").unwrap().get("category1").unwrap(), &10);

    pings::validation.submit(None);
    let map = metrics::dual_labeled_counter.test_get_value(None).unwrap();
    assert_eq!(map.len(), 0);
    assert!(!map.contains_key("label1"));

    glean::shutdown();
}
