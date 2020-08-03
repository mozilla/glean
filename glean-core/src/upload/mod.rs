// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Manages the pending pings queue and directory.
//!
//! * Keeps track of pending pings, loading any unsent ping from disk on startup;
//! * Exposes `get_upload_task` API for the platform layer to request next upload task;
//! * Exposes `process_ping_upload_response` API to check the HTTP response from the ping upload
//!   and either delete the corresponding ping from disk or re-enqueue it for sending.

use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, RwLock, RwLockWriteGuard};
use std::thread;
use std::time::{Duration, Instant};

use crate::error::ErrorKind;
use crate::{internal_metrics::UploadMetrics, Glean};
use directory::{PingDirectoryManager, PingPayloadsByDirectory};
pub use request::{HeaderMap, PingRequest};
pub use result::{ffi_upload_result, UploadResult};

mod directory;
mod request;
mod result;

/// The maximum recoverable failures allowed per uploading window.
///
/// Limiting this is necessary to avoid infinite loops on requesting upload tasks.
const MAX_RECOVERABLE_FAILURES_PER_UPLOADING_WINDOW: u32 = 3;

// The maximum size in bytes a ping body may have to be eligible for upload.
const PING_BODY_MAX_SIZE: usize = 1024 * 1024; // 1 MB

// The maximum size in byte the pending pings directory may have on disk.
const PENDING_PINGS_DIRECTORY_QUOTA: usize = 15 * 1024 * 1024; // 15 MB

#[derive(Debug)]
struct RateLimiter {
    /// The instant the current interval has started.
    started: Option<Instant>,
    /// The count for the current interval.
    count: u32,
    /// The duration of each interval.
    interval: Duration,
    /// The maximum count per interval.
    max_count: u32,
}

/// An enum to represent the current state of the RateLimiter.
#[derive(PartialEq)]
enum RateLimiterState {
    /// The RateLimiter has not reached the maximum count and is still incrementing.
    Incrementing,
    /// The RateLimiter has reached the maximum count for the  current interval.
    Throttled,
}

impl RateLimiter {
    pub fn new(interval: Duration, max_count: u32) -> Self {
        Self {
            started: None,
            count: 0,
            interval,
            max_count,
        }
    }

    fn reset(&mut self) {
        self.started = Some(Instant::now());
        self.count = 0;
    }

    /// The counter should reset if
    ///
    /// 1. It has never started;
    /// 2. It has been started more than the interval time ago;
    /// 3. Something goes wrong while trying to calculate the elapsed time since the last reset.
    fn should_reset(&self) -> bool {
        if self.started.is_none() {
            return true;
        }

        // Safe unwrap, we already stated that `self.started` is not `None` above.
        let elapsed = self.started.unwrap().elapsed();
        if elapsed > self.interval {
            return true;
        }

        false
    }

    /// Tries to increment the internal counter
    /// and returns the current state of the RateLimiter.
    pub fn get_state(&mut self) -> RateLimiterState {
        if self.should_reset() {
            self.reset();
        }

        if self.count == self.max_count {
            return RateLimiterState::Throttled;
        }

        self.count += 1;
        RateLimiterState::Incrementing
    }
}

/// When asking for the next ping request to upload,
/// the requester may receive one out of three possible tasks.
///
/// If new variants are added, this should be reflected in `glean-core/ffi/src/upload.rs` as well.
#[derive(PartialEq, Debug)]
pub enum PingUploadTask {
    /// A PingRequest popped from the front of the queue.
    /// See [`PingRequest`](struct.PingRequest.html) for more information.
    Upload(PingRequest),
    /// A flag signaling that the pending pings directories are not done being processed,
    /// thus the requester should wait and come back later.
    Wait,
    /// A flag signaling that requester doesn't need to request any more upload tasks at this moment.
    ///
    /// There are two possibilities for this scenario:
    /// * Pending pings queue is empty, no more pings to request;
    /// * Requester has reported more than MAX_RECOVERABLE_FAILURES_PER_UPLOADING_WINDOW
    ///   recoverable upload failures on the same uploading window[1]
    ///   and should stop requesting at this moment.
    ///
    /// [1]: An "uploading window" starts when a requester gets a new `PingUploadTask::Upload(PingRequest)`
    ///      response and finishes when they finally get a `PingUploadTask::Done` or `PingUploadTask::Wait` response.
    Done,
}

