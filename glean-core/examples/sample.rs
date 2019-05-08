use std::env;

use glean_core::metrics::*;
use glean_core::ping::PingMaker;
use glean_core::{storage, CommonMetricData, Glean, Lifetime};
use lazy_static::lazy_static;
use tempfile::Builder;

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
    env_logger::init();
    color_backtrace::install();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        path
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().display().to_string()
    };

    Glean::singleton().initialize(&data_path, "org.mozilla.glean_core.example");
    assert!(Glean::singleton().is_initialized());

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    let call_counter: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "calls".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        ..Default::default()
    });

    GLOBAL_METRIC.set(true);
    local_metric.set("I can set this");
    call_counter.add(1);

    println!(
        "Core Data:\n{}",
        storage::StorageManager.snapshot("core", true)
    );
    call_counter.add(2);
    println!(
        "Metrics Data:\n{}",
        storage::StorageManager.snapshot("metrics", true)
    );

    call_counter.add(3);

    println!();
    println!(
        "Core Data 2:\n{}",
        storage::StorageManager.snapshot("core", false)
    );
    println!(
        "Metrics Data 2:\n{}",
        storage::StorageManager.snapshot("metrics", true)
    );

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });
    list.add("once");
    list.add("upon");

    let ping_maker = PingMaker::new();
    let ping = ping_maker.collect("core");
    let ping = ::serde_json::to_string_pretty(&ping).unwrap();
    println!("Ping:\n{}", ping);

    let mut long_string = std::iter::repeat('x').take(49).collect::<String>();
    long_string.push('a');
    long_string.push('b');
    local_metric.set(long_string);
    let ping_maker = PingMaker::new();
    let ping = ping_maker.collect("core");
    let ping = ::serde_json::to_string_pretty(&ping).unwrap();
    println!("Metrics Ping:\n{}", ping);
}
