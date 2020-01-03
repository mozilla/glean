# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import functools
from typing import Callable, List, Tuple


class Dispatcher:
    # This value was chosen in order to allow several tasks to be queued for
    # execution but still be conservative of memory. This queue size is
    # important for cases where setUploadEnabled(false) is not called so that
    # we don't continue to queue tasks and waste memory.
    MAX_QUEUE_SIZE = 100

    # When True, tasks will be queued for running later, otherwise, they
    # are run immediately
    _queue_initial_tasks = True  # type: bool

    # The task queue
    _task_queue = []  # type: List[Tuple[Callable, tuple, dict]]

    @classmethod
    def reset(cls):
        """
        Reset the dispatcher so the queue is cleared, and it is reset into
        queueing mode.
        """
        cls._queue_initial_tasks = True
        cls._task_queue = []

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
                if len(cls._task_queue) >= cls.MAX_QUEUE_SIZE:
                    return
                cls._task_queue.append((func, args, kwargs))
            else:
                func(*args, **kwargs)

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
            if len(cls._task_queue) >= cls.MAX_QUEUE_SIZE:
                return
            cls._task_queue.append((func, (), {}))
        else:
            func()

    @classmethod
    def launch_at_front(cls, func: Callable):
        """
        Either queue the function for running later (before all other queued
        tasks), or run immediately, depending on the state of
        `_queue_initial_tasks`.
        """

        if cls._queue_initial_tasks:
            cls._task_queue.insert(0, (func, (), {}))
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
        for (task, args, kwargs) in cls._task_queue:
            task(*args, **kwargs)
        cls._task_queue.clear()
