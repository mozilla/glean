# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from enum import Enum


class ErrorType(Enum):
    """
    An enumeration for the types of metric errors that Glean records.
    """

    INVALID_VALUE = 0
    """
    For when the value to be recorded does not match the metric-specific
    restrictions
    """

    INVALID_LABEL = 1
    """
    For when the label of a labeled metric does not match the restrictions
    """

    INVALID_STATE = 2
    """
    For when timings are recorded incorrectly
    """

    INVALID_OVERFLOW = 3
    """
    For when the value to be recorded overflows the metric-specific upper range
    """
