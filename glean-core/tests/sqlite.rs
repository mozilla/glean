// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use std::fs;

use crate::common::*;

use glean_core::metrics::*;
use glean_core::CommonMetricData;
use glean_core::Lifetime;
use rusqlite::params;
use rusqlite::TransactionBehavior;
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
fn database_file_is_not_sqlite() {
    let temp = {
        let (glean, temp) = new_glean(None);
        drop(glean);
        temp
    };

    {
        let path = temp.path().join("db").join("glean.sqlite");
        fs::remove_file(&path).unwrap();
        fs::write(&path, "not sqlite").unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None);
    assert!(client_id.is_some());
}

#[test]
fn database_contains_wrong_table() {
    let temp = {
        let (glean, temp) = new_glean(None);
        drop(glean);
        temp
    };

    {
        let path = temp.path().join("db").join("glean.sqlite");
        fs::remove_file(&path).unwrap();

        let conn = rusqlite::Connection::open(path).unwrap();
        conn.execute("CREATE TABLE telemetry (a TEXT)", ()).unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None);
    assert!(client_id.is_some());
}

#[test]
#[ignore]
fn database_contains_correct_user_version_but_wrong_table() {
    let temp = {
        let (glean, temp) = new_glean(None);
        drop(glean);
        temp
    };

    {
        let path = temp.path().join("db").join("glean.sqlite");
        let conn = rusqlite::Connection::open(path).unwrap();
        conn.execute("DROP TABLE telemetry", ()).unwrap();
        conn.execute("CREATE TABLE telemetry (a TEXT)", ()).unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None);
    assert!(client_id.is_some());
}

#[test]
fn invalid_msgpack_value() {
    let (first_client_id, temp) = {
        let (glean, temp) = new_glean(None);
        let client_id = clientid_metric().get_value(&glean, None).unwrap();
        drop(glean);
        (client_id, temp)
    };

    {
        let path = temp.path().join("db").join("glean.sqlite");
        let conn = rusqlite::Connection::open(path).unwrap();
        conn.execute(
            "UPDATE telemetry SET value = ?1 WHERE id = 'client_id'",
            params![b"c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0"],
        )
        .unwrap();

        // Also remove the client_id.txt so the client_id is not re-set from it.
        fs::remove_file(temp.path().join("client_id.txt")).unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    let known_id = uuid!("c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0");
    assert_ne!(known_id, client_id);
    assert_ne!(first_client_id, client_id);
}

#[test]
fn higher_user_version_upgrade_does_not_crash() {
    let (first_client_id, temp) = {
        let (glean, temp) = new_glean(None);
        let client_id = clientid_metric().get_value(&glean, None).unwrap();
        drop(glean);
        (client_id, temp)
    };

    {
        let path = temp.path().join("db").join("glean.sqlite");
        let conn = rusqlite::Connection::open(path).unwrap();
        conn.execute_batch("PRAGMA user_version = 2").unwrap();
    }

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(first_client_id, client_id);
}

// Permissions only really work on Unix systems, definitely not on Windows
#[cfg(unix)]
mod unix {
    use glean_core::Glean;

    use super::*;

    #[test]
    fn database_permission_error() {
        let temp = tempfile::tempdir().unwrap();

        let db_path = temp.path().join("db");
        fs::create_dir_all(&db_path).unwrap();
        let path = db_path.join("glean.sqlite");
        fs::write(&path, "").unwrap();
        let attr = fs::metadata(&path).unwrap();
        let original_permissions = attr.permissions();
        let mut permissions = original_permissions.clone();
        permissions.set_readonly(true);
        fs::set_permissions(&path, permissions).unwrap();

        let cfg = glean_core::InternalConfiguration {
            data_path: path.display().to_string(),
            application_id: GLOBAL_APPLICATION_ID.into(),
            language_binding_name: "Rust".into(),
            upload_enabled: true,
            max_events: None,
            delay_ping_lifetime_io: false,
            app_build: "Unknown".into(),
            use_core_mps: false,
            trim_data_to_registered_pings: false,
            log_level: None,
            rate_limit: None,
            enable_event_timestamps: false,
            experimentation_id: None,
            enable_internal_pings: true,
            ping_schedule: Default::default(),
            ping_lifetime_threshold: 0,
            ping_lifetime_max_time: 0,
        };
        let glean = Glean::new(cfg);
        assert!(glean.is_err());
    }
}

#[test]
#[ignore]
fn database_externally_locked() {
    let temp = {
        let (glean, temp) = new_glean(None);
        drop(glean);
        temp
    };

    let path = temp.path().join("db").join("glean.sqlite");
    let mut conn = rusqlite::Connection::open(path).unwrap();
    let _tx = conn
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .unwrap();

    let (glean, _temp) = new_glean(Some(temp));

    let client_id = clientid_metric().get_value(&glean, None);
    assert!(client_id.is_some());
}

mod migration {
    use super::*;

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
}
