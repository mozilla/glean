// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! This integration test should model how the RLB is used when embedded in another Rust application
//! (e.g. FOG/Firefox Desktop).
//!
//! We write a single test scenario per file to avoid any state keeping across runs
//! (different files run as different processes).

mod common;

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Barrier;
use std::sync::Mutex;
use std::thread;
use std::thread::ThreadId;

use glean::net;
use glean::ConfigurationBuilder;

mod pings {
    use super::*;
    use glean::private::PingType;
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static custom_ping: Lazy<PingType> = Lazy::new(|| {
        common::PingBuilder::new("test-ping")
            .with_send_if_empty(true)
            .build()
    });
}

// Define a fake uploader that counts its number of invocations on a single thread
// and waits for a signal to continue (and signal that it is done).
#[derive(Debug)]
pub struct FakeUploader {
    barrier: Arc<Barrier>,
    counter: Arc<Mutex<HashMap<ThreadId, u32>>>,
}

impl net::PingUploader for FakeUploader {
    fn upload(&self, _upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        let mut map = self.counter.lock().unwrap();
        *map.entry(thread::current().id()).or_insert(0) += 1;

        // Wait for the sync.
        self.barrier.wait();

        // Signal that this uploader thread is done.
        net::UploadResult::done()
    }
}

#[test]
fn signaling_done() {
    common::enable_test_logging();

    // Create a custom configuration to use a fake uploader.
    let dir = tempfile::tempdir().unwrap();
    let tmpname = dir.path().to_path_buf();

    // We use a barrier to sync this test thread with the uploader thread.
    let barrier = Arc::new(Barrier::new(2));
    // We count how many times `upload` was invoked per thread.
    let call_count = Arc::new(Mutex::default());

    let cfg = ConfigurationBuilder::new(true, tmpname, "glean-signaling-done")
        .with_server_endpoint("invalid-test-host")
        .with_uploader(FakeUploader {
            barrier: Arc::clone(&barrier),
            counter: Arc::clone(&call_count),
        })
        .with_internal_pings(false)
        .build();

    common::initialize(cfg);

    // Submit the new ping.
    pings::custom_ping.submit(None);
    pings::custom_ping.submit(None);

    // Sync up with the upload thread.
    barrier.wait();

    // The uploader thread needs some CPU time to actually shut down.
    // We yield and still wait to make the scheduler give it that time.
    // Using just one of the ways wasn't reliable enough.
    std::thread::yield_now();
    std::thread::sleep(std::time::Duration::from_millis(100));

    // Submit another ping and wait for it to do work.
    pings::custom_ping.submit(None);

    // Sync up with the upload thread again.
    // This will not be the same thread as the one before (hopefully).
    barrier.wait();

    // No one's ever gonna wait for the uploader thread (the RLB doesn't store the handle to it),
    // so all we can do is hope it finishes within time.
    std::thread::sleep(std::time::Duration::from_millis(100));

    let map = call_count.lock().unwrap();
    assert_eq!(2, map.len(), "should have launched 2 uploader threads");
    for &count in map.values() {
        assert_eq!(1, count, "each thread should call upload only once");
    }
}