/// Manages the pending pings queue and directory.
#[derive(Debug)]
pub struct PingUploadManager {
    /// A FIFO queue storing a `PingRequest` for each pending ping.
    queue: RwLock<VecDeque<PingRequest>>,
    /// A manager for the pending pings directories.
    directory_manager: PingDirectoryManager,
    /// A flag signaling if we are done processing the pending pings directories.
    processed_pending_pings: Arc<AtomicBool>,
    /// A vector to store the pending pings processed off-thread.
    cached_pings: Arc<RwLock<PingPayloadsByDirectory>>,
    /// The number of upload failures for the current uploading window.
    recoverable_failure_count: AtomicU32,
    /// A ping counter to help rate limit the ping uploads.
    ///
    /// To keep resource usage in check,
    /// we may want to limit the amount of pings sent in a given interval.
    rate_limiter: Option<RwLock<RateLimiter>>,
    /// The name of the programming language used by the binding creating this instance of PingUploadManager.
    ///
    /// This will be used to build the value User-Agent header for each ping request.
    language_binding_name: String,
    /// Metrics related to ping uploading.
    upload_metrics: UploadMetrics,
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
    /// * `sync_scan` - Whether or not ping directory scanning should be synchronous.
    ///
    /// # Panics
    ///
    /// Will panic if unable to spawn a new thread.
    pub fn new<P: Into<PathBuf>>(
        data_path: P,
        language_binding_name: &str,
        sync_scan: bool,
    ) -> Self {
        let queue = RwLock::new(VecDeque::new());
        let directory_manager = PingDirectoryManager::new(data_path);

        let processed_pending_pings = Arc::new(AtomicBool::new(false));
        let cached_pings = Arc::new(RwLock::new(PingPayloadsByDirectory::default()));

        let local_manager = directory_manager.clone();
        let local_cached_pings = cached_pings.clone();
        let local_flag = processed_pending_pings.clone();
        let ping_scanning_thread = thread::Builder::new()
            .name("glean.ping_directory_manager.process_dir".to_string())
            .spawn(move || {
                let mut local_cached_pings = local_cached_pings
                    .write()
                    .expect("Can't write to pending pings cache.");
                local_cached_pings.extend(local_manager.process_dirs());
                local_flag.store(true, Ordering::SeqCst);
            })
            .expect("Unable to spawn thread to process pings directories.");

        if sync_scan {
            ping_scanning_thread
                .join()
                .expect("Unable to wait for startup ping processing to finish.");
        }

        Self {
            queue,
            directory_manager,
            processed_pending_pings,
            recoverable_failure_count: AtomicU32::new(0),
            cached_pings,
            rate_limiter: None,
            language_binding_name: language_binding_name.into(),
            upload_metrics: UploadMetrics::new(),
        }
    }

    fn has_processed_pings_dir(&self) -> bool {
        self.processed_pending_pings.load(Ordering::SeqCst)
    }

    fn recoverable_failure_count(&self) -> u32 {
        self.recoverable_failure_count.load(Ordering::SeqCst)
    }

    fn reset_recoverable_failure_count(&self) {
        self.recoverable_failure_count.store(0, Ordering::SeqCst);
    }

    /// Attempts to build a ping request from a ping file payload.
    ///
    /// Returns the `PingRequest` or `None` if unable to build,
    /// in which case it will delete the ping file and records an error.
    fn build_ping_request(
        &self,
        glean: &Glean,
        document_id: &str,
        path: &str,
        body: &str,
        headers: Option<HeaderMap>,
    ) -> Option<PingRequest> {
        let mut request = PingRequest::builder(&self.language_binding_name, PING_BODY_MAX_SIZE)
            .document_id(document_id)
            .path(path)
            .body(body);

        if let Some(headers) = headers {
            request = request.headers(headers);
        }

        match request.build() {
            Ok(request) => Some(request),
            Err(e) => {
                log::error!("Error trying to build ping request: {}", e);
                self.directory_manager.delete_file(&document_id);

                // Record the error.
                // Currently the only possible error is PingBodyOverflow.
                if let ErrorKind::PingBodyOverflow(s) = e.kind() {
                    self.upload_metrics
                        .discarded_exceeding_pings_size
                        .accumulate(glean, *s as u64 / 1024);
                }

                None
            }
        }
    }

