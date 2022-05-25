# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""Top-level package for Glean SDK."""


import warnings


from pkg_resources import get_distribution, DistributionNotFound


import glean_parser  # type: ignore


from .glean import Glean
from .config import Configuration
from ._loader import load_metrics, load_pings


__version__: str = "unknown"
try:
    __version__ = str(get_distribution("glean-sdk").version)
except DistributionNotFound:  # pragma: no cover
    pass


__author__ = "The Glean Team"
__email__ = "glean-team@mozilla.com"


GLEAN_PARSER_VERSION = "6.0.1"


if glean_parser.__version__ != GLEAN_PARSER_VERSION:
    warnings.warn(
        f"glean_sdk expected glean_parser v{GLEAN_PARSER_VERSION}, "
        f"found v{glean_parser.__version__}",
        Warning,
    )


__all__ = [
    "__author__",
    "__email__",
    "__version__",
    "Glean",
    "Configuration",
    "load_metrics",
    "load_pings",
]


# Tell pdoc3 to ignore the libglean_ffi.so, which is a Rust shared library, not
# a Python extension module.
__pdoc__ = {"libglean_ffi": False}
