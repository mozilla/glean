# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path
from . import PingUploadWorker


class DeletionPingUploadWorker(PingUploadWorker):
    # NOTE: The `DELETION_PINGS_DIR" must be kept in sync with the one in the Rust implementation.
    _DELETION_PINGS_DIR = "deletion_request"

    @classmethod
    def storage_directory(cls) -> Path:
        from .. import Glean

        return Glean.get_data_dir() / cls._DELETION_PINGS_DIR


__all__ = ["DeletionPingUploadWorker"]
