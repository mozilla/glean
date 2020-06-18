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

This uses the lower level `subprocess` library rather than `multiprocessing` to
avoid the unnecessary complexity and not place burdens on Glean's users to
architect their application startup to be `multiprocessing` compatible on
Windows.
"""

import base64
import logging
import os
from pathlib import Path
import pickle
import subprocess
import sys
from typing import Optional, Union


from ._subprocess import _process_dispatcher_helper


log = logging.getLogger(__name__)


class _SyncWorkWrapper:
    """
    A wrapper to synchronously call a function in the current process, but make
    it have the same (limited) interface as if it were a `subprocess.Popen`
    object:

        >>> p = ProcessDispatcher.dispatch(myfunc, ())
        >>> p.wait()
        >>> p.returncode
        0

    This is used only when `Configuration.allow_multiprocessing` is `False`.
    """

    def __init__(self, func, args):
        self._result = func(*args)
        self._waited = False

    def wait(self) -> None:
        self._waited = True

    @property
    def returncode(self) -> int:
        if not self._waited:
            raise RuntimeError("wait() must be called before returncode is available")
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
    _last_process: Optional[subprocess.Popen] = None

    # Detect if coverage is being collected in the current run
    _doing_coverage: bool = "coverage" in sys.modules

    @classmethod
    def _wait_for_last_process(cls) -> None:
        if cls._last_process is not None:
            cls._last_process.wait()
            cls._last_process = None

    @classmethod
    def dispatch(cls, func, args) -> Union[_SyncWorkWrapper, subprocess.Popen]:
        from . import Glean

        if Glean._configuration._allow_multiprocessing:
            # We only want one of these processes running at a time, so if
            # there's already one, join on it. Therefore, this should not be
            # run from the main user thread.
            cls._wait_for_last_process()

            # This sends the data over as a commandline argument, which has a
            # maximum length of:
            #   - 8191 characters on Windows
            #     (see: https://support.microsoft.com/en-us/help/830473/command-prompt-cmd-exe-command-line-string-limitation)  # noqa
            #   - As little as 4096 bytes on POSIX, though in practice much larger
            #     (see _POSIX_ARG_MAX_: https://www.gnu.org/software/libc/manual/html_node/Minimums.html)  # noqa
            # In practice, this is ~700 bytes, and the data is an implementation detail
            # that Glean controls. This approach may need to change to pass over a pipe
            # if it becomes too large.

            payload = base64.b64encode(pickle.dumps((func, args))).decode("ascii")

            if len(payload) > 4096:
                log.warning("data payload to subprocess is greater than 4096 bytes")

            # Help coverage.py do coverage across processes
            if cls._doing_coverage:
                os.environ["COVERAGE_PROCESS_START"] = str(
                    Path(".coveragerc").absolute()
                )

            p = subprocess.Popen(
                [sys.executable, _process_dispatcher_helper.__file__, payload]
            )

            cls._last_process = p

            return p
        else:
            return _SyncWorkWrapper(func, args)


__all__ = ["ProcessDispatcher"]
