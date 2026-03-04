mod common;
use std::fs;

use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;
use glean_core::Lifetime;
use rkv::{Rkv, StoreOptions};
use uuid::uuid;

static RKV_DATABASE: &[u8] = include_bytes!("77ca0472-5124-4f6b-971d-4a2a928fb158.safe.bin");

fn clientid_metric() -> UuidMetric {
    UuidMetric::new(CommonMetricData {
        name: "client_id".into(),
        category: "".into(),
        send_in_pings: vec!["glean_client_info".into()],
        lifetime: Lifetime::User,
        disabled: false,
        label: None,
    })
}

/// Copy of parts of `DatabaseMetrics` in `glean-core/src/internal_metrics.rs` for ease of testing.
struct MigrationMetrics {
    metrics_in_sqlite: CounterMetric,
    migrated_metrics: CounterMetric,
    failed_metrics: CounterMetric,
    migration_duration: TimespanMetric,
    migration_error: CounterMetric,
}

impl MigrationMetrics {
    fn new() -> Self {
        Self {
            migrated_metrics: CounterMetric::new(CommonMetricData {
                name: "migrated_metrics".into(),
                category: "glean.migration".into(),
                send_in_pings: vec!["metrics".into(), "health".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                label: None,
            }),

            metrics_in_sqlite: CounterMetric::new(CommonMetricData {
                name: "metrics_in_sqlite".into(),
                category: "glean.migration".into(),
                send_in_pings: vec!["metrics".into(), "health".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                label: None,
            }),

            failed_metrics: CounterMetric::new(CommonMetricData {
                name: "failed_metrics".into(),
                category: "glean.migration".into(),
                send_in_pings: vec!["metrics".into(), "health".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                label: None,
            }),

            migration_duration: TimespanMetric::new(
                CommonMetricData {
                    name: "migration_duration".into(),
                    category: "glean.migration".into(),
                    send_in_pings: vec!["metrics".into(), "health".into()],
                    lifetime: Lifetime::Ping,
                    disabled: false,
                    label: None,
                },
                TimeUnit::Millisecond,
            ),
            migration_error: CounterMetric::new(CommonMetricData {
                name: "error".into(),
                category: "glean.migration".into(),
                send_in_pings: vec!["metrics".into(), "health".into()],
                lifetime: Lifetime::Ping,
                disabled: false,
                label: None,
            }),
        }
    }
}

#[test]
fn migration_succeeds() {
    let temp = tempfile::tempdir().unwrap();
    let db_path = temp.path().join("db");
    fs::create_dir_all(&db_path).unwrap();

    let safe_bin = db_path.join("data.safe.bin");
    // File has been generated from essentially:
    //
    // ```rust
    // let tmpname = PathBuf::new("/tmp/glean-fc");
    // let cfg = ConfigurationBuilder::new(true, tmpname.clone(), "glean-fc")
    //     .with_server_endpoint("invalid-test-host")
    //     .with_use_core_mps(false)
    //     .build();
    // glean::initialize(cfg, client_info);
    // glean::shutdown();
    // ```
    //
    // All ping-specific metrics have been removed.
    // Only client_info metrics are migrated, including the client ID.
    fs::write(safe_bin, RKV_DATABASE).unwrap();
    let exp_client_id = uuid!("77ca0472-5124-4f6b-971d-4a2a928fb158");

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(exp_client_id, client_id);

    let metrics = MigrationMetrics::new();
    assert_eq!(Some(13), metrics.migrated_metrics.get_value(&glean, None));
    assert_eq!(Some(13), metrics.metrics_in_sqlite.get_value(&glean, None));
    assert_eq!(None, metrics.failed_metrics.get_value(&glean, None));
    assert!(metrics.migration_duration.get_value(&glean, None).is_some());
    assert_eq!(None, metrics.migration_error.get_value(&glean, None));
}

#[test]
fn migration_skipped_if_database_exists() {
    let (first_client_id, temp) = {
        let (glean, temp) = new_glean(None);
        let client_id = clientid_metric().get_value(&glean, None).unwrap();
        drop(glean);
        (client_id, temp)
    };

    let safe_bin = temp.path().join("db").join("data.safe.bin");
    fs::write(
        &safe_bin,
        include_bytes!("77ca0472-5124-4f6b-971d-4a2a928fb158.safe.bin"),
    )
    .unwrap();
    let rkv_client_id = uuid!("77ca0472-5124-4f6b-971d-4a2a928fb158");

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(
        first_client_id, client_id,
        "Client ID should be the one first generated"
    );
    assert_ne!(
        rkv_client_id, client_id,
        "Client ID should not be one from the Rkv database"
    );
    assert!(safe_bin.exists(), "Rkv file should not have been deleted");

    let metrics = MigrationMetrics::new();
    assert_eq!(None, metrics.migrated_metrics.get_value(&glean, None));
    assert_eq!(None, metrics.metrics_in_sqlite.get_value(&glean, None));
    assert_eq!(None, metrics.failed_metrics.get_value(&glean, None));
    assert_eq!(None, metrics.migration_duration.get_value(&glean, None));
    assert_eq!(None, metrics.migration_error.get_value(&glean, None));
}

#[test]
fn migration_succeeds_with_failures() {
    let temp = tempfile::tempdir().unwrap();
    let db_path = temp.path().join("db");
    fs::create_dir_all(&db_path).unwrap();

    let safe_bin = db_path.join("data.safe.bin");
    // Reusing the same database file from above.
    fs::write(safe_bin, RKV_DATABASE).unwrap();
    let exp_client_id = uuid!("77ca0472-5124-4f6b-971d-4a2a928fb158");

    // Modifying the database to force migration errors.
    {
        let rkv = Rkv::new::<rkv::backend::SafeMode>(&db_path).unwrap();
        let ping_store = rkv
            .open_single(Lifetime::Ping.as_str(), StoreOptions::create())
            .unwrap();
        let mut writer = rkv.write().unwrap();

        let key = "metrics#a.broken.metric";
        let value = rkv::Value::Blob(b"not a value");
        ping_store.put(&mut writer, key, &value).unwrap();

        let key = "baseline#second.broken.metric";
        let value = rkv::Value::I64(31);
        ping_store.put(&mut writer, key, &value).unwrap();

        writer.commit().unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(exp_client_id, client_id);

    let metrics = MigrationMetrics::new();
    assert_eq!(Some(13), metrics.migrated_metrics.get_value(&glean, None));
    assert_eq!(Some(13), metrics.metrics_in_sqlite.get_value(&glean, None));

    // We injected 2 broken metrics.
    assert_eq!(Some(2), metrics.failed_metrics.get_value(&glean, None));

    assert!(metrics.migration_duration.get_value(&glean, None).is_some());
    assert_eq!(None, metrics.migration_error.get_value(&glean, None));
}

#[test]
fn migration_fails() {
    let temp = tempfile::tempdir().unwrap();
    let db_path = temp.path().join("db");
    fs::create_dir_all(&db_path).unwrap();

    let safe_bin = db_path.join("data.safe.bin");
    fs::write(safe_bin, b"\0").unwrap();

    let (glean, _temp) = new_glean(Some(temp));

    let metrics = MigrationMetrics::new();
    assert_eq!(Some(1), metrics.migration_error.get_value(&glean, None));

    assert_eq!(None, metrics.migrated_metrics.get_value(&glean, None));
    assert_eq!(None, metrics.metrics_in_sqlite.get_value(&glean, None));
    assert_eq!(None, metrics.failed_metrics.get_value(&glean, None));
    assert_eq!(None, metrics.migration_duration.get_value(&glean, None));
}
