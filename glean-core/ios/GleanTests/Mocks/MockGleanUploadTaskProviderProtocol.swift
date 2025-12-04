/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import UIKit
import Glean

final class MockGleanUploadTaskProviderProtocol: GleanUploadTaskProviderProtocol, @unchecked Sendable {
    let task: PingUploadTask
    var didCallGetUploadTask = false

    init(returningTask: PingUploadTask) {
        self.task = returningTask
    }

    func getUploadTask() -> PingUploadTask {
        // Always return the expected task once, and then `.done` thereafter
        if didCallGetUploadTask {
            return .done(unused: 0)
        } else {
            didCallGetUploadTask = true
            return task
        }
    }
}
