# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


"""
This module loads the built-in metrics and pings.
"""


from pkg_resources import resource_filename


from ._loader import load_metrics, load_pings


metrics = load_metrics(
    resource_filename(__name__, "metrics.yaml"), config={"allow_reserved": True}
)


pings = load_pings(
    resource_filename(__name__, "pings.yaml"), config={"allow_reserved": True}
)


__all__ = ["metrics", "pings"]
