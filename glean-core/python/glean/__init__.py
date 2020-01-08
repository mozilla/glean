# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""Top-level package for Glean SDK."""

from pkg_resources import get_distribution, DistributionNotFound


from .glean import Glean
from .config import Configuration
from ._loader import load_metrics, load_pings


__version__ = "unknown"  # type: str
try:
    __version__ = str(get_distribution("glean-sdk").version)
except DistributionNotFound:  # pragma: no cover
    pass


__author__ = "The Glean Team"
__email__ = "glean-team@mozilla.com"


__all__ = [
    "__author__",
    "__email__",
    "__version__",
    "Glean",
    "Configuration",
    "load_metrics",
    "load_pings",
]
