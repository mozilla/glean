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

/// Get the file name from a path as a &str.
///
/// # Panics
///
/// Won't panic if not able to get file name.
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

/// Manages the pending pings directories.
#[derive(Debug, Clone)]
pub struct PingDirectoryManager {
    /// Path to the pending pings directory.
    data_path: PathBuf,
}

impl PingDirectoryManager {
    /// Creates a new directory manager.
    ///
    /// # Arguments
    ///
    /// * `data_path` - Path to the pending pings directory.
    pub fn new<P: Into<PathBuf>>(data_path: P) -> Self {
        Self {
            data_path: data_path.into(),
        }
    }

    /// Get the pending pings directory path.
    pub fn get_dir(&self) -> PathBuf {
        self.data_path.join("pending_pings")
    }

    /// Attempts to delete a ping file.
    ///
    /// ## Arguments
    ///
    /// * `uuid` - The UUID of the ping file to be deleted
    ///
    /// ## Panics
    ///
    /// Won't panic if unable to delete the file.
    pub fn delete_file(&self, uuid: &str) {
        let path = self.get_dir().join(uuid);
        match fs::remove_file(&path) {
            Err(e) => log::error!("Error deleting file {}. {}", path.display(), e),
            _ => log::info!("Files was deleted {}", path.display()),
        };
    }

    /// Reads a ping file and returns a `PingRequest` from it.
    ///
    /// If the file is not properly formatted, it will be deleted and `None` will be returned.
    ///
    /// ## Arguments
    ///
    /// * `uuid` - The UUID of the ping file to be processed
    pub fn process_file(&self, uuid: &str) -> Option<PingRequest> {
        let path = self.get_dir().join(uuid);
        let file = match File::open(path) {
            Ok(file) => file,
            Err(_) => return None,
        };

        log::info!("Processing ping: {}", uuid);

        // The way the ping file is structured,
        // first line should always have the url
        // and second line should have the body with the ping contents in JSON format
        let mut lines = BufReader::new(file).lines();
        if let (Some(Ok(url)), Some(Ok(body))) = (lines.next(), lines.next()) {
            if let Ok(parsed_body) = serde_json::from_str::<JsonValue>(&body) {
                return Some(PingRequest::new(uuid, &url, parsed_body));
            } else {
                log::warn!(
                    "Error processing ping file: {}. Can't parse ping contents as JSON.",
                    uuid
                );
            }
        } else {
            log::warn!(
                "Error processing ping file: {}. Ping file is not formatted as expected.",
                uuid
            );
        }
        self.delete_file(uuid);
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
    ///
    /// # Return value
    ///
    /// `Result<Vec<PingRequest>>` - see [`PingRequest`](struct.PingRequest.html) for more information.
    ///                              This will only be `Err` in case it is unable to read the pings directory.
    pub fn process_dir(&self) -> Result<Vec<PingRequest>> {
        let pings_dir = self.get_dir();

        log::info!("Processing persisted pings at {}", pings_dir.display());

        // Walk the pings directory and process each file in it,
        // deleting invalid ones and ignoring unreadable ones.
        // Create a vector of tuples: (modified_date, PingRequest)
        // using the contents and metadata of all valid files.
        let mut pending_pings: Vec<_> = pings_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| {
                let path = entry.path();
                if let Some(file_name) = get_file_name_as_str(&path) {
                    // Delete file if it doesn't match the pattern.
                    if Uuid::parse_str(file_name).is_err() {
                        log::warn!("Pattern mismatch. Deleting {}", path.display());
                        self.delete_file(file_name);
                        return None;
                    }
                    // In case we can't process the file we just ignore it.
                    if let Some(request) = self.process_file(file_name) {
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
        let directory_manager = PingDirectoryManager::new(dir.path());

        // Verify that processing the directory didn't panic
        assert!(directory_manager.process_dir().is_err());
    }

    #[test]
    fn test_creates_requests_correctly_from_valid_ping_file() {
        let (mut glean, dir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        let directory_manager = PingDirectoryManager::new(dir.path());

        // Try and process the pings folder
        let requests = directory_manager.process_dir().unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0].url.split('/').nth(3).unwrap();
        assert_eq!(request_ping_type, "test");
    }

    #[test]
    fn test_non_uuid_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        let directory_manager = PingDirectoryManager::new(dir.path());

        // Add non uuid file to pending_pings directory
        let not_uuid_path = directory_manager.get_dir().join("not-uuid-file-name.txt");
        File::create(&not_uuid_path).unwrap();

        // Try and process the pings folder
        let requests = directory_manager.process_dir().unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0].url.split('/').nth(3).unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!not_uuid_path.exists());
    }

    #[test]
    fn test_wrongly_formatted_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        let directory_manager = PingDirectoryManager::new(dir.path());

        // Create a file that will have wrong format contents
        let wrong_contents_file_path = directory_manager.get_dir().join(Uuid::new_v4().to_string());
        File::create(&wrong_contents_file_path).unwrap();

        // Try and process the pings folder
        let requests = directory_manager.process_dir().unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0].url.split('/').nth(3).unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!wrong_contents_file_path.exists());
    }

    #[test]
    fn test_non_json_ping_body_files_are_deleted_and_ignored() {
        let (mut glean, dir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping to populate the pending_pings directory
        glean.submit_ping(&ping_type, None).unwrap();

        let directory_manager = PingDirectoryManager::new(dir.path());

        // Create a file that will have wrong format contents
        let non_json_body_file_path = directory_manager.get_dir().join(Uuid::new_v4().to_string());
        let mut non_json_body_file = File::create(&non_json_body_file_path).unwrap();
        non_json_body_file
            .write_all(
                b"https://doc.rust-lang.org/std/fs/struct.File.html
                This is not JSON!!!!",
            )
            .unwrap();

        // Try and process the pings folder
        let requests = directory_manager.process_dir().unwrap();

        // Verify there is just the one request
        assert_eq!(requests.len(), 1);

        // Verify request was returned for the "test" ping
        let request_ping_type = requests[0].url.split('/').nth(3).unwrap();
        assert_eq!(request_ping_type, "test");

        // Verify that file was indeed deleted
        assert!(!non_json_body_file_path.exists());
    }
}
