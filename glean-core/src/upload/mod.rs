// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Manages the pending pings queue and directory.
//!
//! * Keeps track of pending pings, loading any unsent ping from disk on startup;
//! * Exposes `get_upload_task` API for the platform layer to request next upload task;
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

/// When asking for the next ping request to upload,
/// the requester may receive one out of three possible tasks.
#[derive(PartialEq, Debug)]
pub enum PingUploadTask {
    /// A PingRequest popped from the front of the queue.
    /// See [`PingRequest`](struct.PingRequest.html) for more information.
    Upload(PingRequest),
    /// A flag signaling that the pending pings directories are not done being processed,
    /// thus the requester should wait and come back later.
    Wait,
    /// A flag signaling that the pending pings queue is empty and requester is done.
    Done,
}

/// Manages the pending pings queue and directory.
#[derive(Debug)]
pub struct PingUploadManager {
    /// A FIFO queue storing a `PingRequest` for each pending ping.
    queue: Arc<RwLock<VecDeque<PingRequest>>>,
    /// A flag signaling if we are done processing the pending pings directories.
    ///
    /// This does not indicate that processing of the directory was successful,
    /// only that it did happen.
    processed_pending_pings: Arc<AtomicBool>,
}

impl PingUploadManager {
    /// Create a new PingUploadManager.
    ///
    /// Spawns a new thread and processes the pending pings directory,
    /// filling up the queue with whatever pings are in there.
    ///
    /// # Arguments
    ///
    /// * `data_path` - Path to the pending pings directory.
    ///
    /// # Panics
    ///
    /// Will panic if unable to spawn a new thread.
    pub fn new(data_path: &str) -> Self {
        let queue = Arc::new(RwLock::new(VecDeque::new()));
        let processed_pending_pings = Arc::new(AtomicBool::new(false));

        let data_path = PathBuf::from_str(data_path).expect("data_path must be a valid path.");
        let local_queue = queue.clone();
        let local_flag = processed_pending_pings.clone();
        let _ = thread::Builder::new()
            .name("glean.upload_manager.process_pings_dir".to_string())
            .spawn(move || {
                match process_pings_dir(&data_path) {
                    Ok(requests) => {
                        let mut local_queue = local_queue
                            .write()
                            .expect("Can't write to pending pings queue.");
                        local_queue.extend(requests.into_iter());
                    }
                    Err(e) => log::info!("Error processing pending pings directories! {}", e),
                }
                local_flag.store(true, Ordering::SeqCst);
            })
            .expect("Unable to spawn thread to process pings directories.");

        Self {
            queue,
            processed_pending_pings,
        }
    }

    fn has_processed_pings_dir(&self) -> bool {
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

    /// Clears the pending pings queue.
    pub fn clear_ping_queue(&self) {
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        queue.clear();
    }

    /// Gets the next `PingUploadTask`.
    ///
    /// # Return value
    ///
    /// `PingUploadTask` - see [`PingUploadTask`](enum.PingUploadTask.html) for more information.
    pub fn get_upload_task(&self) -> PingUploadTask {
        if !self.has_processed_pings_dir() {
            return PingUploadTask::Wait;
        }

        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        match queue.pop_front() {
            Some(request) => PingUploadTask::Upload(request),
            None => PingUploadTask::Done,
        }
    }
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    use serde_json::json;

    use super::*;
    use crate::metrics::PingType;
    use crate::tests::new_glean;

    const UUID: &str = "40e31919-684f-43b0-a5aa-e15c2d56a674"; // Just a random UUID.
    const URL: &str = "http://example.com";

    #[test]
    fn test_doesnt_error_when_there_are_no_pending_pings() {
        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let tmpname = dir.path().display().to_string();
        let upload_manager = PingUploadManager::new(&tmpname);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Try and get the next request.
        // Verify request was not returned
        assert_eq!(upload_manager.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn test_returns_ping_request_when_there_is_one() {
        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let tmpname = dir.path().display().to_string();
        let upload_manager = PingUploadManager::new(&tmpname);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping
        upload_manager.enqueue_ping(UUID, URL, json!({}));

        // Try and get the next request.
        // Verify request was returned
        match upload_manager.get_upload_task() {
            PingUploadTask::Upload(_) => {}
            _ => panic!("Expected upload manager to return the next request!"),
        }
    }

    #[test]
    fn test_returns_as_many_ping_requests_as_there_are() {
        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let tmpname = dir.path().display().to_string();
        let upload_manager = PingUploadManager::new(&tmpname);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping multiple times
        let n = 10;
        for _ in 0..n {
            upload_manager.enqueue_ping(UUID, URL, json!({}));
        }

        // Verify a request is returned for each submitted ping
        for _ in 0..n {
            match upload_manager.get_upload_task() {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }

        // Verify that after all requests are returned, none are left
        assert_eq!(upload_manager.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn test_clearing_the_queue_works_correctly() {
        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let tmpname = dir.path().display().to_string();
        let upload_manager = PingUploadManager::new(&tmpname);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping multiple times
        for _ in 0..10 {
            upload_manager.enqueue_ping(UUID, URL, json!({}));
        }

        // Clear the queue
        upload_manager.clear_ping_queue();

        // Verify there really isn't any ping in the queue
        assert_eq!(upload_manager.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn test_fills_up_queue_successfully_from_disk() {
        let (mut glean, dir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping multiple times
        let n = 10;
        for _ in 0..n {
            glean.submit_ping(&ping_type, None).unwrap();
        }

        // Create a new upload_manager
        let data_path = dir.path().to_str().unwrap();
        let upload_manager = PingUploadManager::new(data_path);

        // Wait for processing of pending pings directory to finish.
        let mut upload_task = upload_manager.get_upload_task();
        while upload_task == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
            upload_task = upload_manager.get_upload_task();
        }

        // Verify the requests were properly enqueued
        for _ in 0..n {
            match upload_task {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }

            upload_task = upload_manager.get_upload_task();
        }

        // Verify that after all requests are returned, none are left
        assert_eq!(upload_manager.get_upload_task(), PingUploadTask::Done);
    }
}
