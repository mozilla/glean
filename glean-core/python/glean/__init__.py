# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""Top-level package for Glean SDK."""

import warnings


import importlib.metadata
from semver import VersionInfo  # type: ignore


import glean_parser  # type: ignore


from .glean import Glean
from .config import Configuration
from ._loader import load_metrics, load_pings


__version__: str = "unknown"
try:
    __version__ = importlib.metadata.version("glean-sdk")
except importlib.metadata.PackageNotFoundError:  # pragma: no cover
    pass


__author__ = "The Glean Team"
__email__ = "glean-team@mozilla.com"


GLEAN_PARSER_VERSION = "17.2.0"
parser_version = VersionInfo.parse(GLEAN_PARSER_VERSION)
parser_version_next_major = parser_version.bump_major()


if "dev" not in glean_parser.__version__:
    current_parser = VersionInfo.parse(glean_parser.__version__)
    if current_parser < parser_version or current_parser >= parser_version_next_major:
        warnings.warn(
            f"glean_sdk expected glean_parser ~= v{GLEAN_PARSER_VERSION}, "
            f"found v{glean_parser.__version__}",
            Warning,
            stacklevel=1,
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
