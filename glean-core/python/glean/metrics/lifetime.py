# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from enum import Enum


class Lifetime(Enum):
    """
    An enumeration for the different metric lifetimes that Glean supports.

    Metric lifetimes define when a metric is reset.
    """

    # The metric is reset with each sent ping
    PING = 0

    # The metric is reset on application restart
    APPLICATION = 1

    # The metric is reset with each user profile
    USER = 2
