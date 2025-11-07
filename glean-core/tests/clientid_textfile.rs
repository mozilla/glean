// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use std::fs;
use std::path::Path;

use rkv::Rkv;
use rkv::StoreOptions;
use uuid::Uuid;

use glean_core::metrics::*;
use glean_core::CommonMetricData;
use glean_core::Lifetime;

fn clientid_metric() -> UuidMetric {
    UuidMetric::new(CommonMetricData {
        name: "client_id".into(),
        category: "".into(),
        send_in_pings: vec!["glean_client_info".into()],
        lifetime: Lifetime::User,
        disabled: false,
        dynamic_label: None,
    })
}

fn clientid_from_file(data_path: &Path) -> Option<Uuid> {
    let path = data_path.join("client_id.txt");
    let uuid_str = fs::read_to_string(path).ok()?;
    Uuid::parse_str(uuid_str.trim_end()).ok()
}

fn write_clientid_to_file(data_path: &Path, uuid: &Uuid) -> Option<()> {
    let mut buffer = Uuid::encode_buffer();
    let uuid_str = uuid.hyphenated().encode_lower(&mut buffer);
    write_clientid_to_file_str(data_path, uuid_str)
}

fn write_clientid_to_file_str(data_path: &Path, s: &str) -> Option<()> {
    let path = data_path.join("client_id.txt");
    fs::write(path, s.as_bytes()).ok()?;
    Some(())
}

