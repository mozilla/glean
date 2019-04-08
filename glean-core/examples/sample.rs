use lazy_static::lazy_static;
use glean_core::metrics::{self, StringMetric, CommonMetricData};

lazy_static! {
    pub static ref GLOBAL_METRIC : StringMetric = StringMetric::new(CommonMetricData { name: "global_metric".into() });
}

fn main() {
    let local_metric : StringMetric = StringMetric::new(CommonMetricData { name: "local_metric".into() });

    metrics::flags::a11yEnabled.set(true);
    metrics::app::clientId.set("c0ffee");
    GLOBAL_METRIC.set("I can set this");
    local_metric.set("and set this too");

    println!("{}", metrics::StorageManager.dump());
}
