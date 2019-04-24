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

#[test]
fn it_works() {
    Glean::singleton().initialize();
    assert!(Glean::singleton().is_initialized());
}

#[test]
fn can_set_metrics() {
    Glean::singleton().initialize();

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
}

#[test]
fn can_snapshot() {
    Glean::singleton().initialize();

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "can_snapshot_local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        .. Default::default()
    });

    local_metric.set("snapshot 42");

    let snapshot = storage::StorageManager.snapshot("core", false);
    assert!(snapshot.contains(r#""local.can_snapshot_local_metric": "snapshot 42""#));
}

#[test]
fn snapshot_can_clear_ping_store() {
    Glean::singleton().initialize();

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "clear_snapshot_local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        .. Default::default()
    });

    local_metric.set("snapshot 43");

    let snapshot = storage::StorageManager.snapshot("core", true);
    assert!(snapshot.contains(r#""local.clear_snapshot_local_metric": "snapshot 43""#));

    let snapshot = storage::StorageManager.snapshot("core", true);
    assert!(!snapshot.contains(r#""local.clear_snapshot_local_metric": "snapshot 43""#));
}

#[test]
fn clear_is_store_specific() {
    Glean::singleton().initialize();

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "store_specific".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "baseline".into()],
        .. Default::default()
    });

    local_metric.set("snapshot 44");

    // Snapshot 1: Clear core.
    let core_snapshot = storage::StorageManager.snapshot("core", true);
    let baseline_snapshot = storage::StorageManager.snapshot("baseline", false);

    assert!(core_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
    assert!(baseline_snapshot.contains(r#""local.store_specific": "snapshot 44""#));

    // Snapshot 2: Only baseline should contain the data.
    let core_snapshot = storage::StorageManager.snapshot("core", true);
    let baseline_snapshot = storage::StorageManager.snapshot("baseline", false);

    assert!(!core_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
    assert!(baseline_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
}

lazy_static! {
    pub static ref THREADSAFE_METRIC: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "threadsafe".into(),
        category: "global".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        lifetime: Lifetime::Ping,
        disabled: false,
    });
}

#[test]
fn thread_safety() {
    use std::thread;
    use std::sync::{Arc, Barrier};

    Glean::singleton().initialize();

    let barrier = Arc::new(Barrier::new(2));
    let c = barrier.clone();
    let child = thread::spawn(move || {
        THREADSAFE_METRIC.add(1);
        c.wait();
        THREADSAFE_METRIC.add(1);
    });

    THREADSAFE_METRIC.add(1);
    barrier.wait();
    THREADSAFE_METRIC.add(1);

    child.join().unwrap();

    let snapshot = storage::StorageManager.snapshot("core", true);
    assert!(snapshot.contains(r#""global.threadsafe": 4"#));
}
