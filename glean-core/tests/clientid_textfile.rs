// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

mod common;
use crate::common::*;

use std::fs;
use std::path::Path;

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
    let path = data_path.join("client_id.txt");
    let mut buffer = Uuid::encode_buffer();
    let uuid_str = uuid.hyphenated().encode_lower(&mut buffer);
    fs::write(path, uuid_str.as_bytes()).ok()?;
    Some(())
}

#[test]
fn writes_clientid_file() {
    let (glean, temp) = new_glean(None);

    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    let file_client_id = clientid_from_file(temp.path()).unwrap();

    assert_eq!(file_client_id, db_client_id);
}

#[test]
fn reused_clientid_from_file() {
    let temp =  tempfile::tempdir().unwrap();
    let new_uuid = Uuid::new_v4();

    write_clientid_to_file(temp.path(), &new_uuid).unwrap();

    let (glean, _temp) = new_glean(Some(temp));
    let db_client_id = clientid_metric().get_value(&glean, None).unwrap();
    assert_eq!(new_uuid, db_client_id);
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
}
