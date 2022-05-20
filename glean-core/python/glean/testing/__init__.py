# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
Utilities for writing unit tests involving Glean.
"""


import gzip
from pathlib import Path
from typing import Dict, Optional, Union

from .._uniffi import glean_set_test_mode, glean_set_log_pings
from .._uniffi import ErrorType
from glean import Configuration
from ..net import base_uploader
from ..net.ping_uploader import UploadResult


def reset_glean(
    *,
    application_id: str,
    application_version: str,
    configuration: Optional[Configuration] = None,
    clear_stores: bool = True
) -> None:
    """
    Resets the Glean singleton.

    Args:
        application_id (str): The application id to use when sending pings.
        application_version (str): The version of the application sending
            Glean data.
        configuration (glean.config.Configuration): (optional) An object with
            global settings.
    """
    from glean import Glean

    data_dir: Optional[Path] = None
    if not clear_stores:
        Glean._destroy_data_dir = False
        data_dir = Glean._data_dir

    Glean._reset()

    Glean._testing_mode = True
    glean_set_test_mode(True)
    glean_set_log_pings(True)

    if data_dir is None:
        Glean._initialize_with_tempdir_for_testing(
            application_id=application_id,
            application_version=application_version,
            upload_enabled=True,
            configuration=configuration,
        )
    else:
        Glean.initialize(
            application_id=application_id,
            application_version=application_version,
            upload_enabled=True,
            data_dir=data_dir,
            configuration=configuration,
        )


class _RecordingUploader(base_uploader.BaseUploader):
    """
    A ping uploader that saves the results to disk for later inspection.

    This is used for testing only, but it needs to be importable from the Glean
    package since it runs in the ping upload worker subprocess.
    """

    def __init__(self, file_path):
        self.file_path = file_path

    def do_upload(
        self,
        path: str,
        data: bytes,
        headers: Dict[str, str],
        config: "Configuration",
    ) -> Union[
        UploadResult,
        UploadResult.UNRECOVERABLE_FAILURE,
        UploadResult.RECOVERABLE_FAILURE,
        UploadResult.HTTP_STATUS,
    ]:
        is_gzipped = headers.get("Content-Encoding", None) == "gzip"

        uncompressed_data = gzip.decompress(data) if is_gzipped else data
        with self.file_path.open("w") as fd:
            fd.write(str(path) + "\n")
            fd.write(uncompressed_data.decode("utf-8") + "\n")

        return UploadResult.HTTP_STATUS(200)


__all__ = ["reset_glean", "ErrorType"]
