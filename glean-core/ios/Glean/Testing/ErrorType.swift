/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Enumeration of different metric lifetimes.
public enum ErrorType: Int32 {
    /// For when the value to be recorded does not match the metric-specific restrictions
    case invalidValue = 0

    /// For when the label of a labeled metric does not match the restrictions
    case invalidLabel = 1

    /// For when timings are recorded incorrectly
    case invalidState = 2

    /// For when the value to be recorded overflows the metric-specific upper range
    case invalidOverflow = 3
}
