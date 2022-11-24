// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use dispatch::{Queue, QueueAttribute};

use super::DispatchError;

#[repr(u8)]
enum QueueStatus {
    NotFlushed = 0,
    Flushing = 1,
    IsFlushed = 2,
    Shutdown = 3,
}

/// A dispatcher.
///
/// Run expensive processing tasks sequentially off the main thread.
/// Tasks are processed in a serial queue in the order they are submitted.
/// The dispatch queue will enqueue tasks while not flushed, up to the maximum queue size.
/// Processing will start after flushing once, processing already enqueued tasks first, then
/// waiting for further tasks to be enqueued.
pub struct Dispatcher {
    guard: Arc<DispatchGuard>,
}

impl Dispatcher {
    /// Creates a new dispatcher with a maximum queue size.
    ///
    /// Launched tasks won't run until [`flush_init`] is called.
    ///
    /// [`flush_init`]: #method.flush_init
    pub fn new(max_queue_size: usize) -> Self {
        let queue = Queue::create("glean.dispatcher", QueueAttribute::Serial);
        let preinit_queue = Mutex::new(Vec::with_capacity(10));
        let overflow_count = Arc::new(AtomicUsize::new(0));

        let guard = DispatchGuard {
            queue: Some(queue),
            flushed: AtomicU8::new(QueueStatus::NotFlushed as u8),
            max_queue_size,
            overflow_count,
            preinit_queue,
        };

        Dispatcher {
            guard: Arc::new(guard),
        }
    }

    pub fn guard(&self) -> Arc<DispatchGuard> {
        self.guard.clone()
    }

    /// Waits for the worker thread to finish and finishes the dispatch queue.
    ///
    /// You need to call `shutdown` to initiate a shutdown of the queue.
    pub fn join(&mut self) -> Result<(), DispatchError> {
        if let Some(guard) = Arc::get_mut(&mut self.guard) {
            if let Some(queue) = guard.queue.take() {
                queue.exec_sync(|| {
                    // intentionally left empty
                });
                drop(queue);
            }
        }
        Ok(())
    }
}

/// A clonable guard for a dispatch queue.
pub struct DispatchGuard {
    /// The queue to run on
    queue: Option<Queue>,

    /// Status of the queue. One of `QueueStatus`
    flushed: AtomicU8,

    /// The maximum pre-init queue size
    max_queue_size: usize,

    /// The number of items that were added to the queue after it filled up
    overflow_count: Arc<AtomicUsize>,

    /// The pre-init queue
    ///
    /// Collects tasks before `flush_init` is called up until `max_queue_size`.
    preinit_queue: Mutex<Vec<Box<dyn FnOnce() + Send + 'static>>>,
}

impl DispatchGuard {
    fn queue(&self) -> &Queue {
        self.queue.as_ref().unwrap()
    }

    /// Launch a new task asynchronously.
    ///
    /// The tasks won't run until [`flush_init`] is called.
    pub fn launch(&self, task: impl FnOnce() + Send + 'static) -> Result<(), DispatchError> {
        if self.flushed.load(Ordering::SeqCst) == QueueStatus::IsFlushed as u8 {
            self.queue().exec_async(task);
            Ok(())
        } else {
            let mut queue = self.preinit_queue.lock().unwrap();
            if queue.len() < self.max_queue_size {
                queue.push(Box::new(task));
                Ok(())
            } else {
                self.overflow_count.fetch_add(1, Ordering::SeqCst);
                // Instead of using a bounded queue, we are handling the bounds
                // checking ourselves. If a bounded queue were full, we would return
                // a QueueFull DispatchError, so we do the same here.
                Err(DispatchError::QueueFull)
            }
        }
    }

    /// Shut down the dispatch queue.
    ///
    /// No new tasks will be processed after this.
    pub fn shutdown(&self) -> Result<(), DispatchError> {
        self.flush_init().ok();
        self.flushed
            .store(QueueStatus::Shutdown as u8, Ordering::SeqCst);
        Ok(())
    }

    /// Block until all tasks prior to this call are processed.
    pub fn block_on_queue(&self) {
        let status = self.flushed.load(Ordering::SeqCst);
        if status == QueueStatus::IsFlushed as u8 {
            self.queue().exec_sync(|| {
                // intentionally left empty
            });
        } else if status != QueueStatus::Shutdown as u8 {
            // block_on_queue is test-only, so spin-looping seems okay enough.
            while self.flushed.load(Ordering::SeqCst) != QueueStatus::IsFlushed as u8 {
                std::thread::yield_now();
            }
            self.queue().exec_sync(|| {
                // intentionally left empty
            });
        }
    }

    /// Flushes the pre-init buffer.
    ///
    /// This function blocks until tasks queued prior to this call are finished.
    /// Once the initial queue is empty the dispatcher will wait for new tasks to be launched.
    ///
    /// Returns an error if called multiple times.
    pub fn flush_init(&self) -> Result<usize, DispatchError> {
        if let Err(_old) = self.flushed.compare_exchange(
            QueueStatus::NotFlushed as u8,
            QueueStatus::Flushing as u8,
            Ordering::Acquire,
            Ordering::Relaxed,
        ) {
            return Err(DispatchError::AlreadyFlushed);
        }

        {
            let mut queue = self.preinit_queue.lock().unwrap();
            for task in queue.drain(..) {
                self.queue().exec_sync(task);
            }
        }

        let overflow_count = self.overflow_count.load(Ordering::SeqCst);

        self.flushed
            .store(QueueStatus::IsFlushed as u8, Ordering::SeqCst);

        if overflow_count > 0 {
            Ok(overflow_count)
        } else {
            Ok(0)
        }
    }

    pub fn kill(&self) -> Result<(), DispatchError> {
        Ok(())
    }
}