    fn enqueue_ping(
        &self,
        glean: &Glean,
        document_id: &str,
        path: &str,
        body: &str,
        headers: Option<HeaderMap>,
    ) {
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");

        // Checks if a ping with this `document_id` is already enqueued.
        if queue
            .iter()
            .any(|request| request.document_id == document_id)
        {
            log::trace!(
                "Attempted to enqueue a duplicate ping {} at {}.",
                document_id,
                path
            );
            return;
        }

        log::trace!("Enqueuing ping {} at {}", document_id, path);
        if let Some(request) = self.build_ping_request(glean, document_id, path, body, headers) {
            queue.push_back(request)
        }
    }

    /// Enqueue pings that might have been cached.
    ///
    /// The size of the PENDING_PINGS_DIRECTORY directory will be calculated
    /// (by accumulating each pings size in that directory)
    /// and in case we extrapolate the quota, defined by the `quota` arg,
    /// outstanding pings get deleted and are not enqueued.
    ///
    /// The size of the DELETION_REQUEST_PINGS_DIRECTORY will not be calculated
    /// and no deletion-request pings will be deleted. Deletion request pings
    /// are not very common and usually don't contain any data,
    /// we don't expect that directory to ever reach quota.
    /// Most importantly, we don't want to ever delete deletion-request pings.
    ///
    /// Arguments
    ///
    /// * `glean` - The Glean object holding the database.
    /// * `quota` - The quota, in bytes, for the size of the pending pings directory.
    fn enqueue_cached_pings(&self, glean: &Glean, quota: usize) {
        let mut cached_pings = self
            .cached_pings
            .write()
            .expect("Can't write to pending pings cache.");

        // Enqueue all deletion-request pings, no limitations.
        let deletion_request_pings = cached_pings.deletion_request_pings.drain(..);
        for (_, (document_id, path, body, headers)) in deletion_request_pings {
            self.enqueue_ping(glean, &document_id, &path, &body, headers);
        }

        // Enqueue pending pings until we reach quota,
        // after that delete outstanding pings.
        let pending_pings = cached_pings.pending_pings.drain(..);
        let mut pending_pings_directory_size: usize = 0;
        let mut enqueueing = true;
        for (metadata, (document_id, path, body, headers)) in pending_pings {
            pending_pings_directory_size += metadata.len() as usize;
            if pending_pings_directory_size > quota {
                enqueueing = false;
            }

            if enqueueing {
                self.enqueue_ping(glean, &document_id, &path, &body, headers);
            } else if self.directory_manager.delete_file(&document_id) {
                self.upload_metrics
                    .deleted_pending_pings_after_quota_hit
                    .add(glean, 1);
            }
        }

        self.upload_metrics
            .pending_pings_directory_size
            .accumulate(glean, pending_pings_directory_size as u64);
    }

    /// Adds rate limiting capability to this upload manager. The rate limiter
    /// will limit the amount of calls to `get_upload_task` per interval.
    ///
    /// Setting will restart count and timer, in case there was a previous rate limiter set
    /// (e.g. if we have reached the current limit and call this function, we start counting again
    /// and the caller is allowed to asks for tasks).
    ///
    /// ## Arguments
    ///
    /// * `interval` - the amount of seconds in each rate limiting window.
    /// * `max_tasks` - the maximum amount of task requests allowed per interval.
    pub fn set_rate_limiter(&mut self, interval: u64, max_tasks: u32) {
        self.rate_limiter = Some(RwLock::new(RateLimiter::new(
            Duration::from_secs(interval),
            max_tasks,
        )));
    }

