# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
import logging
from pathlib import Path
import re
import sys
import time
from typing import List, Tuple

from .._uniffi import (
    glean_get_upload_task,
    glean_initialize_for_subprocess,
    glean_process_ping_upload_response,
)
from .._uniffi import InternalConfiguration
from .._process_dispatcher import ProcessDispatcher


log = logging.getLogger("glean")


class PingUploadWorker:
    @classmethod
    def process(cls, testing_mode: bool = False):
        """
        Function to deserialize and process all serialized ping files.

        This function will ignore files that don't match the UUID regex and
        just delete them to prevent files from polluting the ping storage
        directory.
        """
        if testing_mode:
            cls._test_process_sync()
            return

        cls._process()

    @classmethod
    def _process(cls, testing_mode: bool = False):
        from .. import Glean

        return ProcessDispatcher.dispatch(
            _process,
            (
                Glean._data_dir,
                Glean._application_id,
                Glean._configuration,
            ),
        )

    @classmethod
    def _test_process_sync(cls) -> bool:
        """
        This is a test-only function to process the ping uploads in a separate
        process, but blocks until it is complete.

        Returns:
            uploaded (bool): The success of the upload task.
        """
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


def _parse_ping_headers(
    headers_as_json: str, document_id: str
) -> List[Tuple[str, str]]:
    """
    Parse the headers coming from FFI.

    Args:
        headers_as_json (str): The JSON key-value map of the headers.
        document_id (str): The id of the document the headers are for.

    Returns:
        headers (list of (str, str)): The headers to send.
    """
    headers: List[Tuple[str, str]] = []
    try:
        headers = list(json.loads(headers_as_json).items())
    except json.decoder.JSONDecodeError:
        log.error("Error while parsing headers for ping " + document_id)

    return headers


def _process(data_dir: Path, application_id: str, configuration) -> bool:

    # Import here to avoid cyclical import
    from ..glean import Glean

    if not Glean.is_initialized():
        # We don't want to send pings or otherwise update the database during
        # initialization in a subprocess, so we use
        # `glean_initialize_for_subprocess` rather than `glean_initialize` here.
        cfg = InternalConfiguration(
            data_path=str(data_dir),
            application_id=application_id,
            language_binding_name="python",
            # Set upload enabled to False. The subprocess should not record
            # telemetry. In the special `glean_initialize_for_subprocess` mode,
            # this does not have any side effects like resetting the client_id
            # or sending a deletion-request ping.
            upload_enabled=False,
            max_events=configuration.max_events,
            delay_ping_lifetime_io=False,
            use_core_mps=False,
            app_build="",
        )
        if not glean_initialize_for_subprocess(cfg):
            log.error("Couldn't initialize Glean in subprocess")
            sys.exit(1)

    # Limits are enforced by glean-core to avoid an inifinite loop here.
    # Whenever a limit is reached, this binding will receive `UploadTaskTag.DONE` and step out.
    while True:
        task = glean_get_upload_task()

        if task.is_upload():
            # Ping data is available for upload: parse the structure but make
            # sure to let Rust free the memory.
            request = task.request
            doc_id = request.document_id
            url_path = request.path
            body = bytes(request.body)
            headers = request.headers

            # Delegate the upload to the uploader.
            upload_result = configuration.ping_uploader.do_upload(
                url_path, body, headers, configuration
            )

            # Process the response.
            glean_process_ping_upload_response(doc_id, upload_result)
        elif task.is_wait():
            time.sleep(task.time / 1000)
        elif task.is_done():
            return True


__all__ = ["PingUploadWorker"]
