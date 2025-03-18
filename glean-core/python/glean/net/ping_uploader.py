# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import sys
from typing import Union

from .._uniffi import UploadResult
from .ping_upload_worker import CapablePingUploadRequest

if sys.version_info >= (3, 8):
    from typing import Protocol
else:
    Protocol = object


class PingUploader(Protocol):
    def upload(
        self, capable_request: CapablePingUploadRequest
    ) -> Union[
        UploadResult,
        UploadResult.UNRECOVERABLE_FAILURE,
        UploadResult.RECOVERABLE_FAILURE,
        UploadResult.HTTP_STATUS,
        UploadResult.INCAPABLE,
    ]:
        """
        Upload a ping to a server.

        Args:
            capable_request (CapablePingUploadRequest): The ping upload request, locked behind a capability check.

        Returns:
            result (UploadResult): the status code of the upload response.
        """
        pass


__all__ = [
    "CapablePingUploadRequest",
    "PingUploader",
    "UploadResult",
]
