# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import json
import logging
from pathlib import Path
import re
import time
from typing import List, Tuple

from .upload_task_tag import UploadTaskTag

from .. import _ffi
from .._glean_ffi import ffi as ffi_support  # type: ignore
from .._dispatcher import Dispatcher
from .._process_dispatcher import ProcessDispatcher
from .ping_uploader import RecoverableFailure


log = logging.getLogger(__name__)


# How many times to attempt waiting when told to by glean-core's upload API.
MAX_WAIT_ATTEMPTS = 3

# Maximum number of recoverable errors allowed before aborting the ping uploader
MAX_RETRIES = 3


class PingUploadWorker:
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
            _process, (Glean._data_dir, Glean._configuration)
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
    headers = []  # type: List[Tuple[str, str]]
    try:
        headers = list(json.loads(headers_as_json).items())
    except json.decoder.JSONDecodeError:
        log.error("Error while parsing headers for ping " + document_id)

    return headers


def _process(data_dir: Path, configuration) -> bool:

    # Import here to avoid cyclical import
    from ..glean import Glean

    if not Glean.is_initialized():
        # Always load the Glean shared object / dll even if we're in a (ping upload worker)
        # subprocess.
        # To make startup time better in subprocesses, consumers can initialize just the
        # ping upload manager.
        data_dir = ffi_support.new("char[]", _ffi.ffi_encode_string(str(data_dir)))
        _ffi.lib.glean_initialize_standalone_uploader(data_dir)

    wait_attempts = 0

    upload_failures = 0

    while upload_failures < MAX_RETRIES:
        incoming_task = ffi_support.new("FfiPingUploadTask *")
        _ffi.lib.glean_get_upload_task(incoming_task, configuration.log_pings)

        tag = incoming_task.tag
        if tag == UploadTaskTag.UPLOAD:
            # Ping data is available for upload: parse the structure but make
            # sure to let Rust free the memory.
            doc_id = _ffi.ffi_decode_string(
                incoming_task.upload.document_id, free_memory=False
            )
            url_path = _ffi.ffi_decode_string(
                incoming_task.upload.path, free_memory=False
            )
            body = _ffi.ffi_decode_byte_buffer(incoming_task.upload.body)
            headers = _ffi.ffi_decode_string(
                incoming_task.upload.headers, free_memory=False
            )

            # Delegate the upload to the uploader.
            upload_result = configuration.ping_uploader.do_upload(
                url_path, body, _parse_ping_headers(headers, doc_id), configuration
            )

            if isinstance(upload_result, RecoverableFailure):
                upload_failures = upload_failures + 1

            # Process the response.
            _ffi.lib.glean_process_ping_upload_response(
                incoming_task, upload_result.to_ffi()
            )
        elif tag == UploadTaskTag.WAIT:
            # Try not to be stuck waiting forever.
            if wait_attempts < MAX_WAIT_ATTEMPTS:
                wait_attempts += 1
                time.sleep(1)
            else:
                return False
        elif tag == UploadTaskTag.DONE:
            break

    return True


__all__ = ["PingUploadWorker"]
