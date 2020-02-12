// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Manages the pending pings queue and directory.
//!
//! * Keeps track of pending pings, loading any unsent ping from disk on startup;
//! * Exposes `get_next_ping` API for the platform layer to request next ping in line;
//! * Exposes `process_ping_upload_response` API to check the HTTP response from the ping upload
//!   and either delete the corresponding ping from disk or re-enqueue it for sending.

// !IMPORTANT!
// Remove the next line when this module's functionality is in the Glean object.
// This is here just to not have lint error for now.
#![allow(dead_code)]

use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::thread;

use log;
use serde_json::Value as JsonValue;

use request::PingRequest;
use util::process_pings_dir;

mod request;
mod util;

/// Manages the pending pings queue and directory.
#[derive(Debug)]
pub struct PingUploadManager {
    /// A FIFO queue storing a `PingRequest` for each pending ping.
    queue: Arc<RwLock<VecDeque<PingRequest>>>,
    /// A flag, signaling if we are done processing the pending pings directories.
    processed_pending_pings: Arc<AtomicBool>,
}

impl PingUploadManager {
    /// Create a new PingUploadManager.
    ///
    /// This will spawn a new thread and processes the pending pings folder
    /// filling up the queue with whatever pings are in there
    /// ordered by oldest to newest file modified.
    pub fn new(data_path: &str) -> Self {
        let queue = Arc::new(RwLock::new(VecDeque::new()));
        let processed_pending_pings = Arc::new(AtomicBool::new(false));

        let data_path = PathBuf::from_str(&data_path).expect("data_path must be a valid path.");
        let local_queue = queue.clone();
        let local_flag = processed_pending_pings.clone();
        let _ = thread::Builder::new()
            .name("glean.upload_manager.process_pings_directory".to_string())
            .spawn(move || match process_pings_dir(&data_path) {
                Ok(requests) => {
                    let mut local_queue = local_queue
                        .write()
                        .expect("Can't write to pending pings queue.");
                    local_queue.extend(requests.into_iter());
                    local_flag.store(true, Ordering::SeqCst);
                }
                Err(e) => log::info!("Error processing pending pings directories! {}", e),
            });

        Self {
            queue,
            processed_pending_pings,
        }
    }

    fn has_processed_pending_pings(&self) -> bool {
        self.processed_pending_pings.load(Ordering::SeqCst)
    }

    /// Creates a `PingRequest` and adds it to the queue.
    pub fn enqueue_ping(&self, uuid: &str, url: &str, body: JsonValue) {
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        let request = PingRequest::new(uuid, url, body);
        queue.push_back(request);
    }

    /// Clear pending pings queue.
    pub fn clear_ping_queue(&self) {
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        queue.clear();
    }

    /// Get the next `PingRequest` in queue.
    pub fn get_next_ping(&self) -> Option<PingRequest> {
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        queue.pop_front()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json::json;

    static UUID: &str = "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0";
    static URL: &str = "http://example.com";

    #[test]
    fn test_doesnt_error_when_there_are_no_pending_pings() {
        let data_dir = tempfile::tempdir().unwrap();
        let tmpname = data_dir.path().display().to_string();

        // Create a new upload_manager
        let upload_manager = PingUploadManager::new(&tmpname);

        // Try and get the next request
        let request = upload_manager.get_next_ping();

        // Verify request was not returned
        assert!(request.is_none());
    }

    #[test]
    fn test_returns_ping_request_when_there_is_one() {
        let data_dir = tempfile::tempdir().unwrap();
        let tmpname = data_dir.path().display().to_string();

        // Create a new upload_manager
        let upload_manager = PingUploadManager::new(&tmpname);

        // Enqueue a ping
        upload_manager.enqueue_ping(UUID, URL, json!({}));

        // Try and get the next request
        let request = upload_manager.get_next_ping();

        // Verify request was returned
        assert!(request.is_some());
    }

    #[test]
    fn test_returns_as_many_ping_request_as_there_are() {
        let data_dir = tempfile::tempdir().unwrap();
        let tmpname = data_dir.path().display().to_string();

        // Create a new upload_manager
        let upload_manager = PingUploadManager::new(&tmpname);

        // Enqueue a ping multiple times
        let n = 10;
        for _ in 0..n {
            upload_manager.enqueue_ping(UUID, URL, json!({}));
        }

        // Verify a request is returned for each submitted ping
        for _ in 0..n {
            assert!(upload_manager.get_next_ping().is_some());
        }

        // Verify that after all requests are returned, none are left
        assert!(upload_manager.get_next_ping().is_none());
    }

    #[test]
    fn clearing_the_queue_works_correctly() {
        let data_dir = tempfile::tempdir().unwrap();
        let tmpname = data_dir.path().display().to_string();

        // Create a new upload_manager
        let upload_manager = PingUploadManager::new(&tmpname);

        // Submit the ping multiple times
        let n = 10;
        for _ in 0..n {
            upload_manager.enqueue_ping(UUID, URL, json!({}));
        }

        // Clear the queue
        upload_manager.clear_ping_queue();

        // Verify there really isn't any ping in the queue.
        assert!(upload_manager.get_next_ping().is_none());
    }
}
