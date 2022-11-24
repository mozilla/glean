// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A global dispatcher queue.
//!
//! # Example - Global Dispatch queue
//!
//! The global dispatch queue is pre-configured with a maximum queue size of 100 tasks.
//!
//! ```rust,ignore
//! // Ensure the dispatcher queue is being worked on.
//! dispatcher::flush_init();
//!
//! dispatcher::launch(|| {
//!     println!("Executing expensive task");
//!     // Run your expensive task in a separate thread.
//! });
//!
//! dispatcher::launch(|| {
//!     println!("A second task that's executed sequentially, but off the main thread.");
//! });
//! ```

use crossbeam_channel::SendError;
use thiserror::Error;

pub use global::*;

pub(crate) mod global;

#[cfg(not(feature = "native-dispatcher"))]
mod imp;
#[cfg(feature = "native-dispatcher")]
mod native;

#[cfg(not(feature = "native-dispatcher"))]
use imp::*;
#[cfg(feature = "native-dispatcher")]
use native::*;

/// The error returned from operations on the dispatcher
#[derive(Error, Debug, PartialEq, Eq)]
pub enum DispatchError {
    /// The worker panicked while running a task
    #[allow(dead_code)] // only used in `imp`
    #[error("The worker panicked while running a task")]
    WorkerPanic,

    /// Maximum queue size reached
    #[allow(dead_code)] // only used by `imp`
    #[error("Maximum queue size reached")]
    QueueFull,

    /// Pre-init buffer was already flushed
    #[error("Pre-init buffer was already flushed")]
    AlreadyFlushed,

    /// Failed to send command to worker thread
    #[error("Failed to send command to worker thread")]
    SendError,

    /// Failed to receive from channel
    #[error("Failed to receive from channel")]
    RecvError(#[from] crossbeam_channel::RecvError),
}

impl<T> From<SendError<T>> for DispatchError {
    fn from(_: SendError<T>) -> Self {
        DispatchError::SendError
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
    use std::sync::{Arc, Mutex};
    use std::{thread, time::Duration};

    fn enable_test_logging() {
        // When testing we want all logs to go to stdout/stderr by default,
        // without requiring each individual test to activate it.
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn tasks_run_off_the_main_thread() {
        enable_test_logging();

        let main_thread_id = thread::current().id();
        let thread_canary = Arc::new(AtomicBool::new(false));

        let dispatcher = Dispatcher::new(100);

        // Force the Dispatcher out of the pre-init queue mode.
        dispatcher
            .guard()
            .flush_init()
            .expect("Failed to get out of preinit queue mode");

        let canary_clone = thread_canary.clone();
        dispatcher
            .guard()
            .launch(move || {
                assert!(thread::current().id() != main_thread_id);
                // Use the canary bool to make sure this is getting called before
                // the test completes.
                assert!(!canary_clone.load(Ordering::SeqCst));
                canary_clone.store(true, Ordering::SeqCst);
            })
            .expect("Failed to dispatch the test task");

        dispatcher.guard().block_on_queue();
        assert!(thread_canary.load(Ordering::SeqCst));
        assert_eq!(main_thread_id, thread::current().id());
    }

    #[test]
    fn launch_correctly_adds_tasks_to_preinit_queue() {
        enable_test_logging();

        let thread_canary = Arc::new(AtomicU8::new(0));

        let dispatcher = Dispatcher::new(100);

        // Add 3 tasks to queue each one increasing thread_canary by 1 to
        // signal that the tasks ran.
        for _ in 0..3 {
            let canary_clone = thread_canary.clone();
            dispatcher
                .guard()
                .launch(move || {
                    canary_clone.fetch_add(1, Ordering::SeqCst);
                })
                .expect("Failed to dispatch the test task");
        }

        // Ensure that no task ran.
        assert_eq!(0, thread_canary.load(Ordering::SeqCst));

        // Flush the queue and wait for the tasks to complete.
        dispatcher
            .guard()
            .flush_init()
            .expect("Failed to get out of preinit queue mode");
        // Validate that we have the expected canary value.
        assert_eq!(3, thread_canary.load(Ordering::SeqCst));
    }

    #[test]
    fn preinit_tasks_are_processed_after_flush() {
        enable_test_logging();

        let dispatcher = Dispatcher::new(10);

        let result = Arc::new(Mutex::new(vec![]));
        for i in 1..=5 {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    result.lock().unwrap().push(i);
                })
                .unwrap();
        }

        result.lock().unwrap().push(0);
        dispatcher.guard().flush_init().unwrap();
        for i in 6..=10 {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    result.lock().unwrap().push(i);
                })
                .unwrap();
        }

        dispatcher.guard().block_on_queue();

        // This additionally checks that tasks were executed in order.
        assert_eq!(
            &*result.lock().unwrap(),
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        );
    }

    #[test]
    fn tasks_after_shutdown_are_not_processed() {
        enable_test_logging();

        let mut dispatcher = Dispatcher::new(10);

        let result = Arc::new(Mutex::new(vec![]));

        dispatcher.guard().flush_init().unwrap();

        dispatcher.guard().shutdown().unwrap();
        {
            let result = Arc::clone(&result);
            // This might fail because the shutdown is quick enough,
            // or it might succeed and still send the task.
            // In any case that task should not be executed.
            let _ = dispatcher.guard().launch(move || {
                result.lock().unwrap().push(0);
            });
        }

        dispatcher.join().unwrap();

        assert!(result.lock().unwrap().is_empty());
    }

    #[test]
    fn preinit_buffer_fills_up() {
        enable_test_logging();

        let dispatcher = Dispatcher::new(5);

        let result = Arc::new(Mutex::new(vec![]));

        for i in 1..=5 {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    result.lock().unwrap().push(i);
                })
                .unwrap();
        }

        {
            let result = Arc::clone(&result);
            let err = dispatcher.guard().launch(move || {
                result.lock().unwrap().push(10);
            });
            assert_eq!(Err(DispatchError::QueueFull), err);
        }

        dispatcher.guard().flush_init().unwrap();

        {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    result.lock().unwrap().push(20);
                })
                .unwrap();
        }

        dispatcher.guard().block_on_queue();

        assert_eq!(&*result.lock().unwrap(), &[1, 2, 3, 4, 5, 20]);
    }

    #[test]
    fn normal_queue_is_unbounded() {
        enable_test_logging();

        // Note: We can't actually test that it's fully unbounded,
        // but we can quickly queue more slow tasks than the pre-init buffer holds
        // and then guarantuee they all run.

        let mut dispatcher = Dispatcher::new(5);

        let result = Arc::new(Mutex::new(vec![]));

        for i in 1..=5 {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    result.lock().unwrap().push(i);
                })
                .unwrap();
        }

        dispatcher.guard().flush_init().unwrap();

        // Queue more than 5 tasks,
        // Each one is slow to process, so we should be faster in queueing
        // them up than they are processed.
        for i in 6..=20 {
            let result = Arc::clone(&result);
            dispatcher
                .guard()
                .launch(move || {
                    thread::sleep(Duration::from_millis(50));
                    result.lock().unwrap().push(i);
                })
                .unwrap();
        }

        dispatcher.guard().shutdown().unwrap();
        dispatcher.join().unwrap();

        let expected = (1..=20).collect::<Vec<_>>();
        assert_eq!(&*result.lock().unwrap(), &expected);
    }
}
