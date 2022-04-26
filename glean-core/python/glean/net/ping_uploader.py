# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import sys
from typing import List, Tuple


if sys.version_info >= (3, 8):
    from typing import Protocol
else:
    Protocol = object

"""
Result values of attempted ping uploads encoded for FFI use.
They are defined in `glean-core/src/upload/result.rs` and re-defined for use in Kotlin here.

NOTE:
THEY MUST BE THE SAME ACROSS BOTH FILES!
"""
# A recoverable error.
_UPLOAD_RESULT_RECOVERABLE = 0x1
# An unrecoverable error.
_UPLOAD_RESULT_UNRECOVERABLE = 0x2
# A HTTP response code.
_UPLOAD_RESULT_HTTP_STATUS = 0x8000


class UploadResult(Protocol):
    """
    The result of the ping upload.

    See below for the different possible cases.
    """

    def to_ffi(self) -> int:
        return _UPLOAD_RESULT_UNRECOVERABLE


class HttpResponse(UploadResult):
    """
    A HTTP response code.

    This can still indicate an error, depending on the status code.
    """

    def __init__(self, status_code: int):
        self._status_code = status_code

    def to_ffi(self) -> int:
        return _UPLOAD_RESULT_HTTP_STATUS | self._status_code


class UnrecoverableFailure(UploadResult):
    """
    An unrecoverable upload failure.

    A possible cause might be a malformed URL.
    The ping data is removed afterwards.
    """

    def to_ffi(self) -> int:
        return _UPLOAD_RESULT_UNRECOVERABLE


class RecoverableFailure(UploadResult):
    """
    A recoverable failure.

    During upload something went wrong,
    e.g. the network connection failed.
    The upload should be retried at a later time.
    """

    def to_ffi(self) -> int:
        return _UPLOAD_RESULT_RECOVERABLE


class PingUploader(Protocol):
    def upload(
        self, url: str, data: bytes, headers: List[Tuple[str, str]]
    ) -> UploadResult:
        """
        Upload a ping to a server.

        Args:
            url (str): The URL path to upload the data to.
            data (bytes): The serialized data to send.
            headers (list of (str, str)): List of header entries as tuple
                pairs, where the first element is the header name and the
                second is its value.

        Returns:
            result (UploadResult): the status code of the upload response.
        """
        pass


__all__ = [
    "HttpResponse",
    "PingUploader",
    "RecoverableFailure",
    "UnrecoverableFailure",
    "UploadResult",
]
