/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class GleanTests: XCTestCase {
    override func setUp() {
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDown() {
        Glean.shared.setUploadEnabled(true)
    }

    func testInitializeGlean() {
        let glean = Glean.shared

        glean.initialize()
        XCTAssert(glean.isInitialized(), "Glean should be initialized")
        XCTAssert(glean.getUploadEnabled(), "Upload is enabled by default")
    }
}
