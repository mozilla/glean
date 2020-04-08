# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
This module implements a class to dispatch work to another process.
This work always happens one-at-a-time.  That is, it doesn't create
another worker process until the previous worker process has completed.

This is used only by the PingUploadWorker (and the test-only feature to delete
temporary data directories which has a potential race condition with the
PingUploadWorker).
"""

import sys
from typing import Optional, TYPE_CHECKING, Union


if TYPE_CHECKING:
    import multiprocessing  # noqa


def _work_wrapper(func, args):
    """
    A wrapper to call a function and convert its boolean success value into a
    process exitcode.
    """
    success = func(*args)

    if success:
        sys.exit(0)
    else:
        sys.exit(1)


class ProcessDispatcher:
    """
    A class to dispatch work to another process. This work always happens
    one-at-a-time. That is, it doesn't create another worker process until the
    previous worker process has completed.

    Since running a second process might block, this should only be used from
    the worker thread in `_dispatcher.Dispatcher.
    """

    # Store the last run process object so we can `join` it when starting
    # another process.
    _last_process = None  # type: Optional[multiprocessing.Process]

    @classmethod
    def dispatch(cls, func, args) -> Union[bool, "multiprocessing.Process"]:
        from . import Glean

        if Glean._configuration._allow_multiprocessing:
            # Only import the multiprocessing library if it's actually needed
            import multiprocessing  # noqa

            # We only want one of these processes running at a time, so if there's
            # already one, join on it. PingUploadWorker.process is only triggered
            # from a worker thread anyway, so this blocking will not block the main
            # user thread.
            if cls._last_process is not None:
                cls._last_process.join()
                cls._last_process = None

            p = multiprocessing.Process(target=_work_wrapper, args=(func, args))
            p.start()

            cls._last_process = p

            return p
        else:
            return func(*args)


__all__ = ["ProcessDispatcher"]
