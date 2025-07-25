// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;

use glean::ConfigurationBuilder;
use glean_core::TestGetValue;

/// Some user metrics.
mod metrics {
    use glean::Lifetime;
    use glean_core::{CommonMetricData, LabeledBoolean, LabeledMetricData};
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static labeled_boolean: Lazy<LabeledBoolean> = Lazy::new(|| {
        LabeledBoolean::new(
            LabeledMetricData::Common {
                cmd: CommonMetricData {
                    name: "labeled_boolean".into(),
                    category: "sample".into(),
                    send_in_pings: vec!["validation".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    ..Default::default()
                },
            },
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
fn test_labeled_metrics_test_get_value_functions_appropriately() {
    common::enable_test_logging();

    metrics::labeled_boolean.get("label1").set(true);

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    _ = &*pings::validation;
    let cfg = ConfigurationBuilder::new(true, tmpname, "firefox-desktop")
        .with_server_endpoint("invalid-test-host")
        .build();
    common::initialize(cfg);

    let map = metrics::labeled_boolean.test_get_value(None).unwrap();
    assert_eq!(map.len(), 1);
    assert_eq!(map.get("sample.labeled_boolean/label1").unwrap(), &true);

    pings::validation.submit(None);
    let map = metrics::labeled_boolean.test_get_value(None).unwrap();
    assert_eq!(map.len(), 0);
    assert!(!map.contains_key("sample.labeled_boolean/label1"));

    glean::shutdown();
}
