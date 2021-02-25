# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import Callable, List, Optional


from ..glean import Glean
from .._dispatcher import Dispatcher
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
        self._test_callback = None  # type: Optional[Callable[[Optional[str]], None]]
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

    def test_before_next_submit(self, cb: Callable[[Optional[str]], None]):
        """
        **Test-only API**

        Attach a callback to be called right before a new ping is submitted.
        The provided function is called exactly once before submitting a ping.

        Note: The callback will be called on any call to submit.
        A ping might not be sent afterwards, e.g. if the ping is otherwise empty (and
        `send_if_empty` is `False`).
        """
        assert Dispatcher._testing_mode is True
        self._test_callback = cb

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

        if self._test_callback is not None:
            self._test_callback(reason_string)
            self._test_callback = None

        Glean._submit_ping(self, reason_string)
