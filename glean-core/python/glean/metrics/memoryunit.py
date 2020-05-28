# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from enum import IntEnum


from .. import _ffi


class MemoryUnit(IntEnum):
    """
    An enumeration of different resolutions supported by the
    `glean.metrics.MemoryDistribution` metric type.

    These use the power-of-2 values of these units, that is, Kilobyte is
    pedantically a Kibibyte.
    """

    BYTE = _ffi.lib.MemoryUnit_Byte
    """
    Byte: 1 byte.
    """

    KILOBYTE = _ffi.lib.MemoryUnit_Kilobyte
    """
    Kilobyte: 2^10 bytes
    """

    MEGABYTE = _ffi.lib.MemoryUnit_Megabyte
    """
    Megabyte: 2^20 bytes
    """

    GIGABYTE = _ffi.lib.MemoryUnit_Gigabyte
    """
    Gigabyte: 2^30 bytes
    """
