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
    A wrapper to call a function in another process and convert its boolean
    success value into a process exitcode.
    """
    success = func(*args)

    if success:
        sys.exit(0)
    else:
        sys.exit(1)


class _SyncWorkWrapper:
    """
    A wrapper to synchronously call a function in the current process, but make
    it have the same (limited) interface as if it were a
    multiprocessing.Process:

        >>> p = ProcessDispatcher.dispatch(myfunc, ())
        >>> p.join()
        >>> p.exitcode
        0
    """

    def __init__(self, func, args):
        self._result = func(*args)
        self._joined = False

    def join(self):
        self._joined = True

    @property
    def exitcode(self):
        if not self._joined:
            raise RuntimeError("join() must be called before exitcode is available")
        if self._result:
            return 0
        else:
            return 1


class ProcessDispatcher:
    """
    A class to dispatch work to another process. Each dispatched function is
    run in a newly-created process, but only one of these processes will run at
    a given time. That is, it doesn't create another worker process until the
    previous worker process has completed.

    Since running a second process might block, this should only be used from
    the worker thread (such as the one in `_dispatcher.Dispatcher).
    """

    # Store the last run process object so we can `join` it when starting
    # another process.
    _last_process = None  # type: Optional[multiprocessing.Process]

    @classmethod
    def dispatch(cls, func, args) -> Union[_SyncWorkWrapper, "multiprocessing.Process"]:
        from . import Glean

        if Glean._configuration._allow_multiprocessing:
            # Only import the multiprocessing library if it's actually needed
            import multiprocessing  # noqa

            # We only want one of these processes running at a time, so if
            # there's already one, join on it. Therefore, this should not be
            # run from the main user thread.
            if cls._last_process is not None:
                cls._last_process.join()
                cls._last_process = None

            p = multiprocessing.Process(target=_work_wrapper, args=(func, args))
            p.start()

            cls._last_process = p

            return p
        else:
            return _SyncWorkWrapper(func, args)


__all__ = ["ProcessDispatcher"]
