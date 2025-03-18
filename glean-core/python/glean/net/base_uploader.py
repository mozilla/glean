# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A base class for ping uploaders.
"""

from typing import Union, TYPE_CHECKING


from .ping_uploader import PingUploader, UploadResult


if TYPE_CHECKING:
    from .ping_upload_worker import CapablePingUploadRequest


class BaseUploader(PingUploader):
    """
    The logic for uploading pings. This leaves the actual upload implementation
    to the user-provided delegate.
    """

    def do_upload(
        self,
        capable_request: "CapablePingUploadRequest",
    ) -> Union[
        UploadResult,
        UploadResult.UNRECOVERABLE_FAILURE,
        UploadResult.RECOVERABLE_FAILURE,
        UploadResult.HTTP_STATUS,
        UploadResult.INCAPABLE,
    ]:
        """
        This function triggers the actual upload.

        It logs the ping and calls the implementation-specific upload function.

        Args:
            capable_request (CapablePingUploadRequest): The ping upload request, locked behind a capability check.

        Returns:
            result (UploadResult): the status code of the upload response.
        """

        return self.upload(capable_request)


__all__ = ["BaseUploader"]
