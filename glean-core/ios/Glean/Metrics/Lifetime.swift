/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// Enumeration of different metric lifetimes.
extension Lifetime {
    var rawValue: Int32 {
        switch self {
        case .ping: return 0
        case .application: return 1
        case .user: return 2
        }
    }
}
