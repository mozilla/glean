# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import sys
from typing import Dict, Union

from .._uniffi import UploadResult


if sys.version_info >= (3, 8):
    from typing import Protocol
else:
    Protocol = object


class PingUploader(Protocol):
    def upload(
        self, url: str, data: bytes, headers: Dict[str, str]
    ) -> Union[
        UploadResult,
        UploadResult.UNRECOVERABLE_FAILURE,
        UploadResult.RECOVERABLE_FAILURE,
        UploadResult.HTTP_STATUS,
    ]:
        """
        Upload a ping to a server.

        Args:
            url (str): The URL path to upload the data to.
            data (bytes): The serialized data to send.
            headers (dict of (str, str)): Dictionary of header entries.

        Returns:
            result (UploadResult): the status code of the upload response.
        """
        pass


__all__ = [
    "PingUploader",
    "UploadResult",
]
