/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

namespace Mozilla.Glean.Private
{
    /// <summary>
    /// Enumeration of different resolutions supported by the MemoryDistribution metric type.
    ///
    /// These use the power-of-2 values of these units, that is, Kilobyte is pedantically a Kibibyte.
    /// </summary>
    public enum MemoryUnit: int
    {
        ///<summary>Byte: 1 byte.</summary>
        Byte,

        ///<summary>Kilobyte: 2^10 bytes.</summary>
        Kilobyte,

        ///<summary>Megabyte: 2^20 bytes.</summary>
        Megabyte,

        ///<summary>Gigabyte: 2^30 bytes</summary>
        Gigabyte,
    }
}
