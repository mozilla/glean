# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A base class for ping uploaders.
"""


from typing import Union, Dict, TYPE_CHECKING


from .ping_uploader import PingUploader, UploadResult


if TYPE_CHECKING:
    from glean.config import Configuration


class BaseUploader(PingUploader):
    """
    The logic for uploading pings. This leaves the actual upload implementation
    to the user-provided delegate.
    """

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
        """
        This function triggers the actual upload.

        It logs the ping and calls the implementation-specific upload function.

        Args:
            url (str): The URL path to upload the data to.
            data (bytes): The serialized data to send.
            headers (dict of (str, str)): Dictionary of header entries.
            config (glean.Configuration): The Glean Configuration object.

        Returns:
            result (UploadResult): the status code of the upload response.
        """

        return self.upload(
            url=config.server_endpoint + path,
            data=data,
            headers=headers,
        )


__all__ = ["BaseUploader"]
