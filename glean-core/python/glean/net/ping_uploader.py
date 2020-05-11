# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import sys
from typing import List, Tuple


if sys.version_info >= (3, 8):
    from typing import Protocol
else:
    Protocol = object


class PingUploader(Protocol):
    def upload(self, url: str, data: str, headers: List[Tuple[str, str]]) -> bool:
        """
        Upload a ping to a server.

        Args:
            url (str): The URL path to upload the data to.
            data (str): The serialized text data to send.
            headers (list of (str, str)): List of header entries as tuple
                pairs, where the first element is the header name and the
                second is its value.

        Returns:
            sent (bool): True if the ping was correctly dealt with (sent
                successfully or faced an unrecoverable error). False if there
                was a recoverable error that callers can deal with.
        """
        pass


__all__ = ["PingUploader"]
