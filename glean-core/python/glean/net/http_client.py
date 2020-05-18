# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
This module contains a ping uploader based on the Python stdlib's http.client
module.
"""


import http.client
import logging
import socket
from typing import List, Tuple
import urllib.parse


from . import base_uploader
from . import ping_uploader


log = logging.getLogger(__name__)


class HttpClientUploader(base_uploader.BaseUploader):
    # The timeout, in seconds, to use for all operations with the server.
    _DEFAULT_TIMEOUT = 10

    @classmethod
    def upload(
        cls, url: str, data: bytes, headers: List[Tuple[str, str]]
    ) -> ping_uploader.UploadResult:
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
            # If we don't know the URL scheme, log an error and mark this as an unrecoverable
            # error, like if it were a malformed URL.
            log.error("Unknown URL scheme {}".format(parsed_url.scheme))
            return ping_uploader.UnrecoverableFailure()

        try:
            conn.request(
                "POST", parsed_url.path, body=data, headers=dict(headers),
            )
            response = conn.getresponse()
        except http.client.InvalidURL as e:
            log.error(
                "Could not upload telemetry due to malformed URL: '{}' {}".format(
                    url, e
                )
            )
            return ping_uploader.UnrecoverableFailure()
        except http.client.HTTPException as e:
            log.error(
                "http.client.HTTPException while uploading ping: '{}' {}".format(url, e)
            )
            return ping_uploader.RecoverableFailure()
        except socket.gaierror as e:
            log.error("socket.gaierror while uploading ping: '{}' {}".format(url, e))
            return ping_uploader.RecoverableFailure()
        except OSError as e:
            log.error("OSError while uploading ping: '{}' {}".format(url, e))
            return ping_uploader.RecoverableFailure()
        except Exception as e:
            log.error("Unknown Exception while uploading ping: '{}' {}".format(url, e))
            return ping_uploader.RecoverableFailure()

        status_code = response.status

        conn.close()

        return ping_uploader.HttpResponse(status_code)


__all__ = ["HttpClientUploader"]
