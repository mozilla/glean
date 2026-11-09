use std::{collections::BTreeSet, env, sync::atomic::AtomicU8};

use glean::{ClientInfoMetrics, CommonMetricData, ConfigurationBuilder, Lifetime, net, private::PingType};
use glean_core::{MetricLabel, metrics::Metric};

const DATA: &str = include_str!("data.json");

#[derive(serde::Deserialize, Debug)]
struct Data {
    key: String,
    ping: String,
    lifetime: Lifetime,
    labels: String,
    value: Metric,
}

#[derive(Debug)]
struct MockUploader;

impl MockUploader {
    fn new() -> Self {
        Self
    }
}

impl net::PingUploader for MockUploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        net::UploadResult::http_status(200)
    }
}

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);
    let data_path = args.next().expect("DATA_PATH");

    log::info!("Data path: {}", data_path);

    let uploader = MockUploader::new();
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.filldb")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .with_uploader(uploader)
        .with_delay_ping_lifetime_io(false)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
        os_version: None,
    };

    let mut all_metrics = vec![];
    for line in DATA.lines() {
        let data: Data = serde_json::from_str(line).unwrap();
        all_metrics.push(data);
    }

    // Register all pings so we can register data.
    let pings: BTreeSet<_> = all_metrics.iter().map(|data| &data.ping).collect();
    for ping in pings {
        let ping = PingType::new(ping, true, false, true, true, true, vec![], vec![], true, vec![]);
        _ = ping;
    }

    glean::initialize(cfg, client_info);
    glean_core::join_init();

    glean_core::core::with_glean(|glean| {
        let storage = glean.storage();

        for metric in all_metrics {
            let (category, name) = metric.key.rsplit_once(".").unwrap();
            let category = category.to_string();
            let name = name.to_string();
            let label = if metric.labels.is_empty() {
                None
            } else {
                Some(MetricLabel::Static(metric.labels))
            };
            let cmd = glean_core::CommonMetricDataInternal {
                inner: CommonMetricData {
                    name,
                    category,
                    send_in_pings: vec![metric.ping],
                    lifetime: metric.lifetime,
                    disabled: false,
                    in_session: false,
                    label,
                },
                disabled: AtomicU8::new(0),
            };

            let value = &metric.value;
            storage.record(glean, &cmd, value);
        }
    });

    //glean::shutdown();
}
