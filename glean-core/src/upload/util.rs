// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Pings directory processing utilities.

use std::cmp::Ordering;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use log;
use serde_json::Value as JsonValue;
use uuid::Uuid;

use super::PingRequest;
use crate::Result;

/// Attempt to delete a file.
///
/// This won't panic if not able to delete, just log.
fn delete_file(path: &Path) {
    match fs::remove_file(path) {
        Err(e) => log::error!("Error deleting file {}. {}", path.display(), e),
        _ => log::info!("Deleted file {}", path.display()),
    };
}

/// Get the file name from a path as a &str.
///
/// This won't panic if not able to get file name, just log.
fn get_file_name_as_str(path: &Path) -> Option<&str> {
    match path.file_name() {
        None => {
            log::warn!("Error getting file name from path: {}", path.display());
            None
        }
        Some(file_name) => {
            let file_name = file_name.to_str();
            if file_name.is_none() {
                log::warn!("File name is not valid unicode: {}", path.display());
            }
            file_name
        }
    }
}

/// Get the pending pings directory path.
fn get_pings_dir(data_path: &Path) -> PathBuf {
    data_path.join("pending_pings")
}

/// Reads a ping file and returns a `PingRequest` from it.
///
/// If the file is not properly formatted, it will be deleted.
///
/// ## Panics
///
/// Will panic if unable to read the file.
fn process_ping_file(uuid: &str, path: &Path) -> Option<PingRequest> {
    let file = File::open(path).expect("Should be able to read ping file.");
    let mut lines = BufReader::new(file).lines();
    // The way the ping file is structured,
    // first line should always have the url
    // and second line should have the body with the ping contents in JSON format
    if let (Some(Ok(url)), Some(Ok(body))) = (lines.next(), lines.next()) {
        if let Ok(parsed_body) = serde_json::from_str::<JsonValue>(&body) {
            return Some(PingRequest::new(uuid, &url, parsed_body));
        } else {
            log::warn!("Can't parse ping contents as JSON.");
        }
    } else {
        log::warn!("Ping file is not formatted as expected.");
    }
    delete_file(path);
    None
}

/// Process the pings directory and return a vector of `PingRequest`s
/// corresponding to each valid ping file in the directory.
/// This vector will be ordered by file `modified_date`.
///
/// Any files that don't match the UUID regex will be deleted
/// to prevent files from polluting the pings directory.
///
/// Files that are not correctly formatted will also be deleted.
pub fn process_pings_dir(data_path: &Path) -> Result<Vec<PingRequest>> {
    let pings_dir = get_pings_dir(data_path);
    let mut pending_pings: Vec<_> = pings_dir
        .read_dir()?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if let Some(file_name) = get_file_name_as_str(&path) {
                // Delete file if it doesn't match the pattern.
                if Uuid::parse_str(file_name).is_err() {
                    log::warn!("Pattern mismatch {}", path.display());
                    delete_file(&path);
                    return None;
                }
                // In case we can't process the file we just ignore it.
                if let Some(request) = process_ping_file(file_name, &path) {
                    // Get the modified date of the file, which will later be used
                    // for sorting the resulting vector.
                    let modified_date = fs::metadata(&path).and_then(|data| data.modified());
                    return Some((modified_date, request));
                }
            };
            None
        })
        .collect();

    // Sort by `modified_date`.
    pending_pings.sort_by(|(a, _), (b, _)| {
        // We might not be able to get the modified date for a given file,
        // in which case we just put it at the end.
        if let (Ok(a), Ok(b)) = (a, b) {
            a.partial_cmp(b).unwrap()
        } else {
            Ordering::Less
        }
    });

    // Return the vector leaving only the `PingRequest`s in it
    Ok(pending_pings
        .into_iter()
        .map(|(_, request)| request)
        .collect())
}

#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::prelude::*;
    use uuid::Uuid;

    use super::*;
    use crate::metrics::PingType;
    use crate::tests::new_glean;

    #[test]
    fn test_doesnt_panic_if_no_pending_pings_directory() {
        let dir = tempfile::tempdir().unwrap();
        assert!(process_pings_dir(&dir.path()).is_err());
    }

    #[test]
    fn test_creates_requests_correctly_from_valid_ping_file() {
        let (mut glean, dir) = new_glean(None);
        let data_path = dir.path();

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        // Try and process the pings folder
        let requests = process_pings_dir(&data_path).unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0]
            .body
            .get("ping_info")
            .and_then(|value| value.get("ping_type"))
            .unwrap();
        assert_eq!(request_ping_type, "test");
    }

    #[test]
    fn test_non_uuid_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);
        let data_path = dir.path();

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        // Add non uuid file to pending_pings directory
        let not_uuid_path = get_pings_dir(&data_path).join("not-uuid-file-name.txt");
        File::create(&not_uuid_path).unwrap();

        // Try and process the pings folder
        let requests = process_pings_dir(&data_path).unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0]
            .body
            .get("ping_info")
            .and_then(|value| value.get("ping_type"))
            .unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!not_uuid_path.exists());
    }

    #[test]
    fn test_wrongly_formatted_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);
        let data_path = dir.path();

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        // Create a file that will have wrong format contents
        let wrong_contents_file_path = get_pings_dir(&data_path).join(Uuid::new_v4().to_string());
        File::create(&wrong_contents_file_path).unwrap();

        // Try and process the pings folder
        let requests = process_pings_dir(&data_path).unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0]
            .body
            .get("ping_info")
            .and_then(|value| value.get("ping_type"))
            .unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!wrong_contents_file_path.exists());
    }

    #[test]
    fn test_non_json_ping_body_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);
        let data_path = dir.path();

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        // Create a file that will have wrong format contents
        let non_json_body_file_path = get_pings_dir(&data_path).join(Uuid::new_v4().to_string());
        let mut non_json_body_file = File::create(&non_json_body_file_path).unwrap();
        non_json_body_file
            .write_all(
                b"https://doc.rust-lang.org/std/fs/struct.File.html
                This is not JSON!!!!",
            )
            .unwrap();

        // Try and process the pings folder
        let requests = process_pings_dir(&data_path).unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0]
            .body
            .get("ping_info")
            .and_then(|value| value.get("ping_type"))
            .unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!non_json_body_file_path.exists());
    }
}