#[test]
fn writes_clientid_file() {
    let (glean, temp) = new_glean(None);

    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    let file_client_id = clientid_from_file(temp.path()).unwrap();

    assert_eq!(file_client_id, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = &payload["metrics"]["string"]["glean.health.exception_state"];
    assert_eq!(&serde_json::Value::Null, state);
}

#[test]
fn reused_clientid_from_file() {
    let temp = tempfile::tempdir().unwrap();
    let new_uuid = Uuid::new_v4();

    write_clientid_to_file(temp.path(), &new_uuid).unwrap();

    let (glean, temp) = new_glean(Some(temp));
    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(new_uuid, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = payload["metrics"]["string"]["glean.health.exception_state"].as_str();
    assert_eq!(Some("empty-db"), state);
}

#[test]
fn restores_clientid_file_from_db() {
    let (db_client_id, temp) = {
        // Ensure we initialize once to get a client_id
        let (glean, temp) = new_glean(None);
        let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
        drop(glean);

        (db_client_id, temp)
    };

    // Removing the file. `Glean::new` should restore it
    fs::remove_file(temp.path().join("client_id.txt")).unwrap();

    let (glean, temp) = new_glean(Some(temp));
    let file_client_id = clientid_from_file(temp.path()).unwrap();
    assert_eq!(file_client_id, db_client_id);

    let db_client_id2 = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(db_client_id, db_client_id2);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = payload["metrics"]["string"]["glean.health.exception_state"].as_str();
    assert_eq!(None, state);
}

#[test]
fn clientid_regen_issue_with_existing_db() {
    let (file_client_id, temp) = {
        // Ensure we initialize once to get a client_id
        let (glean, temp) = new_glean(None);
        let file_client_id = clientid_from_file(temp.path()).unwrap();
        drop(glean);

        (file_client_id, temp)
    };

    // We modify the database and ONLY clear out the client id.
    {
        let path = temp.path().join("db");
        let rkv = Rkv::new::<rkv::backend::SafeMode>(&path).unwrap();
        let user_store = rkv.open_single("user", StoreOptions::create()).unwrap();

        // We know this.
        let client_id_key = "glean_client_info#client_id";

        let mut writer = rkv.write().unwrap();
        user_store.delete(&mut writer, client_id_key).unwrap();
        writer.commit().unwrap();
    }

    let (glean, temp) = new_glean(Some(temp));

    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(file_client_id, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = payload["metrics"]["string"]["glean.health.exception_state"].as_str();
    assert_eq!(Some("regen-db"), state);
}

#[test]
fn db_client_id_prefered_over_file_client_id() {
    let (db_client_id, temp) = {
        // Ensure we initialize once to get a client_id
        let (glean, temp) = new_glean(None);
        let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
        drop(glean);

        (db_client_id, temp)
    };

    // We modify the client id file
    let new_uuid = Uuid::new_v4();
    write_clientid_to_file(temp.path(), &new_uuid).unwrap();

    let (glean, temp) = new_glean(Some(temp));
    let db_client_id2 = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(db_client_id, db_client_id2);

    let file_client_id = clientid_from_file(temp.path()).unwrap();
    assert_eq!(file_client_id, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = payload["metrics"]["string"]["glean.health.exception_state"].as_str();
    assert_eq!(Some("client-id-mismatch"), state);

    let exception_uuid = &payload["metrics"]["uuid"]["glean.health.recovered_client_id"].as_str();
    let mut buffer = Uuid::encode_buffer();
    let expected_uuid = &*new_uuid.hyphenated().encode_lower(&mut buffer);
    assert_eq!(&Some(expected_uuid), exception_uuid);
}

#[test]
fn c0ffee_in_db_gets_overwritten_by_stored_client_id() {
    let (file_client_id, temp) = {
        // Ensure we initialize once to get a client_id
        let (glean, temp) = new_glean(None);
        let file_client_id = clientid_from_file(temp.path()).unwrap();
        drop(glean);

        (file_client_id, temp)
    };

    // We modify the database and ONLY set the client id to c0ffee.
    {
        let path = temp.path().join("db");
        let rkv = Rkv::new::<rkv::backend::SafeMode>(&path).unwrap();
        let user_store = rkv.open_single("user", StoreOptions::create()).unwrap();

        // We know this.
        let client_id_key = "glean_client_info#client_id";

        let mut writer = rkv.write().unwrap();
        let encoded = bincode::serialize(&Metric::Uuid(String::from(
            "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0",
        )))
        .unwrap();
        let known_client_id = rkv::Value::Blob(&encoded);
        user_store
            .put(&mut writer, client_id_key, &known_client_id)
            .unwrap();
        writer.commit().unwrap();
    }

    let (glean, temp) = new_glean(Some(temp));

    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(file_client_id, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = payload["metrics"]["string"]["glean.health.exception_state"].as_str();
    assert_eq!(Some("c0ffee-in-db"), state);

    let mut buffer = Uuid::encode_buffer();
    let file_client_id = &*file_client_id.hyphenated().encode_lower(&mut buffer);

    let exception_uuid = &payload["metrics"]["uuid"]["glean.health.recovered_client_id"].as_str();
    assert_eq!(&Some(file_client_id), exception_uuid);

    let exception_uuid = &payload["client_info"]["client_id"].as_str();
    assert_eq!(&Some(file_client_id), exception_uuid);
    // We ensure we don't see the c0ffee ID either.
    let client_id = payload["client_info"]["client_id"].as_str().unwrap();
    assert_ne!("c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0", client_id);
}

#[test]
fn clientid_file_gets_deleted() {
    let (mut glean, temp) = new_glean(None);

    let path = temp.path().join("client_id.txt");
    assert!(path.exists());

    let file_client_id = clientid_from_file(temp.path());
    assert!(file_client_id.is_some());

    glean.set_upload_enabled(false);
    assert!(!path.exists());
}

#[test]
fn clientid_file_casing_doesnt_matter() {
    let (file_client_id, temp) = {
        // Ensure we initialize once to get a client_id
        let (glean, temp) = new_glean(None);
        let file_client_id = clientid_from_file(temp.path()).unwrap();
        drop(glean);

        (file_client_id, temp)
    };

    {
        let mut buffer = Uuid::encode_buffer();
        let uuid_str = file_client_id.hyphenated().encode_lower(&mut buffer);
        write_clientid_to_file_str(temp.path(), &uuid_str.to_uppercase());
    }

    let (glean, temp) = new_glean(Some(temp));

    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(file_client_id, db_client_id);

    glean.submit_ping_by_name("health", Some("pre_init"));
    let mut pending = get_queued_pings(temp.path()).unwrap();
    assert_eq!(1, pending.len());
    let payload = pending.pop().unwrap().1;

    let state = &payload["metrics"]["string"]["glean.health.exception_state"];
    assert_eq!(&serde_json::Value::Null, state);
}

#[test]
fn invalid_client_id_file_doesnt_crash() {
    let (glean, mut temp) = new_glean(None);
    drop(glean);

    let test_values = &[
        "",
        "abc",
        "e9f87afa-48b6-489e-841a-401522aaf1f", // one byte short
        "e9f87afa-48b6-489e-841a-401522aaf1ff\n", // explicit newline!
        "k9f87afa-48b6-489e-841a-401522aaf1ff", // k is not a valid char
        "\0\0\0",
    ];

    for value in test_values {
        write_clientid_to_file_str(temp.path(), value);

        let (glean, new_temp) = new_glean(Some(temp));
        drop(glean);
        temp = new_temp;
    }

    // Now testing with a directory in place.
    {
        let p = temp.path().join("client_id.txt");
        fs::remove_file(&p).unwrap();
        fs::create_dir_all(&p).unwrap();

        let (glean, new_temp) = new_glean(Some(temp));
        drop(glean);
        temp = new_temp;
    }

    drop(temp);
}

mod read_errors {
    use super::*;

    #[test]
    fn parse_error_is_reported() {
        let (glean, temp) = new_glean(None);
        drop(glean);

        write_clientid_to_file_str(temp.path(), "abc");

        let (glean, temp) = new_glean(Some(temp));

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]["parse"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }

    #[test]
    fn io_error_is_reported() {
        let (glean, temp) = new_glean(None);
        drop(glean);

        let p = temp.path().join("client_id.txt");
        fs::remove_file(&p).unwrap();
        fs::create_dir_all(&p).unwrap();

        let (glean, temp) = new_glean(Some(temp));

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]["io"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }

    #[cfg(unix)]
    #[test]
    fn permission_error_is_reported() {
        use std::os::unix::fs::PermissionsExt;

        let (glean, temp) = new_glean(None);
        drop(glean);

        let p = temp.path().join("client_id.txt");
        // Remove write permissions on the file.
        let attr = fs::metadata(&p).unwrap();
        let original_permissions = attr.permissions();
        let mut permissions = original_permissions.clone();
        // No permissions at all, no read, no write, for anyone.
        permissions.set_mode(0o0);
        fs::set_permissions(&p, permissions).unwrap();

        let (glean, temp) = new_glean(Some(temp));

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]
            ["permission-denied"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }

    #[test]
    fn c0ffee_is_reported() {
        let (glean, temp) = new_glean(None);
        drop(glean);

        write_clientid_to_file_str(temp.path(), "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0");

        let (glean, temp) = new_glean(Some(temp));

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]
            ["c0ffee-in-file"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }

    #[test]
    fn file_not_found_is_reported() {
        let (glean, temp) = new_glean(None);

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]
            ["file-not-found"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }
}

mod write_errors {
    use super::*;

    #[cfg(unix)]
    #[test]
    fn permission_error_is_reported() {
        let (glean, temp) = new_glean(None);
        drop(glean);

        // Force empty file, so that it will be written again later.
        write_clientid_to_file_str(temp.path(), "");

        let p = temp.path().join("client_id.txt");
        // Remove write permissions on the file.
        let attr = fs::metadata(&p).unwrap();
        let original_permissions = attr.permissions();
        let mut permissions = original_permissions.clone();
        // No permissions at all, no read, no write, for anyone.
        permissions.set_readonly(true);
        fs::set_permissions(&p, permissions).unwrap();

        let (glean, temp) = new_glean(Some(temp));

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]["parse"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_write_error"]
            ["permission-denied"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }

    #[cfg(unix)]
    #[test]
    fn permission_error_on_later_regeneration() {
        let (mut glean, temp) = new_glean(None);

        glean.submit_ping_by_name("health", Some("pre_init"));
        let pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());

        glean.set_upload_enabled(false);

        let path = temp.path().join("client_id.txt");
        assert!(!path.exists());

        // Force empty file, so that it will be written again later.
        write_clientid_to_file_str(temp.path(), "");

        let p = temp.path().join("client_id.txt");
        // Remove write permissions on the file.
        let attr = fs::metadata(&p).unwrap();
        let original_permissions = attr.permissions();
        let mut permissions = original_permissions.clone();
        // No permissions at all, no read, no write, for anyone.
        permissions.set_readonly(true);
        fs::set_permissions(&p, permissions).unwrap();

        glean.set_upload_enabled(true);

        glean.submit_ping_by_name("health", Some("pre_init"));
        let mut pending = get_queued_pings(temp.path()).unwrap();
        assert_eq!(1, pending.len());
        let payload = pending.pop().unwrap().1;

        let state = &payload["metrics"]["string"]["glean.health.exception_state"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = &payload["metrics"]["labeled_counter"]["glean.health.file_read_error"]["parse"];
        assert_eq!(&serde_json::Value::Null, state);

        let state = payload["metrics"]["labeled_counter"]["glean.health.file_write_error"]
            ["permission-denied"]
            .as_i64()
            .unwrap();
        assert_eq!(1, state);
    }
}
