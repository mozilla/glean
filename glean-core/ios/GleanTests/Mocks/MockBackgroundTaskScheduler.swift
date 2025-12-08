/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit
import Glean

final class MockBackgroundTaskScheduler: BackgroundTaskScheduler, @unchecked Sendable {
    let withValidTaskIdentifier: Bool

    var calledBeginBackgroundTask = 0
    var calledEndBackgroundTask = 0

    init(withValidTaskIdentifier: Bool) {
        self.withValidTaskIdentifier = withValidTaskIdentifier
    }

    func beginBackgroundTask(
        withName taskName: String?,
        expirationHandler handler: (@MainActor @Sendable () -> Void)?
    ) -> UIBackgroundTaskIdentifier {
        calledBeginBackgroundTask += 1
        return withValidTaskIdentifier
               ? UIBackgroundTaskIdentifier(rawValue: Int.random(in: 0...Int.max))
               : .invalid
    }

    func endBackgroundTask(_ identifier: UIBackgroundTaskIdentifier) {
        calledEndBackgroundTask += 1
    }
}
