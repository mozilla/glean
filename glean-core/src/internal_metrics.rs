#![allow(non_upper_case_globals)]

use super::{metrics::StringMetric, CommonMetricData};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref clientId: StringMetric = StringMetric::new(CommonMetricData {
        name: "glean.clientId".into()
    });
}
