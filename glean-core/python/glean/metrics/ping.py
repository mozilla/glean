# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from ..glean import Glean
from .. import _ffi


class PingType:
    def __init__(self, name: str, include_client_id: bool):
        """
        This implements the developer facing API for custom pings.

        The Ping API only exposes the `PingType.send` method, which schedules a
        ping for sending.
        """
        self._name = name
        self._handle = _ffi.lib.glean_new_ping_type(
            _ffi.ffi_encode_string(name), include_client_id
        )
        Glean.register_ping_type(self)

    def __del__(self):
        if self._handle != 0:
            _ffi.lib.glean_destroy_ping_type(self._handle)

    @property
    def name(self):
        """
        Get the name of the ping.
        """
        return self._name

    def send(self):
        """
        Send the ping.

        If the ping currently contains no content, it will not be sent.
        """
        Glean._send_pings([self])
