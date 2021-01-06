// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

use once_cell::sync::Lazy;
use tempfile::Builder;

use glean::{private::PingType, ClientInfoMetrics, Configuration};

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
pub static PrototypePing: Lazy<PingType> =
    Lazy::new(|| PingType::new("prototype", true, true, vec![]));

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        path
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().display().to_string()
    };

    let cfg = Configuration {
        data_path,
        application_id: "org.mozilla.glean_core.example".into(),
        upload_enabled: true,
        max_events: None,
        delay_ping_lifetime_io: false,
        channel: None,
        server_endpoint: Some("invalid-test-host".into()),
        uploader: None,
    };

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
    };

    glean::initialize(cfg, client_info);
    glean::register_ping_type(&PrototypePing);

    glean_metrics::sample_boolean.set(true);

    glean::submit_ping_by_name("prototype", None);
}
