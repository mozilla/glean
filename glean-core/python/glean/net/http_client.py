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
from typing import Union
import urllib.parse


from . import base_uploader
from .ping_uploader import CapablePingUploadRequest, UploadResult


log = logging.getLogger("glean")


class HttpClientUploader(base_uploader.BaseUploader):
    # The timeout, in seconds, to use for all operations with the server.
    _DEFAULT_TIMEOUT = 10

    @classmethod
    def upload(
        cls, capable_request: CapablePingUploadRequest
    ) -> Union[
        UploadResult,
        UploadResult.UNRECOVERABLE_FAILURE,
        UploadResult.RECOVERABLE_FAILURE,
        UploadResult.HTTP_STATUS,
        UploadResult.INCAPABLE,
    ]:
        """
        Synchronously upload a ping to a server.

        Args:
            capable_request (CapablePingUploadRequest): The ping upload request, locked behind a capability check.
        """
        # This uploader has no special capabilities.
        request = capable_request.capable(lambda capabilities: len(capabilities) == 0)
        if request is None:
            return UploadResult.INCAPABLE(0)

        url = request.url
        data = request.body
        headers = request.headers

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
            log.error(f"Unknown URL scheme {parsed_url.scheme}")
            return UploadResult.UNRECOVERABLE_FAILURE(0)

        try:
            conn.request(
                "POST",
                parsed_url.path,
                body=data,
                headers=headers,
            )
            response = conn.getresponse()
        except http.client.InvalidURL as e:
            log.error(f"Could not upload telemetry due to malformed URL: '{url}' {e}")
            return UploadResult.UNRECOVERABLE_FAILURE(0)
        except http.client.HTTPException as e:
            log.debug(f"http.client.HTTPException while uploading ping: '{url}' {e}")
            return UploadResult.RECOVERABLE_FAILURE(0)
        except socket.gaierror as e:
            log.debug(f"socket.gaierror while uploading ping: '{url}' {e}")
            return UploadResult.RECOVERABLE_FAILURE(0)
        except OSError as e:
            log.debug(f"OSError while uploading ping: '{url}' {e}")
            return UploadResult.RECOVERABLE_FAILURE(0)
        except Exception as e:
            log.error(f"Unknown Exception while uploading ping: '{url}' {e}")
            return UploadResult.RECOVERABLE_FAILURE(0)

        status_code = response.status

        conn.close()

        return UploadResult.HTTP_STATUS(status_code)


__all__ = ["HttpClientUploader"]
