// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;
use std::path::PathBuf;
use std::{thread, time};

use once_cell::sync::Lazy;

use glean::net;
use glean::{private::PingType, ClientInfoMetrics, ConfigurationBuilder, TestGetValue};

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

// Define a fake uploader that sleeps.
#[derive(Debug)]
struct FakeUploader;

impl net::PingUploader for FakeUploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        thread::sleep(time::Duration::from_millis(100));
        net::UploadResult::http_status(200)
    }
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

    let data_path = PathBuf::from(args.next().expect("need data path"));

    _ = &*PrototypePing;
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.longrunning")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(FakeUploader)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    // Wait for init to finish,
    // otherwise we might be to quick with calling `shutdown`.
    let _ = glean_metrics::sample_boolean.test_get_value(None);

    glean_metrics::sample_boolean.set(true);

    log::info!("submitting PrototypePing");
    PrototypePing.submit(None);

    glean::shutdown();
}
