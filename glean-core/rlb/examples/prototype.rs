// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use tempfile::Builder;

use glean::{private::PingType, ClientInfoMetrics, ConfigurationBuilder};

pub mod glean_metrics {
    use glean::{private::BooleanMetric, CommonMetricData, Lifetime};

    #[allow(non_upper_case_globals)]
    pub static sample_boolean: once_cell::sync::Lazy<BooleanMetric> =
        once_cell::sync::Lazy::new(|| {
            BooleanMetric::new(CommonMetricData {
                name: "sample_boolean".into(),
                category: "test.metrics".into(),
                send_in_pings: vec!["prototype".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            })
        });
}

#[allow(non_upper_case_globals)]
pub static PrototypePing: Lazy<PingType> = Lazy::new(|| {
    PingType::new(
        "prototype",
        true,
        true,
        true,
        true,
        true,
        vec![],
        vec![],
        true,
        vec![],
    )
});

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    };

    _ = &*PrototypePing;
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    glean_metrics::sample_boolean.set(true);
    _ = glean_metrics::sample_boolean.test_get_value(None);

    PrototypePing.submit(None);

    glean_core::dispatcher::launch(|| {
        std::thread::sleep(std::time::Duration::from_secs(15));
    });

    glean::shutdown();
    std::thread::sleep(std::time::Duration::from_secs(10));
}
