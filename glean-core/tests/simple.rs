use glean_core::metrics::*;
use glean_core::{CommonMetricData, Glean};

fn tempdir() -> (tempfile::TempDir, String) {
    let t = tempfile::tempdir().unwrap();
    let name = t.path().display().to_string();
    (t, name)
}

const GLOBAL_APPLICATION_ID: &str = "org.mozilla.glean.test.app";

#[test]
fn it_works() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);
    assert!(glean.is_initialized());
}

#[test]
fn can_set_metrics() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let local_metric = StringMetric::new(CommonMetricData {
        name: "local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    let call_counter = CounterMetric::new(CommonMetricData {
        name: "calls".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        ..Default::default()
    });

    local_metric.set(&glean, "I can set this");
    call_counter.add(&glean, 1);
}

#[test]
fn can_snapshot() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let local_metric = StringMetric::new(CommonMetricData {
        name: "can_snapshot_local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    local_metric.set(&glean, "snapshot 42");

    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.can_snapshot_local_metric": "snapshot 42""#));
}

#[test]
fn snapshot_can_clear_ping_store() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let local_metric = StringMetric::new(CommonMetricData {
        name: "clear_snapshot_local_metric".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    local_metric.set(&glean, "snapshot 43");

    let snapshot = glean.snapshot("core", true);
    assert!(snapshot.contains(r#""local.clear_snapshot_local_metric": "snapshot 43""#));

    let snapshot = glean.snapshot("core", true);
    assert!(!snapshot.contains(r#""local.clear_snapshot_local_metric": "snapshot 43""#));
}

#[test]
fn clear_is_store_specific() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let local_metric: StringMetric = StringMetric::new(CommonMetricData {
        name: "store_specific".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "baseline".into()],
        ..Default::default()
    });

    local_metric.set(&glean, "snapshot 44");

    // Snapshot 1: Clear core.
    let core_snapshot = glean.snapshot("core", true);
    let baseline_snapshot = glean.snapshot("baseline", false);

    assert!(core_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
    assert!(baseline_snapshot.contains(r#""local.store_specific": "snapshot 44""#));

    // Snapshot 2: Only baseline should contain the data.
    let core_snapshot = glean.snapshot("core", true);
    let baseline_snapshot = glean.snapshot("baseline", false);

    assert!(!core_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
    assert!(baseline_snapshot.contains(r#""local.store_specific": "snapshot 44""#));
}

#[test]
fn thread_safety() {
    use std::sync::{Arc, Barrier, Mutex};
    use std::thread;

    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);
    let glean = Arc::new(Mutex::new(glean));

    let threadsafe_metric = CounterMetric::new(CommonMetricData {
        name: "threadsafe".into(),
        category: "global".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        ..Default::default()
    });
    let threadsafe_metric = Arc::new(threadsafe_metric);

    let barrier = Arc::new(Barrier::new(2));
    let c = barrier.clone();
    let threadsafe_metric_clone = threadsafe_metric.clone();
    let glean_clone = glean.clone();
    let child = thread::spawn(move || {
        threadsafe_metric_clone.add(&*glean_clone.lock().unwrap(), 1);
        c.wait();
        threadsafe_metric_clone.add(&*glean_clone.lock().unwrap(), 1);
    });

    threadsafe_metric.add(&*glean.lock().unwrap(), 1);
    barrier.wait();
    threadsafe_metric.add(&*glean.lock().unwrap(), 1);

    child.join().unwrap();

    let snapshot = glean.lock().unwrap().snapshot("core", true);
    assert!(snapshot.contains(r#""global.threadsafe": 4"#));
}

#[test]
fn transformation_works() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let counter: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "transformation".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into(), "metrics".into()],
        ..Default::default()
    });

    counter.add(&glean, 2);

    let core_snapshot = glean.snapshot("core", true);
    let metrics_snapshot = glean.snapshot("metrics", false);
    assert!(
        core_snapshot.contains(r#""local.transformation": 2"#),
        format!("core snapshot 1: {}", core_snapshot)
    );
    assert!(
        metrics_snapshot.contains(r#""local.transformation": 2"#),
        format!("metrics snapshot 1: {}", metrics_snapshot)
    );

    counter.add(&glean, 2);
    let core_snapshot = glean.snapshot("core", true);
    let metrics_snapshot = glean.snapshot("metrics", false);
    assert!(
        core_snapshot.contains(r#""local.transformation": 2"#),
        format!("core snapshot 2: {}", core_snapshot)
    );
    assert!(
        metrics_snapshot.contains(r#""local.transformation": 4"#),
        format!("metrics snapshot 2: {}", metrics_snapshot)
    );
}

#[test]
fn uuid() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let uuid: UuidMetric = UuidMetric::new(CommonMetricData {
        name: "uuid".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    uuid.generate(&glean);
    let snapshot = glean.snapshot("core", false);
    assert!(
        snapshot.contains(r#""local.uuid": ""#),
        format!("Snapshot 1: {}", snapshot)
    );

    uuid.generate(&glean);
    let snapshot = glean.snapshot("core", false);
    assert!(
        snapshot.contains(r#""local.uuid": ""#),
        format!("Snapshot 2: {}", snapshot)
    );
}

#[test]
fn list() {
    let (_t, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    let list: StringListMetric = StringListMetric::new(CommonMetricData {
        name: "list".into(),
        category: "local".into(),
        send_in_pings: vec!["core".into()],
        ..Default::default()
    });

    list.add(&glean, "first");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));

    list.add(&glean, "second");
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(snapshot.contains(r#""first""#));
    assert!(snapshot.contains(r#""second""#));

    list.set(&glean, vec!["third".into()]);
    let snapshot = glean.snapshot("core", false);
    assert!(snapshot.contains(r#""local.list": ["#));
    assert!(!snapshot.contains(r#""first""#));
    assert!(!snapshot.contains(r#""second""#));
    assert!(snapshot.contains(r#""third""#));
}

#[test]
fn write_ping_to_disk() {
    let (temp, tmpname) = tempdir();
    let mut glean = Glean::new();
    glean.initialize(&tmpname, GLOBAL_APPLICATION_ID);

    glean.send_ping("metrics").unwrap();

    let path = temp.path().join("pings");

    let mut count = 0;
    for entry in std::fs::read_dir(path).unwrap() {
        assert!(entry.unwrap().path().is_file());
        count += 1;
    }
    assert_eq!(1, count);
}
