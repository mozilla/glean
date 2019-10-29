# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""Top-level package for Glean SDK."""

from pkg_resources import get_distribution, DistributionNotFound

try:
    __version__ = get_distribution(__name__).version
except DistributionNotFound:
    # package is not installed
    pass

__author__ = "The Glean Team"
__email__ = "telemetry-client-dev@mozilla.com"

from .glean import Glean
from .config import Configuration

__all__ = ["__author__", "__email__", "__version__", "Glean", "Configuration"]
