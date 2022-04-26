/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Enumeration of different resolutions supported by
/// the `MemoryDistributionMetricType`.
///
/// These use the power-of-2 values of these units, that is, Kilobyte is pedantically a Kibibyte.
public enum MemoryUnit: Int32 {
    /// 1
    case byte = 0

    /// 2^10
    case kilobyte = 1

    /// 2^20
    case megabyte

    /// 2^30
    case gigabyte
}