    /// Reads a ping file, creates a `PingRequest` and adds it to the queue.
    ///
    /// Duplicate requests won't be added.
    ///
    /// # Arguments
    ///
    /// * `glean` - The Glean object holding the database.
    /// * `document_id` - The UUID of the ping in question.
    pub fn enqueue_ping_from_file(&self, glean: &Glean, document_id: &str) {
        if let Some((doc_id, path, body, headers)) =
            self.directory_manager.process_file(document_id)
        {
            self.enqueue_ping(glean, &doc_id, &path, &body, headers)
        }
    }

    /// Clears the pending pings queue, leaves the deletion-request pings.
    pub fn clear_ping_queue(&self) -> RwLockWriteGuard<'_, VecDeque<PingRequest>> {
        log::trace!("Clearing ping queue");
        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");

        queue.retain(|ping| ping.is_deletion_request());
        log::trace!(
            "{} pings left in the queue (only deletion-request expected)",
            queue.len()
        );
        queue
    }

    fn get_upload_task_internal(&self, glean: &Glean, log_ping: bool) -> PingUploadTask {
        if !self.has_processed_pings_dir() {
            log::info!(
                "Tried getting an upload task, but processing is ongoing. Will come back later."
            );
            return PingUploadTask::Wait;
        }
        self.enqueue_cached_pings(glean, PENDING_PINGS_DIRECTORY_QUOTA);

        if self.recoverable_failure_count() >= MAX_RECOVERABLE_FAILURES_PER_UPLOADING_WINDOW {
            log::warn!(
                "Reached maximum recoverable failures for the current uploading window. You are done."
            );

            return PingUploadTask::Done;
        }

        let mut queue = self
            .queue
            .write()
            .expect("Can't write to pending pings queue.");
        match queue.front() {
            Some(request) => {
                if let Some(rate_limiter) = &self.rate_limiter {
                    let mut rate_limiter = rate_limiter
                        .write()
                        .expect("Can't write to the rate limiter.");
                    if rate_limiter.get_state() == RateLimiterState::Throttled {
                        log::info!(
                            "Tried getting an upload task, but we are throttled at the moment."
                        );
                        return PingUploadTask::Wait;
                    }
                }

                log::info!(
                    "New upload task with id {} (path: {})",
                    request.document_id,
                    request.path
                );

                if log_ping {
                    if let Some(body) = request.pretty_body() {
                        chunked_log_info(&request.path, &body);
                    } else {
                        chunked_log_info(&request.path, "<invalid ping payload>");
                    }
                }

                PingUploadTask::Upload(queue.pop_front().unwrap())
            }
            None => {
                log::info!("No more pings to upload! You are done.");
                PingUploadTask::Done
            }
        }
    }

    /// Gets the next `PingUploadTask`.
    ///
    /// ## Arguments
    ///
    /// * `glean` - The Glean object holding the database.
    /// * `log_ping` - Whether to log the ping before returning.
    ///
    /// # Return value
    ///
    /// `PingUploadTask` - see [`PingUploadTask`](enum.PingUploadTask.html) for more information.
    pub fn get_upload_task(&self, glean: &Glean, log_ping: bool) -> PingUploadTask {
        let task = self.get_upload_task_internal(glean, log_ping);
        if task == PingUploadTask::Done || task == PingUploadTask::Wait {
            self.reset_recoverable_failure_count()
        }

        task
    }

    /// Processes the response from an attempt to upload a ping.
    ///
    /// Based on the HTTP status of said response,
    /// the possible outcomes are:
    ///
    /// * **200 - 299 Success**
    ///   Any status on the 2XX range is considered a succesful upload,
    ///   which means the corresponding ping file can be deleted.
    ///   _Known 2XX status:_
    ///   * 200 - OK. Request accepted into the pipeline.
    ///
    /// * **400 - 499 Unrecoverable error**
    ///   Any status on the 4XX range means something our client did is not correct.
    ///   It is unlikely that the client is going to recover from this by retrying,
    ///   so in this case the corresponding ping file can also be deleted.
    ///   _Known 4XX status:_
    ///   * 404 - not found - POST/PUT to an unknown namespace
    ///   * 405 - wrong request type (anything other than POST/PUT)
    ///   * 411 - missing content-length header
    ///   * 413 - request body too large Note that if we have badly-behaved clients that
    ///           retry on 4XX, we should send back 202 on body/path too long).
    ///   * 414 - request path too long (See above)
    ///
    /// * **Any other error**
    ///   For any other error, a warning is logged and the ping is re-enqueued.
    ///   _Known other errors:_
    ///   * 500 - internal error
    ///
    /// # Note
    ///
    /// The disk I/O performed by this function is not done off-thread,
    /// as it is expected to be called off-thread by the platform.
    ///
    /// # Arguments
    ///
    /// * `glean` - The Glean object holding the database.
    /// * `document_id` - The UUID of the ping in question.
    /// * `status` - The HTTP status of the response.
    pub fn process_ping_upload_response(
        &self,
        glean: &Glean,
        document_id: &str,
        status: UploadResult,
    ) {
        use UploadResult::*;

        if let Some(label) = status.get_label() {
            let metric = self.upload_metrics.ping_upload_failure.get(label);
            metric.add(glean, 1);
        }

        match status {
            HttpStatus(status @ 200..=299) => {
                log::info!("Ping {} successfully sent {}.", document_id, status);
                self.directory_manager.delete_file(document_id);
            }

            UnrecoverableFailure | HttpStatus(400..=499) => {
                log::error!(
                    "Unrecoverable upload failure while attempting to send ping {}. Error was {:?}",
                    document_id,
                    status
                );
                self.directory_manager.delete_file(document_id);
            }

            RecoverableFailure | HttpStatus(_) => {
                log::error!(
                    "Recoverable upload failure while attempting to send ping {}, will retry. Error was {:?}",
                    document_id,
                    status
                );
                self.enqueue_ping_from_file(glean, &document_id);
                self.recoverable_failure_count
                    .fetch_add(1, Ordering::SeqCst);
            }
        };
    }
}

