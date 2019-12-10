/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class GleanTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
        Glean.shared.enableTestingMode()
    }

    override func tearDown() {
        Glean.shared.setUploadEnabled(true)
    }

    func testInitializeGlean() {
        // Glean is already initialized by the `setUp()` function
        XCTAssert(Glean.shared.isInitialized(), "Glean should be initialized")
        XCTAssert(Glean.shared.getUploadEnabled(), "Upload is enabled by default")
    }
}
