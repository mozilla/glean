# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
This module implements a single-threaded (mostly FIFO) work queue on which
most Glean work is done.
"""

import functools
import logging
import queue
import threading
from typing import Callable, Dict, List, Tuple


# This module uses threading, rather than multiprocessing for parallelism. This
# is normally not recommended for Python due to the Global Interpreter Lock
# (GIL), however the usual problems with the GIL are lessened by the fact that:
#
#   - Most long-running work and I/O is done in the Rust extension. The cffi
#     library used to interface with Rust releases the GIL around every foreign
#     call. See https://cffi.readthedocs.io/en/latest/ref.html#conversions
#
#   - The other significant blocking I/O is in networking code, which runs
#     in a separate child process (see net/ping_upload_worker.py).
#
# This approach greatly reduces complexity of the implementation. Using
# multiprocessing would imply going to a 100% IPC-like approach, since the
# Rust-side Glean objects could not be easily shared or message-passed across
# the process boundary, whereas sharing across threads works transparently.


log = logging.getLogger(__name__)


class _ThreadWorker:
    """
    Manages a single worker to perform tasks in another thread.
    """

    END_MARKER = "END"

    def __init__(self):
        self._queue = queue.Queue()
        # The worker thread is only started when work needs to be performed so
        # that importing Glean alone does not start an unnecessary thread.
        self._started = False

    def add_task(self, sync: bool, task: Callable, *args, **kwargs):
        """
        Add a task to the worker queue.

        Args:
            sync (bool): If `True`, block until the task is complete.
            task (Callable): The task to run.

        Additional arguments are passed to the task.
        """
        if not self._started:
            self._start_worker()
        # If we are already on the worker thread, don't place the tasks in the
        # queue, just run them now. This is required for synchronous testing
        # mode, and also to run the tasks in the expected order.
        if threading.get_ident() == self._ident:
            task(*args, **kwargs)
        else:
            args = args or ()
            kwargs = kwargs or {}
            self._queue.put((task, args, kwargs))
            if sync:
                self._queue.join()

    def _start_worker(self):
        """
        Starts the worker thread.
        """
        self._thread = threading.Thread(target=self._worker)
        # Start the thread in daemon mode.
        self._thread.daemon = True
        self._thread.start()
        self._started = True
        self._ident = self._thread.ident

    def _worker(self):
        """
        Implements the worker thread. Takes tasks off of the queue and runs
        them.
        """
        while True:
            task, args, kwargs = self._queue.get()
            if task == self.END_MARKER:
                self._queue.task_done()
                break
            try:
                task(*args, **kwargs)
            except Exception:
                log.exception("Glean error")
            finally:
                self._queue.task_done()

    def _shutdown_thread(self):
        """
        Tell the worker thread to shutdown and then wait for 1 seconds for it
        to finish.
        """
        if not self._started:
            return

        # Send an END_MARKER to the worker thread to shut it down cleanly.
        self._queue.put((self.END_MARKER, (), {}))
        # Wait up to 1 second for the worker thread to complete.
        self._thread.join(1.0)
        if self._thread.is_alive():
            log.error("Timeout sending Glean telemetry")
        self._started = False
        self._thread = None


class Dispatcher:
    # This value was chosen in order to allow several tasks to be queued for
    # execution but still be conservative of memory. This queue size is
    # important for cases where setUploadEnabled(false) is not called so that
    # we don't continue to queue tasks and waste memory.
    MAX_QUEUE_SIZE = 100

    # When True, tasks will be queued for running later, otherwise, they
    # are run immediately
    _queue_initial_tasks = True  # type: bool

    # The preinit task queue
    _preinit_task_queue = []  # type: List[Tuple[Callable, tuple, dict]]

    # The live task queue to run things in another thread
    _task_worker = _ThreadWorker()

    # The number of tasks that overflowed the queue
    _overflow_count = 0  # type: int

    # When `True`, all tasks are run synchronously
    _testing_mode = False  # type: bool

    # A threading lock for synchronized work
    _thread_lock = threading.RLock()

    @classmethod
    def reset(cls):
        """
        Reset the dispatcher so the queue is cleared, and it is reset into
        queueing mode.
        """
        cls._queue_initial_tasks = True
        cls._preinit_task_queue = []
        cls._overflow_count = 0

    @classmethod
    def _execute_task(cls, func: Callable, *args, **kwargs):
        cls._task_worker.add_task(cls._testing_mode, func, *args, **kwargs)

    @classmethod
    def _add_task_to_queue(cls, func: Callable, args: Tuple, kwargs: Dict):
        """
        Helper function to add a task to the task queue.
        """
        with cls._thread_lock:
            if len(cls._preinit_task_queue) >= cls.MAX_QUEUE_SIZE:
                log.error("Exceeded maximum queue size, discarding task")

                # This value ends up in the `preinit_tasks_overflow` metric,
                # but we can't record directly there, because that would only
                # add the recording to an already-overflowing task queue and
                # would be silently dropped.
                cls._overflow_count += 1
                return
            cls._preinit_task_queue.append((func, args, kwargs))

    @classmethod
    def task(cls, func: Callable):
        """
        A decorator for coroutines that might either run in the task queue or
        immediately.

        This should only be used to decorate functions that are evaluated at
        import time and don't need to run immediately. To decorate a nested
        function at run time, use `DispatcherInternal.launch`.
        """

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            if cls._queue_initial_tasks:
                cls._add_task_to_queue(func, args, kwargs)
            else:
                cls._execute_task(func, *args, **kwargs)

        return wrapper

    @classmethod
    def launch(cls, func: Callable):
        """
        Either queue the function for running later, or run immediately,
        depending on the state of `_queue_initial_tasks`.

        This should only be used to decorate nested functions that are
        evaluated at runtime.  To decorate a function evaluated at
        import time, use `DispatcherInternal.task`.

        Can be used as a decorator::

            @Dispatcher.launch
            def my_task():
                # ... do work ...
                pass

        or as a function::

            def my_task():
                # ... do work ...
                pass
            Dispatcher.launch(my_task)
        """

        if cls._queue_initial_tasks:
            cls._add_task_to_queue(func, (), {})
        else:
            cls._execute_task(func)

    @classmethod
    def launch_at_front(cls, func: Callable):
        """
        Either queue the function for running later (before all other queued
        tasks), or run immediately, depending on the state of
        `_queue_initial_tasks`.
        """

        if cls._queue_initial_tasks:
            with cls._thread_lock:
                cls._preinit_task_queue.insert(0, (func, (), {}))
        else:
            func()

    @classmethod
    def set_task_queueing(cls, enabled: bool):
        """
        Enable queueing mode, which causes tasks to be queued until launched by
        calling `DispatcherInternal.flushQueuedInitialTasks`.

        Args:
            enabled (bool): Whether or not to queue tasks.
        """
        cls._queue_initial_tasks = enabled

    @classmethod
    def flush_queued_initial_tasks(cls):
        """
        Stops queueing tasks and processes any tasks in the queue.
        """
        cls.set_task_queueing(False)

        with cls._thread_lock:
            queue_copy = cls._preinit_task_queue[:]
            cls._preinit_task_queue.clear()

        for (task, args, kwargs) in queue_copy:
            cls._execute_task(task, *args, **kwargs)
        cls._preinit_task_queue.clear()

        if cls._overflow_count > 0:
            from ._builtins import metrics

            # This must happen after `cls.set_task_queueing(False)` is run, or
            # it would be added to a full task queue and be silently dropped.
            metrics.glean.error.preinit_tasks_overflow.add(
                cls.MAX_QUEUE_SIZE + cls._overflow_count
            )

            cls._overflow_count = 0
