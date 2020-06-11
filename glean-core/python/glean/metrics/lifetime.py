# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from enum import Enum


from .. import _ffi


class Lifetime(Enum):
    """
    An enumeration for the different metric lifetimes that Glean supports.

    Metric lifetimes define when a metric is reset.
    """

    PING = _ffi.lib.Lifetime_Ping
    """
    The metric is reset with each sent ping
    """

    APPLICATION = _ffi.lib.Lifetime_Application
    """
    The metric is reset on application restart
    """

    USER = _ffi.lib.Lifetime_User
    """
    The metric is reset with each user profile
    """
