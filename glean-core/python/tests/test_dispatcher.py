# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import threading
import sys
import time


import pytest


from glean._dispatcher import Dispatcher
from glean import Glean
from glean import metrics
from glean.metrics import Lifetime


def test_launch_correctly_adds_tasks_to_queue_if_queue_tasks_is_true():
    thread_canary = [0]

    Dispatcher.set_task_queueing(True)

    @Dispatcher.task
    def update():
        thread_canary[0] += 1

    for _ in range(3):
        update()

    assert 3 == len(Dispatcher._preinit_task_queue)
    assert 0 == thread_canary[0]

    Dispatcher.flush_queued_initial_tasks()

    assert 3 == thread_canary[0]
    assert 0 == len(Dispatcher._preinit_task_queue)


def test_maximum_tasks():
    Dispatcher.set_task_queueing(True)

    for _ in range(Dispatcher.MAX_QUEUE_SIZE + 10):
        Dispatcher.task(lambda: 0)()

    assert len(Dispatcher._preinit_task_queue) == Dispatcher.MAX_QUEUE_SIZE


def test_maximum_queue():
    Dispatcher.set_task_queueing(True)

    for _ in range(Dispatcher.MAX_QUEUE_SIZE + 10):
        Dispatcher.launch(lambda: 0)

    assert len(Dispatcher._preinit_task_queue) == Dispatcher.MAX_QUEUE_SIZE


def test_tasks_run_off_the_main_thread():
    main_thread_id = threading.get_ident()
    thread_canary = [False]

    def test_task():
        assert main_thread_id != threading.get_ident()
        assert False is thread_canary[0]
        thread_canary[0] = True

    Dispatcher.launch(test_task)
    Dispatcher._task_worker._queue.join()

    assert True is thread_canary[0]


def test_queue_tasks_are_flushed_off_the_main_thread():
    main_thread_id = threading.get_ident()
    Dispatcher._testing_mode = False
    Dispatcher._queue_initial_tasks = False
    thread_canary = [0]

    def test_task():
        assert main_thread_id != threading.get_ident()
        thread_canary[0] += 1

    Dispatcher._testing_mode = False
    Dispatcher._queue_initial_tasks = True

    def task_runner():
        for _ in range(3):
            Dispatcher.launch(test_task)

        assert 3 == len(Dispatcher._preinit_task_queue)
        assert 0 == thread_canary[0]

        Dispatcher.flush_queued_initial_tasks()

    Dispatcher._task_worker.add_task(True, task_runner)

    Dispatcher._task_worker._queue.join()

    assert 3 == thread_canary[0]
    assert 0 == len(Dispatcher._preinit_task_queue)


def test_queued_tasks_are_executed_in_the_order_they_are_received():
    main_thread_id = threading.get_ident()
    Dispatcher._testing_mode = False
    Dispatcher._queue_initial_tasks = True

    class Job:
        thread_counter = [0]
        thread_list = []

        def __init__(self, num):
            self.num = num

        def __lt__(self, other):
            return id(self) < id(other)

        def __call__(self):
            assert main_thread_id != threading.get_ident()
            self.thread_counter[0] += 1
            self.thread_list.append(self.num)

    def task_runner():
        for i in range(50):
            Dispatcher.launch(Job(i))

        Dispatcher.flush_queued_initial_tasks()

        for i in range(50, 100):
            Dispatcher.launch(Job(i))

    Dispatcher._task_worker.add_task(True, task_runner)

    Dispatcher._task_worker._queue.join()

    assert Job.thread_list == list(range(100))
    assert Job.thread_counter[0] == 100


def test_dispatched_tasks_throwing_exceptions_are_correctly_handled():
    Dispatcher._testing_mode = False
    Dispatcher._queue_initial_tasks = False
    thread_canary = [0]

    def exception_task():
        42 / 0

    Dispatcher.launch(exception_task)

    def working_task():
        thread_canary[0] += 1

    for _ in range(3):
        Dispatcher.launch(working_task)

    Dispatcher._task_worker._queue.join()

    assert 3 == thread_canary[0]


def test_that_thread_joins_before_directory_is_deleted_in_reset():
    Dispatcher._testing_mode = False
    Dispatcher._queue_initial_tasks = False
    thread_canary = [0]

    boolean_metric = metrics.BooleanMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="boolean_metric",
        send_in_pings=["store1"],
    )

    def slow_task():
        time.sleep(1)
        # This will cause a Rust panic if the data directory was deleted in
        # Glean._reset() before this has a chance to run.
        boolean_metric.set(True)
        thread_canary[0] = 1

    Dispatcher.launch(slow_task)
    Glean._reset()

    assert thread_canary[0] == 1


def _subprocess():
    # If this runs on a thread, it won't complete before the process is
    # shutdown

    string_metric = metrics.StringMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_metric",
        send_in_pings=["store1"],
    )

    for i in range(1000):
        string_metric.set(str(i))


@pytest.mark.skipif(
    not sys.platform.startswith("linux"), reason="Test only works on Linux"
)
def test_single_threaded_in_multiprocessing_subprocess():
    import multiprocessing

    p = multiprocessing.Process(target=_subprocess)
    p.start()
    p.join()

    # This must match the definition in _subprocess() above.
    string_metric = metrics.StringMetricType(
        disabled=False,
        category="telemetry",
        lifetime=Lifetime.APPLICATION,
        name="string_metric",
        send_in_pings=["store1"],
    )

    assert string_metric.test_get_value() == "999"
