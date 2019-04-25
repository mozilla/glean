#![allow(non_upper_case_globals)]

use super::{metrics::*, CommonMetricData, Lifetime};
use lazy_static::lazy_static;

lazy_static! {
    /// A UUID identifying a profile and allowing user-oriented Correlation of data.
    /// Some Unicode: جمع 搜集
    pub static ref clientId: StringMetric = StringMetric::new(CommonMetricData {
        name: "client_id".into(),
        category: "".into(),
        send_in_pings: vec!["glean_client_info".into(), ],
        lifetime: Lifetime::Application,
        disabled: false,
    });
}
