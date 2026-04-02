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

    #[allow(non_upper_case_globals)]
    pub static labeled_boolean_static_labels: Lazy<LabeledBoolean> = Lazy::new(|| {
        LabeledBoolean::new(
            LabeledMetricData::Common {
                cmd: CommonMetricData {
                    name: "labeled_boolean_static_labels".into(),
                    category: "sample".into(),
                    send_in_pings: vec!["validation".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    ..Default::default()
                },
            },
            Some(vec!["label_1".into(), "label_2".into()]),
        )
    });

    #[allow(non_upper_case_globals)]
    pub static labeled_boolean_static_labels_clone: Lazy<LabeledBoolean> = Lazy::new(|| {
        LabeledBoolean::new(
            LabeledMetricData::Common {
                cmd: CommonMetricData {
                    name: "labeled_boolean_static_labels".into(),
                    category: "sample".into(),
                    send_in_pings: vec!["validation".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    ..Default::default()
                },
            },
            Some(vec!["label_1".into(), "label_2".into()]),
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

    // Definitely overrun the 16-label limit.
    for i in 1..=20 {
        metrics::labeled_boolean.get(format!("label{i}")).set(true);
    }

    metrics::labeled_boolean_static_labels
        .get("label_1")
        .set(true);
    metrics::labeled_boolean_static_labels
        .get("invalid_1")
        .set(true);
    // Overwrite the __other__ value using a second invalid label.
    metrics::labeled_boolean_static_labels
        .get("invalid_2")
        .set(false);

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    _ = &*pings::validation;
    let cfg = ConfigurationBuilder::new(true, tmpname, "firefox-desktop")
        .with_server_endpoint("invalid-test-host")
        .build();
    common::initialize(cfg);

    let map = metrics::labeled_boolean.test_get_value(None).unwrap();
    assert_eq!(map.len(), 17);
    assert_eq!(map.get("label1").unwrap(), &true);
    assert_eq!(map.get("__other__").unwrap(), &true);

    // Use the clone metric to make sure we're not reading in-memory data.
    let mut map = metrics::labeled_boolean_static_labels_clone
        .test_get_value(None)
        .unwrap();
    let mut entries: Vec<_> = map.drain().collect();
    entries.sort_unstable();
    assert_eq!(
        vec![
            ("__other__".to_string(), false),
            ("label_1".to_string(), true),
        ],
        entries
    );

    pings::validation.submit(None);
    let map = metrics::labeled_boolean.test_get_value(None).unwrap();
    assert_eq!(map.len(), 0);

    let map = metrics::labeled_boolean_static_labels
        .test_get_value(None)
        .unwrap();
    assert_eq!(map.len(), 0);

    glean::shutdown();
}
