# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from typing import Callable, List, Optional


from .._uniffi import PingType as GleanPingType


class PingType:
    def __init__(
        self,
        name: str,
        include_client_id: bool,
        send_if_empty: bool,
        precise_timestamps: bool,
        include_info_sections: bool,
        schedules_pings: List[str],
        reason_codes: List[str],
        uploader_capabilities: List[str],
        enabled: bool = True,
        follows_collection_enabled: bool = True,
    ):
        """
        This implements the developer facing API for custom pings.

        The Ping API only exposes the `PingType.submit` method, which schedules a
        ping for eventual uploading.
        """
        self._reason_codes = reason_codes
        self._inner = GleanPingType(
            name,
            include_client_id,
            send_if_empty,
            precise_timestamps,
            include_info_sections,
            enabled,
            schedules_pings,
            reason_codes,
            follows_collection_enabled,
            uploader_capabilities,
        )
        self._test_callback = None  # type: Optional[Callable[[Optional[str]], None]]

    def test_before_next_submit(self, cb: Callable[[Optional[str]], None]):
        """
        **Test-only API**

        Attach a callback to be called right before a new ping is submitted.
        The provided function is called exactly once before submitting a ping.

        Note: The callback will be called on any call to submit.
        A ping might not be sent afterwards, e.g. if the ping is otherwise empty (and
        `send_if_empty` is `False`).
        """
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

        self._inner.submit(reason_string)

    def set_enabled(self, enabled: bool) -> None:
        """
        Enable or disable a ping.

        Disabling a ping causes all data for that ping to be removed from storage
        and all pending pings of that type to be deleted.
        """
        self._inner.set_enabled(enabled)
