# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from glean._dispatcher import Dispatcher


def test_launch_correctly_adds_tasks_to_queue_if_queue_tasks_is_true():
    thread_canary = [0]

    Dispatcher.set_task_queueing(True)

    @Dispatcher.launch
    def update():
        thread_canary[0] += 1

    for i in range(3):
        update()

    assert 3 == len(Dispatcher._task_queue)
    assert 0 == thread_canary[0]

    Dispatcher.flush_queued_initial_tasks()

    assert 3 == thread_canary[0]
    assert 0 == len(Dispatcher._task_queue)
