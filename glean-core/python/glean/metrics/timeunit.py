# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from enum import IntEnum


class TimeUnit(IntEnum):
    """
    An enumeration of different resolutions supported by the time-related
    metric types.
    """

    NANOSECOND = 0
    """
    Represents nanosecond precision.
    """

    MICROSECOND = 1
    """
    Represents microsecond precision.
    """

    MILLISECOND = 2
    """
    Represents millisecond precision.
    """

    SECOND = 3
    """
    Represents second precision.
    """

    MINUTE = 4
    """
    Represents minute precision.
    """

    HOUR = 5
    """
    Represents hour precision.
    """

    DAY = 6
    """
    Represents day precision.
    """
