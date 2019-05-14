use std::env;

use glean_core::metrics::*;
use glean_core::ping;
use glean_core::{CommonMetricData, Glean};
use tempfile::Builder;

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

    let mut glean = Glean::new(&data_path, "org.mozilla.glean_core.example");
    assert!(glean.is_initialized());

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

    local_metric.set(&glean, "I can set this");
    call_counter.add(&glean, 1);

    println!("Core Data:\n{}", glean.snapshot("core", true));

    call_counter.add(&glean, 2);
    println!("Metrics Data:\n{}", glean.snapshot("metrics", true));

    call_counter.add(&glean, 3);

    println!();
    println!("Core Data 2:\n{}", glean.snapshot("core", false));
    println!("Metrics Data 2:\n{}", glean.snapshot("metrics", true));

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });
    list.add(&glean, "once");
    list.add(&glean, "upon");

    let ping = ping::collect(glean.storage(), "core");
    let ping = ::serde_json::to_string_pretty(&ping).unwrap();
    println!("Ping:\n{}", ping);

    let mut long_string = std::iter::repeat('x').take(49).collect::<String>();
    long_string.push('a');
    long_string.push('b');
    local_metric.set(&glean, long_string);
    let ping = ping::collect(glean.storage(), "core");
    let ping = ::serde_json::to_string_pretty(&ping).unwrap();
    println!("Metrics Ping:\n{}", ping);
}
