# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
import logging
from pathlib import Path
import re
import sys
import time
from typing import Callable, List, Optional, Tuple

from .._uniffi import (
    glean_get_upload_task,
    glean_initialize_for_subprocess,
    glean_process_ping_upload_response,
)
from .._uniffi import InternalConfiguration, UploadTaskAction, PingUploadTask, PingRequest
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
    "[0-9a-fA-F]{8}-" "[0-9a-fA-F]{4}-" "[0-9a-fA-F]{4}-" "[0-9a-fA-F]{4}-" "[0-9a-fA-F]{12}"
)


def _parse_ping_headers(headers_as_json: str, document_id: str) -> List[Tuple[str, str]]:
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
            trim_data_to_registered_pings=False,
            log_level=None,
            rate_limit=None,
            enable_event_timestamps=False,
            experimentation_id=None,
            enable_internal_pings=False,
            ping_schedule={},
            ping_lifetime_threshold=0,
            ping_lifetime_max_time=0,
        )
        if not glean_initialize_for_subprocess(cfg):
            log.error("Couldn't initialize Glean in subprocess")
            sys.exit(1)

    # Limits are enforced by glean-core to avoid an inifinite loop here.
    # Whenever a limit is reached, this binding will receive `UploadTaskTag.DONE` and step out.
    while True:
        task = glean_get_upload_task()

        if isinstance(task, PingUploadTask.UPLOAD):
            # Ping data is available for upload: parse the structure but make
            # sure to let Rust free the memory.
            request = task.request

            capable_request = CapablePingUploadRequest(
                PingUploadRequest(request, configuration.server_endpoint)
            )

            # Delegate the upload to the uploader.
            upload_result = configuration.ping_uploader.do_upload(capable_request)

            # Process the response.
            action = glean_process_ping_upload_response(request.document_id, upload_result)
            if action == UploadTaskAction.NEXT:
                continue
            else:  # action == UploadTaskAction.END
                return True

        elif isinstance(task, PingUploadTask.WAIT):
            time.sleep(task.time / 1000)
        elif task.is_done():
            return True


class PingUploadRequest:
    def __init__(self, request: PingRequest, server_endpoint: str) -> None:
        self.doc_id = (request.document_id,)
        self.url = server_endpoint + request.path
        self.body = bytes(request.body)
        self.headers = request.headers
        self.uploader_capabilities = request.uploader_capabilities


class CapablePingUploadRequest:
    def __init__(self, request: PingUploadRequest) -> None:
        self.request = request

    def capable(self, func: Callable[[List[str]], bool]) -> Optional[PingUploadRequest]:
        if func(self.request.uploader_capabilities):
            return self.request

        return None


__all__ = ["PingUploadWorker", "CapablePingUploadRequest"]
