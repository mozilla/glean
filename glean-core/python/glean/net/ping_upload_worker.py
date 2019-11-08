# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import logging
from pathlib import Path
import re


log = logging.getLogger(__name__)


class PingUploadWorker:
    # Ping files are UUIDs.  This matches UUIDs for filtering purposes.
    _FILE_PATTERN = re.compile(
        "[0-9a-fA-F]{8}-"
        "[0-9a-fA-F]{4}-"
        "[0-9a-fA-F]{4}-"
        "[0-9a-fA-F]{4}-"
        "[0-9a-fA-F]{12}"
    )

    # NOTE: The `PINGS_DIR" must be kept in sync with the one in the Rust implementation.
    _PINGS_DIR = "pending_pings"

    @classmethod
    def process(cls) -> bool:
        """
        Function to deserialize and process all serialized ping files.

        This function will ignore files that don't match the UUID regex and
        just delete them to prevent files from polluting the ping storage
        directory.

        Returns:
            uploaded (bool): The success of the upload task.
        """
        from .. import Glean

        success = True

        storage_dir = Glean.get_data_dir() / cls._PINGS_DIR

        log.debug(f"Processing persisted pings at {storage_dir.resolve()}")

        for path in storage_dir.iterdir():
            if path.is_file():
                if cls._FILE_PATTERN.match(path.name):
                    log.debug(f"Processing ping: {path.name}")
                    if not cls._process_file(path):
                        log.error(f"Error processing ping file: {path.name}")
                        success = False
                else:
                    log.debug(f"Pattern mismatch. Deleting {path.name}")
                    path.unlink()

        return success

    @classmethod
    def _process_file(cls, path: Path) -> bool:
        """
        Processes a single ping file.
        """
        from .. import Glean

        processed = False

        try:
            with open(path, "r", encoding="utf-8") as fd:
                lines = iter(fd)
                url_path = next(lines).strip()
                serialized_ping = next(lines)
        except FileNotFoundError:
            log.error(f"Could not find ping file {path.resolve()}")
            return False
        except IOError as e:
            log.error(f"IOError when reading {path.resolve()}: {e}")
            return False

        processed = Glean._configuration.ping_uploader.do_upload(
            url_path, serialized_ping, Glean._configuration
        )

        if processed:
            path.unlink()
            log.debug(f"{path.name} was deleted")

        return processed


__all__ = ["PingUploadWorker"]
