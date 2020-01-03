# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A base class for ping uploaders.
"""


import datetime
from email.utils import formatdate
import json
import logging
import time
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

    @staticmethod
    def _log_ping(path: str, data: str):
        """
        Log the contents of the ping to the console.

        Args:
            path (str): The URL path to append to the server address.
            data (str): The serialized text data to send.
        """
        try:
            parsed_json = json.loads(data)
        except json.decoder.JSONDecodeError as e:
            log.debug("Exception parsing ping as JSON: " + str(e))
        else:
            indented = json.dumps(parsed_json, indent=2)

            log.debug("Glean ping to URL: {}\n{}".format(path, indented))

    @staticmethod
    def _create_date_header_value():
        """
        Generate an RFC 1123 date string to be used in the HTTP header.
        """
        # Roundabout way to do this using only the standard library and without
        # monkeying with the global locale state.
        dt = datetime.datetime.now()
        stamp = time.mktime(dt.timetuple())
        return formatdate(timeval=stamp, localtime=False, usegmt=True)

    @classmethod
    def _get_headers_to_send(cls, config: "Configuration") -> List[Tuple[str, str]]:
        """
        Generate a list of headers to send with the request.

        Args:
            config (glean.Configuration): The Glean Configuration object.

        Returns:
            headers (list of (str, str)): The headers to send.
        """
        import glean

        headers = [
            ("Content-Type", "application/json; charset=utf-8"),
            ("User-Agent", config.user_agent),
            ("Date", cls._create_date_header_value()),
            # Add headers for supporting the legacy pipeline
            ("X-Client-Type", "Glean"),
            ("X-Client-Version", glean.__version__),
        ]

        if config.ping_tag is not None:
            headers.append(("X-Debug-ID", config.ping_tag))

        return headers

    def do_upload(self, path: str, data: str, config: "Configuration") -> bool:
        """
        This function triggers the actual upload.

        It logs the ping and calls the implementation-specific upload function.

        Args:
            path (str): The URL path to append to the server address.
            data (str): The serialized text data to send.
            config (glean.Configuration): The Glean Configuration object.

        Returns:
            sent (bool): True if the ping was correctly dealt with (sent
                successfully or faced and unrecoverable error). False if there
                was a recoverable error that callers can deal with.
        """
        if config.log_pings:
            self._log_ping(path, data)

        return self.upload(
            url=config.server_endpoint + path,
            data=data,
            headers=self._get_headers_to_send(config),
        )


__all__ = ["BaseUploader"]
