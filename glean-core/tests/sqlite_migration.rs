mod common;
use std::fs;

use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;
use glean_core::Lifetime;
use uuid::uuid;

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

#[test]
fn migration_succeeds() {
    let temp = tempfile::tempdir().unwrap();
    let db_path = temp.path().join("db");
    fs::create_dir_all(&db_path).unwrap();

    let safe_bin = db_path.join("data.safe.bin");
    // File has been generated from essentially:
    //
    // ```rust
    //         let tmpname = PathBuf::new("/tmp/glean-fc");
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
    fs::write(
        safe_bin,
        include_bytes!("77ca0472-5124-4f6b-971d-4a2a928fb158.safe.bin"),
    )
    .unwrap();
    let exp_client_id = uuid!("77ca0472-5124-4f6b-971d-4a2a928fb158");

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(exp_client_id, client_id);

    // TODO: validate migration metrics
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
}
