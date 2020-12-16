// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

mod common;

use glean::Configuration;

/// Some user metrics.
mod metrics {
    pub mod fog {
        use glean::private::*;
        use glean::{Lifetime, TimeUnit};
        use glean_core::CommonMetricData;
        use once_cell::sync::Lazy;

        #[allow(non_upper_case_globals)]
        /// generated from fog.initialization
        ///
        /// Time the FOG initialization takes.
        pub static initialization: Lazy<TimespanMetric> = Lazy::new(|| {
            TimespanMetric::new(
                CommonMetricData {
                    name: "initialization".into(),
                    category: "fog".into(),
                    send_in_pings: vec!["fog-validation".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    ..Default::default()
                },
                TimeUnit::Nanosecond,
            )
        });
    }
}

mod pings {
    use glean::private::PingType;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static fog_validation: Lazy<PingType> =
        Lazy::new(|| glean::private::PingType::new("fog-validation", true, true, vec![]));
}

/// Test scenario: Glean initialization fails.
///
/// FOG tries to initializate Glean, but that somehow fails.
#[test]
fn fog_init_fails() {
    common::enable_test_logging();

    metrics::fog::initialization.start();

    // Create a custom configuration to use a validating uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().display().to_string();

    let cfg = Configuration {
        data_path: tmpname,
        application_id: "firefox-desktop".into(), // An empty application ID is invalid.
        upload_enabled: true,
        max_events: None,
        delay_ping_lifetime_io: false,
        channel: Some("testing".into()),
        server_endpoint: Some("invalid-test-host".into()),
        uploader: None,
    };
    common::initialize(cfg);

    metrics::fog::initialization.stop();

    pings::fog_validation.submit(None);

    // We don't test for data here, as that would block on the dispatcher.

    // Shut it down immediately; this might not be enough time to initialize.

    // eventually this is called by `FOG::Shutdown()`.
    glean::shutdown();
}
