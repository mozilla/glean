/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Enumeration of different resolutions supported by
/// the `TimespanMetricType` and `TimingDistributionMetricType`.
public enum TimeUnit: Int32 {
    /// Represents nanosecond precision.
    case nanosecond = 0

    /// Represents microsecond precision.
    case microsecond = 1

    /// Represents millisecond precision.
    case millisecond = 2

    /// Represents second precision.
    case second = 3

    /// Represents minute precision.
    case minute = 4

    /// Represents hour precision.
    case hour = 5

    /// Represents day precision.
    case day = 6
}
