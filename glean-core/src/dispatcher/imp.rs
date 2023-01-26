// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::{
    mem,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    thread::{self, JoinHandle},
};

use crossbeam_channel::{bounded, unbounded, Sender};

use super::DispatchError;

/// Command received while blocked from further work.
enum Blocked {
    /// Shutdown immediately without processing the queue.
    Shutdown,
    /// Unblock and continue with work as normal.
    Continue,
}

/// The command a worker should execute.
enum Command {
    /// A task is a user-defined function to run.
    Task(Box<dyn FnOnce() + Send>),

    /// Swap the channel
    Swap(Sender<()>),

    /// Signal the worker to finish work and shut down.
    Shutdown,
}

/// A clonable guard for a dispatch queue.
#[derive(Clone)]
pub struct DispatchGuard {
    /// Whether to queue on the preinit buffer or on the unbounded queue
    queue_preinit: Arc<AtomicBool>,

    /// The number of items that were added to the queue after it filled up.
    overflow_count: Arc<AtomicUsize>,

    /// The maximum pre-init queue size
    max_queue_size: usize,

    /// Used to unblock the worker thread initially.
    block_sender: Sender<Blocked>,

    /// Sender for the preinit queue.
    preinit_sender: Sender<Command>,

    /// Sender for the unbounded queue.
    sender: Sender<Command>,
}

impl DispatchGuard {
    pub fn launch(&self, task: impl FnOnce() + Send + 'static) -> Result<(), DispatchError> {
        let task = Command::Task(Box::new(task));
        self.send(task)
    }

    pub fn shutdown(&self) -> Result<(), DispatchError> {
        // Need to flush in order for the thread to actually process anything,
        // including the shutdown command.
        self.flush_init().ok();
        self.send(Command::Shutdown)
    }

    fn send(&self, task: Command) -> Result<(), DispatchError> {
        if self.queue_preinit.load(Ordering::SeqCst) {
            if self.preinit_sender.len() < self.max_queue_size {
                self.preinit_sender.send(task)?;
                Ok(())
            } else {
                self.overflow_count.fetch_add(1, Ordering::SeqCst);
                // Instead of using a bounded queue, we are handling the bounds
                // checking ourselves. If a bounded queue were full, we would return
                // a QueueFull DispatchError, so we do the same here.
                Err(DispatchError::QueueFull)
            }
        } else {
            self.sender.send(task)?;
            Ok(())
        }
    }

    pub fn block_on_queue(&self) {
        let (tx, rx) = crossbeam_channel::bounded(0);

        // We explicitly don't use `self.launch` here.
        // We always put this task on the unbounded queue.
        // The pre-init queue might be full before its flushed, in which case this would panic.
        // Blocking on the queue can only work if it is eventually flushed anyway.

        let task = Command::Task(Box::new(move || {
            tx.send(())
                .expect("(worker) Can't send message on single-use channel");
        }));
        self.sender
            .send(task)
            .expect("Failed to launch the blocking task");

        rx.recv()
            .expect("Failed to receive message on single-use channel");
    }

    pub fn kill(&self) -> Result<(), DispatchError> {
        // We immediately stop queueing in the pre-init buffer.
        let old_val = self.queue_preinit.swap(false, Ordering::SeqCst);
        if !old_val {
            return Err(DispatchError::AlreadyFlushed);
        }

        // Unblock the worker thread exactly once.
        self.block_sender.send(Blocked::Shutdown)?;
        Ok(())
    }

    /// Flushes the pre-init buffer.
    ///
    /// This function blocks until tasks queued prior to this call are finished.
    /// Once the initial queue is empty the dispatcher will wait for new tasks to be launched.
    ///
    /// Returns an error if called multiple times.
    pub fn flush_init(&self) -> Result<usize, DispatchError> {
        // We immediately stop queueing in the pre-init buffer.
        let old_val = self.queue_preinit.swap(false, Ordering::SeqCst);
        if !old_val {
            return Err(DispatchError::AlreadyFlushed);
        }

        // Unblock the worker thread exactly once.
        self.block_sender.send(Blocked::Continue)?;

        // Single-use channel to communicate with the worker thread.
        let (swap_sender, swap_receiver) = bounded(0);

        // Send final command and block until it is sent.
        self.preinit_sender
            .send(Command::Swap(swap_sender))
            .map_err(|_| DispatchError::SendError)?;

        // Now wait for the worker thread to do the swap and inform us.
        // This blocks until all tasks in the preinit buffer have been processed.
        swap_receiver.recv()?;

        // We're not queueing anymore.
        super::global::QUEUE_TASKS.store(false, Ordering::SeqCst);

        let overflow_count = self.overflow_count.load(Ordering::SeqCst);
        if overflow_count > 0 {
            Ok(overflow_count)
        } else {
            Ok(0)
        }
    }
}

