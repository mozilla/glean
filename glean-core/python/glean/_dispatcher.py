# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import functools
from typing import Callable, List


class DispatcherInternal:
    # This value was chosen in order to allow several tasks to be queued for
    # execution but still be conservative of memory. This queue size is
    # important for cases where setUploadEnabled(false) is not called so that
    # we don't continue to queue tasks and waste memory.
    MAX_QUEUE_SIZE = 100

    def __init__(self):
        """
        Dispatcher to manage work that might be set up before
        `glean.Glean.initialize` is called, but needs to actually be performed
        afterward.
        """
        self._queue_initial_tasks: bool = True
        self._task_queue: List[Callable] = []

    def launch(self, func: Callable):
        """
        A decorator for coroutines that might either run in the task
        queue or immediately.
        """

        @functools.wraps(func)
        def wrapper(*args, **kwargs):
            if self._queue_initial_tasks:
                if len(self._task_queue) >= self.MAX_QUEUE_SIZE:
                    return
                self._task_queue.append((func, args, kwargs))
            else:
                func(*args, **kwargs)

        return wrapper

    def queue_at_front(self, func: Callable):
        """
        A decorator for coroutines that might either run at the front of the
        task queue or immediately.
        """

        @functools.wraps(func)
        def wrapper():
            if self._queue_initial_tasks:
                self._task_queue.insert(0, func)
            else:
                func()

        return wrapper

    def set_task_queueing(self, enabled: bool):
        """
        Enable queueing mode, which causes tasks to be queued until launched by
        calling `DispatcherInternal.flushQueuedInitialTasks`.

        Args:
            enabled (bool): Whether or not to queue tasks.
        """
        self._queue_initial_tasks = enabled

    def flush_queued_initial_tasks(self):
        """
        Stops queueing tasks and processes any tasks in the queue.
        """
        self.set_task_queueing(False)
        for (task, args, kwargs) in self._task_queue:
            task(*args, **kwargs)
        self._task_queue.clear()


Dispatcher = DispatcherInternal()
