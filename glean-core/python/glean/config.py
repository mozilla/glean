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


@dataclasses.dataclass
class Configuration:
    """
    Configuration values for Glean.
    """

    # The server pings are sent to.
    server_endpoint: str = DEFAULT_TELEMETRY_ENDPOINT

    # The user agent used when sending pings.
    user_agent: Optional[str] = None

    # The release channel the application is on, if known.
    channel: Optional[str] = None

    # The number of events to store before force-sending.
    max_events: int = DEFAULT_MAX_EVENTS

    # Whether to log ping contents to the console.
    log_pings: bool = False

    # String tag to be applied to headers when uploading pings for debug view.
    ping_tag: Optional[str] = None

    # The ping uploader implementation
    ping_uploader: net.BaseUploader = net.HttpClientUploader()

    def __post_init__(self):
        # It would be preferable to use a field(default_factory=...) for this,
        # but that breaks our documentation generation due to this bug:
        #   https://github.com/pdoc3/pdoc/issues/116

        if self.user_agent is None:
            import glean

            self.user_agent = f"Glean/{glean.__version__} (Python on {sys.platform})"


__all__ = ["Configuration"]
