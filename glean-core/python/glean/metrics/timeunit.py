# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from enum import IntEnum


from .. import _ffi


class TimeUnit(IntEnum):
    """
    An enumeration of different resolutions supported by the time-related
    metric types.
    """

    NANOSECOND = _ffi.lib.TimeUnit_Nanosecond
    """
    Represents nanosecond precision.
    """

    MICROSECOND = _ffi.lib.TimeUnit_Microsecond
    """
    Represents microsecond precision.
    """

    MILLISECOND = _ffi.lib.TimeUnit_Millisecond
    """
    Represents millisecond precision.
    """

    SECOND = _ffi.lib.TimeUnit_Second
    """
    Represents second precision.
    """

    MINUTE = _ffi.lib.TimeUnit_Minute
    """
    Represents minute precision.
    """

    HOUR = _ffi.lib.TimeUnit_Hour
    """
    Represents hour precision.
    """

    DAY = _ffi.lib.TimeUnit_Day
    """
    Represents day precision.
    """
