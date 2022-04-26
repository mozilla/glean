/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Enumeration of different metric lifetimes.
public enum Lifetime: Int32 {
    /// The metric is reset with each sent ping
    case ping = 0

    /// The metric is reset on application restart
    case application = 1

    /// The metric is reset with each user profile
    case user = 2
}
