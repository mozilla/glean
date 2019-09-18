/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Enumeration of different resolutions supported by
/// the `TimespanMetricType` and `TimingDistributionMetricType`.
public enum TimeUnit: Int32 {
    /// Represents a nanosecond precision.
    case nanosecond = 0

    /// Represents a microsecond precision.
    case microsecond = 1

    /// Represents a millisecond precision.
    case millisecond = 2

    /// Represents a second precision.
    case second = 3

    /// Represents a minute precision.
    case minute = 4

    /// Represents a hour precision.
    case hour = 5

    /// Represents a day precision.
    case day = 6
}
