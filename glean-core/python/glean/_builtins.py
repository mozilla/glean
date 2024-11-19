# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
This module loads the built-in metrics and pings.
"""

import sys

if sys.version_info >= (3, 9):
    import importlib.resources as importlib_resources
else:
    import importlib_resources

from ._loader import load_metrics, load_pings

# Python <3.12 makes this something like `glean._builtin`,
# above that it's just `glean`.
pkg_name = __name__.split(".")[0]

ref = importlib_resources.files(pkg_name) / "metrics.yaml"
with importlib_resources.as_file(ref) as path:
    metrics = load_metrics(path, config={"allow_reserved": True})

ref = importlib_resources.files(pkg_name) / "pings.yaml"
with importlib_resources.as_file(ref) as path:
    pings = load_pings(path, config={"allow_reserved": True})


__all__ = ["metrics", "pings"]