/// Split log message into chunks on Android.
#[cfg(target_os = "android")]
pub fn chunked_log_info(path: &str, payload: &str) {
    // Since the logcat ring buffer size is configurable, but it's 'max payload' size is not,
    // we must break apart long pings into chunks no larger than the max payload size of 4076b.
    // We leave some head space for our prefix.
    const MAX_LOG_PAYLOAD_SIZE_BYTES: usize = 4000;

    // If the length of the ping will fit within one logcat payload, then we can
    // short-circuit here and avoid some overhead, otherwise we must split up the
    // message so that we don't truncate it.
    if path.len() + payload.len() <= MAX_LOG_PAYLOAD_SIZE_BYTES {
        log::info!("Glean ping to URL: {}\n{}", path, payload);
        return;
    }

    // Otherwise we break it apart into chunks of smaller size,
    // prefixing it with the path and a counter.
    let mut start = 0;
    let mut end = MAX_LOG_PAYLOAD_SIZE_BYTES;
    let mut chunk_idx = 1;
    // Might be off by 1 on edge cases, but do we really care?
    let total_chunks = payload.len() / MAX_LOG_PAYLOAD_SIZE_BYTES + 1;

    while end < payload.len() {
        // Find char boundary from the end.
        // It's UTF-8, so it is within 4 bytes from here.
        for _ in 0..4 {
            if payload.is_char_boundary(end) {
                break;
            }
            end -= 1;
        }

        log::info!(
            "Glean ping to URL: {} [Part {} of {}]\n{}",
            path,
            chunk_idx,
            total_chunks,
            &payload[start..end]
        );

        // Move on with the string
        start = end;
        end = end + MAX_LOG_PAYLOAD_SIZE_BYTES;
        chunk_idx += 1;
    }

    // Print any suffix left
    if start < payload.len() {
        log::info!(
            "Glean ping to URL: {} [Part {} of {}]\n{}",
            path,
            chunk_idx,
            total_chunks,
            &payload[start..]
        );
    }
}

/// Log payload in one go (all other OS).
#[cfg(not(target_os = "android"))]
pub fn chunked_log_info(_path: &str, payload: &str) {
    log::info!("{}", payload)
}

