# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
This module contains a ping uploader based on the Python stdlib's http.client
module.
"""


import http.client
import logging
from typing import List, Tuple
import urllib.parse


from . import base_uploader


log = logging.getLogger(__name__)


class HttpClientUploader(base_uploader.BaseUploader):
    # The timeout, in seconds, to use for all operations with the server.
    _DEFAULT_TIMEOUT = 10

    @classmethod
    def upload(cls, url: str, data: str, headers: List[Tuple[str, str]]) -> bool:
        """
        Synchronously upload a ping to a server.

        Args:
            url (str): The URL path to upload the data to.
            data (str): The serialized text data to send.
            headers (list of (str, str)): HTTP headers to send.
        """
        parsed_url = urllib.parse.urlparse(url)
        if parsed_url.scheme == "http":
            conn = http.client.HTTPConnection(
                parsed_url.hostname or "",
                port=parsed_url.port or 80,
                timeout=cls._DEFAULT_TIMEOUT,
            )
        elif parsed_url.scheme == "https":
            conn = http.client.HTTPSConnection(
                parsed_url.hostname or "",
                port=parsed_url.port or 443,
                timeout=cls._DEFAULT_TIMEOUT,
            )
        else:
            raise ValueError("Unknown URL scheme {}".format(parsed_url.scheme))

        conn.request(
            "POST", parsed_url.path, body=data.encode("utf-8"), headers=dict(headers),
        )
        response = conn.getresponse()

        log.debug("Ping upload: {}".format(response.status))

        status_class = response.status // 100

        conn.close()

        if status_class == 2:  # 2xx status
            # Known success
            # 200 - OK.  Request accepted into the pipeline
            log.debug("Ping successfully sent ({})".format(response.status))
            return True
        elif status_class == 4:  # 4xx status
            # Known client (4xx) errors:
            # 404 - not found - POST/PUT to an unknown namespace
            # 405 - wrong request type (anything other than POST/PUT)
            # 411 - missing content-length header
            # 413 - request body too large (Note that if we have badly-behaved
            #       clients that retry on 4XX, we should send back 202 on
            #       body/path too long).
            # 414 - request path too long (See above)

            # Something our client did is not correct. It's unlikely that the
            # client is going to recover from this by re-trying again, so we
            # just log and error and report a successful upload to the service.
            log.error("Server returned client error code: {}".format(response.status))
            return True
        else:
            # Known other errors:
            # 500 - internal error

            # For all other errors, we log a warning and try again at a later time.
            log.error("Server returned response code: {}".format(response.status))
            return False


__all__ = ["HttpClientUploader"]
