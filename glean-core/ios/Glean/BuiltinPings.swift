/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// TODO: This file should be auto-generated from a `pings.yaml`

import Foundation

class Pings {
    public static let shared = Pings()

    let baseline = Ping(name: "baseline", includeClientId: true)
    let metrics = Ping(name: "metrics", includeClientId: true)
    let events = Ping(name: "events", includeClientId: true)

    private init() {
        // intentionally left private, no external user can instantiate a new global object.
    }
}