#[cfg(test)]
mod test {
    use std::thread;
    use std::time::Duration;

    use uuid::Uuid;

    use super::UploadResult::*;
    use super::*;
    use crate::metrics::PingType;
    use crate::{tests::new_glean, PENDING_PINGS_DIRECTORY};

    const PATH: &str = "/submit/app_id/ping_name/schema_version/doc_id";

    #[test]
    fn doesnt_error_when_there_are_no_pending_pings() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Try and get the next request.
        // Verify request was not returned
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn returns_ping_request_when_there_is_one() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping
        upload_manager.enqueue_ping(&glean, &Uuid::new_v4().to_string(), PATH, "", None);

        // Try and get the next request.
        // Verify request was returned
        match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(_) => {}
            _ => panic!("Expected upload manager to return the next request!"),
        }
    }

    #[test]
    fn returns_as_many_ping_requests_as_there_are() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping multiple times
        let n = 10;
        for _ in 0..n {
            upload_manager.enqueue_ping(&glean, &Uuid::new_v4().to_string(), PATH, "", None);
        }

        // Verify a request is returned for each submitted ping
        for _ in 0..n {
            match upload_manager.get_upload_task(&glean, false) {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }

        // Verify that after all requests are returned, none are left
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn limits_the_number_of_pings_when_there_is_rate_limiting() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let mut upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Add a rate limiter to the upload mangager with max of 10 pings every 3 seconds.
        let secs_per_interval = 3;
        let max_pings_per_interval = 10;
        upload_manager.set_rate_limiter(secs_per_interval, 10);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping multiple times
        for _ in 0..max_pings_per_interval {
            upload_manager.enqueue_ping(&glean, &Uuid::new_v4().to_string(), PATH, "", None);
        }

        // Verify a request is returned for each submitted ping
        for _ in 0..max_pings_per_interval {
            match upload_manager.get_upload_task(&glean, false) {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }

        // Enqueue just one more ping.
        // We should still be within the default rate limit time.
        upload_manager.enqueue_ping(&glean, &Uuid::new_v4().to_string(), PATH, "", None);

        // Verify that we are indeed told to wait because we are at capacity
        assert_eq!(
            PingUploadTask::Wait,
            upload_manager.get_upload_task(&glean, false)
        );

        thread::sleep(Duration::from_secs(secs_per_interval));

        match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(_) => {}
            _ => panic!("Expected upload manager to return the next request!"),
        }
    }

    #[test]
    fn clearing_the_queue_works_correctly() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Enqueue a ping multiple times
        for _ in 0..10 {
            upload_manager.enqueue_ping(&glean, &Uuid::new_v4().to_string(), PATH, "", None);
        }

        // Clear the queue
        drop(upload_manager.clear_ping_queue());

        // Verify there really isn't any ping in the queue
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn clearing_the_queue_doesnt_clear_deletion_request_pings() {
        let (mut glean, _) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping multiple times
        let n = 10;
        for _ in 0..n {
            glean.submit_ping(&ping_type, None).unwrap();
        }

        glean
            .internal_pings
            .deletion_request
            .submit(&glean, None)
            .unwrap();

        // Clear the queue
        drop(glean.upload_manager.clear_ping_queue());

        let upload_task = glean.get_upload_task();
        match upload_task {
            PingUploadTask::Upload(request) => assert!(request.is_deletion_request()),
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify there really isn't any other pings in the queue
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn fills_up_queue_successfully_from_disk() {
        let (mut glean, tmpdir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping multiple times
        let n = 10;
        for _ in 0..n {
            glean.submit_ping(&ping_type, None).unwrap();
        }

        // Create a new upload manager pointing to the same data_path as the glean instance.
        let upload_manager = PingUploadManager::new(tmpdir.path(), "Rust", true);

        // Verify the requests were properly enqueued
        for _ in 0..n {
            match upload_manager.get_upload_task(&glean, false) {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }

        // Verify that after all requests are returned, none are left
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn processes_correctly_success_upload_response() {
        let (mut glean, dir) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit a ping
        glean.submit_ping(&ping_type, None).unwrap();

        // Get the pending ping directory path
        let pending_pings_dir = dir.path().join(PENDING_PINGS_DIRECTORY);

        // Get the submitted PingRequest
        match glean.get_upload_task() {
            PingUploadTask::Upload(request) => {
                // Simulate the processing of a sucessfull request
                let document_id = request.document_id;
                glean.process_ping_upload_response(&document_id, HttpStatus(200));
                // Verify file was deleted
                assert!(!pending_pings_dir.join(document_id).exists());
            }
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify that after request is returned, none are left
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn processes_correctly_client_error_upload_response() {
        let (mut glean, dir) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit a ping
        glean.submit_ping(&ping_type, None).unwrap();

        // Get the pending ping directory path
        let pending_pings_dir = dir.path().join(PENDING_PINGS_DIRECTORY);

        // Get the submitted PingRequest
        match glean.get_upload_task() {
            PingUploadTask::Upload(request) => {
                // Simulate the processing of a client error
                let document_id = request.document_id;
                glean.process_ping_upload_response(&document_id, HttpStatus(404));
                // Verify file was deleted
                assert!(!pending_pings_dir.join(document_id).exists());
            }
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify that after request is returned, none are left
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn processes_correctly_server_error_upload_response() {
        let (mut glean, _) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit a ping
        glean.submit_ping(&ping_type, None).unwrap();

        // Get the submitted PingRequest
        match glean.get_upload_task() {
            PingUploadTask::Upload(request) => {
                // Simulate the processing of a client error
                let document_id = request.document_id;
                glean.process_ping_upload_response(&document_id, HttpStatus(500));
                // Verify this ping was indeed re-enqueued
                match glean.get_upload_task() {
                    PingUploadTask::Upload(request) => {
                        assert_eq!(document_id, request.document_id);
                    }
                    _ => panic!("Expected upload manager to return the next request!"),
                }
            }
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify that after request is returned, none are left
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn processes_correctly_unrecoverable_upload_response() {
        let (mut glean, dir) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit a ping
        glean.submit_ping(&ping_type, None).unwrap();

        // Get the pending ping directory path
        let pending_pings_dir = dir.path().join(PENDING_PINGS_DIRECTORY);

        // Get the submitted PingRequest
        match glean.get_upload_task() {
            PingUploadTask::Upload(request) => {
                // Simulate the processing of a client error
                let document_id = request.document_id;
                glean.process_ping_upload_response(&document_id, UnrecoverableFailure);
                // Verify file was deleted
                assert!(!pending_pings_dir.join(document_id).exists());
            }
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify that after request is returned, none are left
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);
    }

    #[test]
    fn new_pings_are_added_while_upload_in_progress() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        let doc1 = Uuid::new_v4().to_string();
        let path1 = format!("/submit/app_id/test-ping/1/{}", doc1);

        let doc2 = Uuid::new_v4().to_string();
        let path2 = format!("/submit/app_id/test-ping/1/{}", doc2);

        // Enqueue a ping
        upload_manager.enqueue_ping(&glean, &doc1, &path1, "", None);

        // Try and get the first request.
        let req = match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(req) => req,
            _ => panic!("Expected upload manager to return the next request!"),
        };
        assert_eq!(doc1, req.document_id);

        // Schedule the next one while the first one is "in progress"
        upload_manager.enqueue_ping(&glean, &doc2, &path2, "", None);

        // Mark as processed
        upload_manager.process_ping_upload_response(&glean, &req.document_id, HttpStatus(200));

        // Get the second request.
        let req = match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(req) => req,
            _ => panic!("Expected upload manager to return the next request!"),
        };
        assert_eq!(doc2, req.document_id);

        // Mark as processed
        upload_manager.process_ping_upload_response(&glean, &req.document_id, HttpStatus(200));

        // ... and then we're done.
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn uploader_sync_init() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager, with a synchronous ping dir scan.
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", true);

        // Since the scan was synchronous and the directory was empty,
        // we expect the upload task to always be `Done`.
        assert_eq!(
            PingUploadTask::Done,
            upload_manager.get_upload_task(&glean, false)
        )
    }

    #[test]
    fn adds_debug_view_header_to_requests_when_tag_is_set() {
        let (mut glean, _) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        glean.set_debug_view_tag("valid-tag");

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit a ping
        glean.submit_ping(&ping_type, None).unwrap();

        // Get the submitted PingRequest
        match glean.get_upload_task() {
            PingUploadTask::Upload(request) => {
                assert_eq!(request.headers.get("X-Debug-ID").unwrap(), "valid-tag")
            }
            _ => panic!("Expected upload manager to return the next request!"),
        }
    }

    #[test]
    fn duplicates_are_not_enqueued() {
        let (glean, _) = new_glean(None);

        // Create a new upload_manager
        let dir = tempfile::tempdir().unwrap();
        let upload_manager = PingUploadManager::new(dir.path(), "Testing", false);

        // Wait for processing of pending pings directory to finish.
        while upload_manager.get_upload_task(&glean, false) == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        let doc_id = Uuid::new_v4().to_string();
        let path = format!("/submit/app_id/test-ping/1/{}", doc_id);

        // Try to enqueue a ping with the same doc_id twice
        upload_manager.enqueue_ping(&glean, &doc_id, &path, "", None);
        upload_manager.enqueue_ping(&glean, &doc_id, &path, "", None);

        // Get a task once
        match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(_) => {}
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // There should be no more queued tasks
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }

    #[test]
    fn maximum_of_recoverable_errors_is_enforced_for_uploading_window() {
        let (mut glean, _) = new_glean(None);

        // Wait for processing of pending pings directory to finish.
        while glean.get_upload_task() == PingUploadTask::Wait {
            thread::sleep(Duration::from_millis(10));
        }

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping multiple times
        let n = 5;
        for _ in 0..n {
            glean.submit_ping(&ping_type, None).unwrap();
        }

        // Return the max recoverable error failures in a row
        for _ in 0..MAX_RECOVERABLE_FAILURES_PER_UPLOADING_WINDOW {
            match glean.get_upload_task() {
                PingUploadTask::Upload(req) => {
                    glean.process_ping_upload_response(&req.document_id, RecoverableFailure)
                }
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }

        // Verify that after returning the max amount of recoverable failures,
        // we are done even though we haven't gotten all the enqueued requests.
        assert_eq!(glean.get_upload_task(), PingUploadTask::Done);

        // Verify all requests are returned when we try again.
        for _ in 0..n {
            match glean.get_upload_task() {
                PingUploadTask::Upload(_) => {}
                _ => panic!("Expected upload manager to return the next request!"),
            }
        }
    }

    #[test]
    fn quota_is_enforced_when_enqueueing_cached_pings() {
        let (mut glean, tmpdir) = new_glean(None);

        // Register a ping for testing
        let ping_type = PingType::new("test", true, /* send_if_empty */ true, vec![]);
        glean.register_ping_type(&ping_type);

        // Submit the ping multiple times
        let n = 10;
        for _ in 0..n {
            glean.submit_ping(&ping_type, None).unwrap();
        }

        // Create a new upload manager pointing to the same data_path as the glean instance.
        let upload_manager = PingUploadManager::new(tmpdir.path(), "Rust", true);

        // Enqueue cached pings and set the quota to just a little over the size on an empty ping file.
        // This way we can check that one ping is kept and all others are deleted.
        //
        // From manual testing I figured out an empty ping file is 324bytes,
        // I am setting this a little over just so that minor changes to the ping structure
        // don't mmediatelly break this.
        upload_manager.enqueue_cached_pings(&glean, 500);

        // Get a task once
        // One ping should have been enqueued.
        match upload_manager.get_upload_task(&glean, false) {
            PingUploadTask::Upload(_) => {}
            _ => panic!("Expected upload manager to return the next request!"),
        }

        // Verify that no other requests were returned,
        // they should all have been deleted because pending pings quota was hit.
        assert_eq!(
            upload_manager.get_upload_task(&glean, false),
            PingUploadTask::Done
        );
    }
}
