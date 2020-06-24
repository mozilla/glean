# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import List, Optional


from ..glean import Glean
from .. import _ffi


class PingType:
    def __init__(
        self,
        name: str,
        include_client_id: bool,
        send_if_empty: bool,
        reason_codes: List[str],
    ):
        """
        This implements the developer facing API for custom pings.

        The Ping API only exposes the `PingType.submit` method, which schedules a
        ping for eventual uploading.
        """
        self._name = name
        self._reason_codes = reason_codes
        self._handle = _ffi.lib.glean_new_ping_type(
            _ffi.ffi_encode_string(name),
            include_client_id,
            send_if_empty,
            _ffi.ffi_encode_vec_string(reason_codes),
            len(reason_codes),
        )
        Glean.register_ping_type(self)

    def __del__(self):
        if self._handle != 0:
            _ffi.lib.glean_destroy_ping_type(self._handle)

    @property
    def name(self) -> str:
        """
        Get the name of the ping.
        """
        return self._name

    def submit(self, reason: Optional[int] = None) -> None:
        """
        Collect and submit the ping for eventual uploading.

        If the ping currently contains no content, it will not be sent.

        Args:
            reason (enum, optional): The reason the ping was submitted.
        """
        reason_string: Optional[str] = None
        if reason is not None:
            reason_string = self._reason_codes[reason]
        else:
            reason_string = None
        Glean._submit_ping(self, reason_string)