/// A dispatcher.
///
/// Run expensive processing tasks sequentially off the main thread.
/// Tasks are processed in a single separate thread in the order they are submitted.
/// The dispatch queue will enqueue tasks while not flushed, up to the maximum queue size.
/// Processing will start after flushing once, processing already enqueued tasks first, then
/// waiting for further tasks to be enqueued.
pub struct Dispatcher {
    /// Guard used for communication with the worker thread.
    guard: DispatchGuard,

    /// Handle to the worker thread, allows to wait for it to finish.
    pub worker: Option<JoinHandle<()>>,
}

impl Dispatcher {
    /// Creates a new dispatcher with a maximum queue size.
    ///
    /// Launched tasks won't run until [`flush_init`] is called.
    ///
    /// [`flush_init`]: #method.flush_init
    pub fn new(max_queue_size: usize) -> Self {
        let (block_sender, block_receiver) = bounded(1);
        let (preinit_sender, preinit_receiver) = unbounded();
        let (sender, mut unbounded_receiver) = unbounded();

        let queue_preinit = Arc::new(AtomicBool::new(true));
        let overflow_count = Arc::new(AtomicUsize::new(0));

        let worker = thread::Builder::new()
            .name("glean.dispatcher".into())
            .spawn(move || {
                match block_receiver.recv() {
                    Err(_) => {
                        // The other side was disconnected.
                        // There's nothing the worker thread can do.
                        log::error!("The task producer was disconnected. Worker thread will exit.");
                        return;
                    }
                    Ok(Blocked::Shutdown) => {
                        // The other side wants us to stop immediately
                        return;
                    }
                    Ok(Blocked::Continue) => {
                        // Queue is unblocked, processing continues as normal.
                    }
                }

                let mut receiver = preinit_receiver;
                loop {
                    use Command::*;

                    match receiver.recv() {
                        Ok(Shutdown) => {
                            break;
                        }

                        Ok(Task(f)) => {
                            (f)();
                        }

                        Ok(Swap(swap_done)) => {
                            // A swap should only occur exactly once.
                            // This is upheld by `flush_init`, which errors out if the preinit buffer
                            // was already flushed.

                            // We swap the channels we listen on for new tasks.
                            // The next iteration will continue with the unbounded queue.
                            mem::swap(&mut receiver, &mut unbounded_receiver);

                            // The swap command MUST be the last one received on the preinit buffer,
                            // so by the time we run this we know all preinit tasks were processed.
                            // We can notify the other side.
                            swap_done
                                .send(())
                                .expect("The caller of `flush_init` has gone missing");
                        }

                        // Other side was disconnected.
                        Err(_) => {
                            log::error!(
                                "The task producer was disconnected. Worker thread will exit."
                            );
                            return;
                        }
                    }
                }
            })
            .expect("Failed to spawn Glean's dispatcher thread");

        let guard = DispatchGuard {
            queue_preinit,
            overflow_count,
            max_queue_size,
            block_sender,
            preinit_sender,
            sender,
        };

        Dispatcher {
            guard,
            worker: Some(worker),
        }
    }

    pub fn guard(&self) -> Arc<DispatchGuard> {
        Arc::new(self.guard.clone())
    }

    /// Waits for the worker thread to finish and finishes the dispatch queue.
    ///
    /// You need to call `shutdown` to initiate a shutdown of the queue.
    pub fn join(&mut self) -> Result<(), DispatchError> {
        if let Some(worker) = self.worker.take() {
            worker.join().map_err(|_| DispatchError::WorkerPanic)?;
        }
        Ok(())
    }
}
