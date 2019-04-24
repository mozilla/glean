use glean_core::metrics::{BooleanMetric, StringMetric, CounterMetric};
use glean_core::{storage, CommonMetricData, Lifetime, Glean};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref GLOBAL_METRIC: BooleanMetric = BooleanMetric::new(CommonMetricData {
        name: "global_metric".into(),
        category: "global".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        lifetime: Lifetime::Ping,
        disabled: false,
    });
}

fn main() {
    Glean::singleton().initialize();
    assert!(Glean::singleton().is_initialized());

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        .. Default::default()
    });

    let call_counter: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "calls".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        .. Default::default()
    });

    GLOBAL_METRIC.set(true);
    local_metric.set("I can set this");
    call_counter.add(1);

    println!("Core Data:\n{}", storage::StorageManager.snapshot("core", true));
    call_counter.add(2);
    println!("Metrics Data:\n{}", storage::StorageManager.snapshot("metrics", true));

    call_counter.add(3);

    println!();
    println!("Core Data 2:\n{}", storage::StorageManager.snapshot("core", true));
    println!("Metrics Data 2:\n{}", storage::StorageManager.snapshot("metrics", true));
}
