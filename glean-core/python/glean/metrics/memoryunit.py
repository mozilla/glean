# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from enum import IntEnum


class MemoryUnit(IntEnum):
    """
    An enumeration of different resolutions supported by the
    `glean.metrics.MemoryDistribution` metric type.

    These use the power-of-2 values of these units, that is, Kilobyte is
    pedantically a Kibibyte.
    """

    BYTE = 0
    """
    Byte: 1 byte.
    """

    KILOBYTE = 1
    """
    Kilobyte: 2^10 bytes
    """

    MEGABYTE = 2
    """
    Megabyte: 2^20 bytes
    """

    GIGABYTE = 3
    """
    Gigabyte: 2^30 bytes
    """
