# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import dataclasses
import sys
from typing import Optional


from . import net


# The default server pings are sent to
DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"


# The default number of events to store before sending
DEFAULT_MAX_EVENTS = 500


def _get_default_user_agent():
    import glean

    return f"Glean/{glean.__version__} (Python on {sys.platform})"


@dataclasses.dataclass
class Configuration:
    """
    Configuration values for Glean.
    """

    server_endpoint: str = DEFAULT_TELEMETRY_ENDPOINT
    """The server pings are sent to."""

    user_agent: Optional[str] = dataclasses.field(
        default_factory=_get_default_user_agent
    )
    """The user agent used when sending pings."""

    channel: Optional[str] = None
    """The release channel the application is on, if known."""

    max_events: int = DEFAULT_MAX_EVENTS
    """The number of events to store before force-sending."""

    log_pings: bool = False
    """Whether to log ping contents to the console."""

    ping_tag: Optional[str] = None
    """String tag to be applied to headers when uploading pings for debug view."""

    ping_uploader: net.BaseUploader = net.HttpClientUploader()
    """The ping uploader implementation."""


__all__ = ["Configuration"]
