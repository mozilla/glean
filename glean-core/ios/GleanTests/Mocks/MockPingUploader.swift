/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import XCTest

@testable import Glean

final class MockPingUploader: PingUploader, @unchecked Sendable {
    var uploadRequested: (CapablePingUploadRequest) -> Void

    init(uploadRequested: @escaping (CapablePingUploadRequest) -> Void) {
        self.uploadRequested = uploadRequested
    }

    func upload(request: CapablePingUploadRequest, callback: @escaping @Sendable (UploadResult) -> Void) {
        // Skip calling the regular callback for this mock's testing purposes; the global Glean object may not
        // be initialized (which will cause a crash).
        uploadRequested(request)
    }
}
