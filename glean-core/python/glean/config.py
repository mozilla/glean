# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import dataclasses
from typing import Optional


# The default server pings are sent to
DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"


# The default UserAgent string
def get_default_user_agent():
    # Need to use a function here to avoid a cyclical import problem
    from glean import __version__

    return f"Glean/{__version__} (Python)"


# The default number of events to store before sending
DEFAULT_MAX_EVENTS = 500


@dataclasses.dataclass
class Configuration:
    """
    Configuration values for Glean.
    """

    # The server pings are sent to.
    server_endpoint: str = DEFAULT_TELEMETRY_ENDPOINT

    # The user agent used when sending pings.
    user_agent: str = dataclasses.field(default_factory=get_default_user_agent)

    # The release channel the application is on, if known.
    channel: Optional[str] = None

    # The number of events to store before force-sending.
    max_events: int = DEFAULT_MAX_EVENTS

    # Whether to log ping contents to the console.
    log_pings: bool = False

    # String tag to be applied to headers when uploading pings for debug view.
    ping_tag: Optional[str] = None


__all__ = ["Configuration"]
