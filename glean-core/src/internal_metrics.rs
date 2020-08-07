// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use super::{metrics::*, CommonMetricData, Lifetime};

#[derive(Debug)]
pub struct CoreMetrics {
    pub client_id: UuidMetric,
    pub first_run_date: DatetimeMetric,
    pub os: StringMetric,
}

impl CoreMetrics {
    pub fn new() -> CoreMetrics {
        CoreMetrics {
            client_id: UuidMetric::new(CommonMetricData {
                name: "client_id".into(),
                category: "".into(),
                send_in_pings: vec!["glean_client_info".into()],
                lifetime: Lifetime::User,
                disabled: false,
                dynamic_label: None,
            }),

            first_run_date: DatetimeMetric::new(
                CommonMetricData {
                    name: "first_run_date".into(),
                    category: "".into(),
                    send_in_pings: vec!["glean_client_info".into()],
                    lifetime: Lifetime::User,
                    disabled: false,
                    dynamic_label: None,
                },
                TimeUnit::Day,
            ),

            os: StringMetric::new(CommonMetricData {
                name: "os".into(),
                category: "".into(),
                send_in_pings: vec!["glean_client_info".into()],
                lifetime: Lifetime::Application,
                disabled: false,
                dynamic_label: None,
            }),
        }
    }
}

#[derive(Debug)]
pub struct UploadMetrics {
    pub ping_upload_failure: LabeledMetric<CounterMetric>,
    pub discarded_exceeding_pings_size: MemoryDistributionMetric,
    pub pending_pings_directory_size: MemoryDistributionMetric,
    pub deleted_pings_after_quota_hit: CounterMetric,
}

impl UploadMetrics {
    pub fn new() -> UploadMetrics {
        UploadMetrics {
            ping_upload_failure: LabeledMetric::new(
                CounterMetric::new(CommonMetricData {
                    name: "ping_upload_failure".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                }),
                Some(vec![
                    "status_code_4xx".into(),
                    "status_code_5xx".into(),
                    "status_code_unknown".into(),
                    "unrecoverable".into(),
                    "recoverable".into(),
                ]),
            ),

            discarded_exceeding_pings_size: MemoryDistributionMetric::new(
                CommonMetricData {
                    name: "discarded_exceeding_ping_size".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                },
                MemoryUnit::Kilobyte,
            ),

            pending_pings_directory_size: MemoryDistributionMetric::new(
                CommonMetricData {
                    name: "pending_pings_directory_size".into(),
                    category: "glean.upload".into(),
                    send_in_pings: vec!["metrics".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    dynamic_label: None,
                },
                MemoryUnit::Kilobyte,
            ),

            deleted_pings_after_quota_hit: CounterMetric::new(CommonMetricData {
                name: "deleted_pings_after_quota_hit".into(),
                category: "glean.upload".into(),
                send_in_pings: vec!["metrics".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                dynamic_label: None,
            }),
        }
    }
}
