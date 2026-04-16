# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
Provides an object to pass configuration to Glean.
"""

from typing import Optional


from . import net
from ._uniffi import SessionMode


# The default server pings are sent to
DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"


# The default number of events to store before sending
DEFAULT_MAX_EVENTS = 500


# The default inactivity timeout (milliseconds) for AUTO-mode sessions: 30 minutes.
DEFAULT_SESSION_INACTIVITY_TIMEOUT_MS = 1_800_000


class Configuration:
    """
    Configuration values for Glean.
    """

    def __init__(
        self,
        server_endpoint: Optional[str] = None,
        channel: Optional[str] = None,
        max_events: int = DEFAULT_MAX_EVENTS,
        ping_uploader: Optional[net.BaseUploader] = None,
        allow_multiprocessing: bool = True,
        enable_event_timestamps: bool = True,
        experimentation_id: Optional[str] = None,
        enable_internal_pings: bool = True,
        session_mode: SessionMode = SessionMode.AUTO,
        session_sample_rate: float = 1.0,
        session_inactivity_timeout_ms: int = DEFAULT_SESSION_INACTIVITY_TIMEOUT_MS,
    ):
        """
        Args:
            server_endpoint (str): Optional. The server pings are sent to.
                Defaults to `DEFAULT_TELEMETRY_ENDPOINT`.
            channel (str): Optional. The release channel the application is on,
                if known.
            max_events (int): Optional.The number of events to store before
                force-sending. Defaults to `DEFAULT_MAX_EVENTS`.
            ping_uploader (glean.net.BaseUploader): Optional. The ping uploader
                implementation. Defaults to `glean.net.HttpClientUploader`.
            allow_multiprocessing (bool): When True (default), use a subprocess
                to offload some work (such as ping uploading).
            enable_event_timestamps (bool): Whether to add a wallclock timestamp
                to all events. Default: `True`.
            experimentation_id (string): An experimentation identifier derived
                by the application to be sent with all pings. Default: None.
            enable_internal_pings (bool): Whether to enable internal pings. Default: `True`.
            session_mode (SessionMode): How Glean manages session boundaries.
                Default: `SessionMode.AUTO`.
            session_sample_rate (float): Session sampling rate (0.0–1.0).
                Default: `1.0`.
            session_inactivity_timeout_ms (int): Inactivity timeout (milliseconds)
                before AUTO-mode sessions expire. Default: 30 minutes.
        """
        if server_endpoint is None:
            server_endpoint = DEFAULT_TELEMETRY_ENDPOINT
        self._server_endpoint = server_endpoint
        self._channel = channel
        self._max_events = max_events
        if ping_uploader is None:
            ping_uploader = net.HttpClientUploader()
        self._ping_uploader = ping_uploader
        self._allow_multiprocessing = allow_multiprocessing
        self._enable_event_timestamps = enable_event_timestamps
        self._experimentation_id = experimentation_id
        self._enable_internal_pings = enable_internal_pings
        self._session_mode = session_mode
        self._session_sample_rate = session_sample_rate
        self._session_inactivity_timeout_ms = session_inactivity_timeout_ms

    @property
    def server_endpoint(self) -> str:
        """The server pings are sent to."""
        return self._server_endpoint

    @server_endpoint.setter
    def server_endpoint(self, value: str):
        self._server_endpoint = value

    @property
    def channel(self) -> Optional[str]:
        """The release channel the application is on, if known."""
        return self._channel

    @channel.setter
    def channel(self, value: str):
        from ._builtins import metrics

        self._channel = value

        metrics.glean.internal.metrics.app_channel.set(value)

    @property
    def max_events(self) -> int:
        """The number of events to store before force-sending."""
        return self._max_events

    # max_events can't be changed after Glean is initialized

    @property
    def enable_event_timestamps(self) -> bool:
        """Whether to add a wallclock timestamp to all events."""
        return self._enable_event_timestamps

    @property
    def experimentation_id(self) -> Optional[str]:
        """An experimentation id that will be sent in all pings"""
        return self._experimentation_id

    @property
    def enable_internal_pings(self) -> bool:
        """Whether to enable internal pings."""
        return self._enable_internal_pings

    @property
    def ping_uploader(self) -> net.BaseUploader:
        """The ping uploader implementation."""
        return self._ping_uploader

    @ping_uploader.setter
    def ping_uploader(self, value: net.BaseUploader):
        self._ping_uploader = value

    @property
    def session_mode(self) -> SessionMode:
        """How Glean manages session boundaries."""
        return self._session_mode

    @property
    def session_sample_rate(self) -> float:
        """Session sampling rate (0.0–1.0)."""
        return self._session_sample_rate

    @property
    def session_inactivity_timeout_ms(self) -> int:
        """Inactivity timeout (milliseconds) before AUTO-mode sessions expire."""
        return self._session_inactivity_timeout_ms


__all__ = ["Configuration"]
