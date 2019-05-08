use super::{metrics::*, CommonMetricData, Lifetime};

#[derive(Debug)]
pub struct CoreMetrics {
    pub client_id: UuidMetric,
    pub first_run: BooleanMetric,
}

impl CoreMetrics {
    pub fn new() -> CoreMetrics {
        CoreMetrics {
            client_id: UuidMetric::new(CommonMetricData {
                name: "client_id".into(),
                category: "".into(),
                send_in_pings: vec!["glean_client_info".into(), ],
                lifetime: Lifetime::Application,
                disabled: false,
            }),

            first_run: BooleanMetric::new(CommonMetricData {
                name: "first_run".into(),
                category: "".into(),
                send_in_pings: vec!["glean_client_info".into(), ],
                lifetime: Lifetime::Application,
                disabled: false,
            }),
        }
    }
}
