# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
from pathlib import Path
import re


from .._dispatcher import Dispatcher
from .._process_dispatcher import ProcessDispatcher


log = logging.getLogger(__name__)


class PingUploadWorker:

    # NOTE: The `PINGS_DIR" must be kept in sync with the one in the Rust implementation.
    _PINGS_DIR = "pending_pings"

    @classmethod
    def storage_directory(cls) -> Path:
        from .. import Glean

        return Glean.get_data_dir() / cls._PINGS_DIR

    @classmethod
    def process(cls):
        """
        Function to deserialize and process all serialized ping files.

        This function will ignore files that don't match the UUID regex and
        just delete them to prevent files from polluting the ping storage
        directory.
        """
        if Dispatcher._testing_mode:
            cls._test_process_sync()
            return

        cls._process()

    @classmethod
    def _process(cls):
        from .. import Glean

        return ProcessDispatcher.dispatch(
            _process, (cls.storage_directory(), Glean._configuration)
        )

    @classmethod
    def _test_process_sync(cls) -> bool:
        """
        This is a test-only function to process the ping uploads in a separate
        process, but blocks until it is complete.

        Returns:
            uploaded (bool): The success of the upload task.
        """
        assert Dispatcher._testing_mode is True

        p = cls._process()
        p.wait()
        return p.returncode == 0


# Ping files are UUIDs.  This matches UUIDs for filtering purposes.
_FILE_PATTERN = re.compile(
    "[0-9a-fA-F]{8}-"
    "[0-9a-fA-F]{4}-"
    "[0-9a-fA-F]{4}-"
    "[0-9a-fA-F]{4}-"
    "[0-9a-fA-F]{12}"
)


def _process(storage_dir: Path, configuration) -> bool:
    success = True

    log.debug("Processing persisted pings at {}".format(storage_dir.resolve()))

    try:
        for path in storage_dir.iterdir():
            if path.is_file():
                if _FILE_PATTERN.match(path.name):
                    log.debug("Processing ping: {}".format(path.name))
                    if not _process_file(path, configuration):
                        log.error("Error processing ping file: {}".format(path.name))
                        success = False
                else:
                    log.debug("Pattern mismatch. Deleting {}".format(path.name))
                    path.unlink()
    except FileNotFoundError:
        log.debug("File not found: {}".format(storage_dir.resolve()))
        success = False

    return success


def _process_file(path: Path, configuration) -> bool:
    """
    Processes a single ping file.
    """
    processed = False

    try:
        with path.open("r", encoding="utf-8") as fd:
            lines = iter(fd)
            try:
                url_path = next(lines).strip()
                serialized_ping = next(lines)
                valid_content = True
            except StopIteration:
                valid_content = False
        # On Windows, we must close the file before deleting it
        if not valid_content:
            path.unlink()
            log.error("Invalid ping content in {}".format(path.resolve()))
            return False
    except FileNotFoundError:
        log.error("Could not find ping file {}".format(path.resolve()))
        return False
    except IOError as e:
        log.error("IOError when reading {}: {}".format(path.resolve(), e))
        return False

    processed = configuration.ping_uploader.do_upload(
        url_path, serialized_ping, configuration
    )

    if processed:
        path.unlink()
        log.debug("{} was deleted".format(path.name))

    return processed


__all__ = ["PingUploadWorker"]
