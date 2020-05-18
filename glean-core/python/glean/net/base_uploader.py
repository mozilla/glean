# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A base class for ping uploaders.
"""


import logging
from typing import List, Tuple, TYPE_CHECKING


from . import ping_uploader


if TYPE_CHECKING:
    from glean.config import Configuration


log = logging.getLogger(__name__)


class BaseUploader(ping_uploader.PingUploader):
    """
    The logic for uploading pings. This leaves the actual upload implementation
    to the user-provided delegate.
    """

    def do_upload(
        self,
        path: str,
        data: bytes,
        headers: List[Tuple[str, str]],
        config: "Configuration",
    ) -> ping_uploader.UploadResult:
        """
        This function triggers the actual upload.

        It logs the ping and calls the implementation-specific upload function.

        Args:
            url (str): The URL path to upload the data to.
            data (bytes): The serialized data to send.
            headers (list of (str, str)): List of header entries as tuple
                pairs, where the first element is the header name and the
                second is its value.
            config (glean.Configuration): The Glean Configuration object.

        Returns:
            result (UploadResult): the status code of the upload response.
        """
        if config.ping_tag is not None:
            headers.append(("X-Debug-ID", config.ping_tag))

        return self.upload(
            url=config.server_endpoint + path, data=data, headers=headers,
        )


__all__ = ["BaseUploader"]
