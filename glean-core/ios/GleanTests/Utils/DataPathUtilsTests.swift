/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class DataPathMetricTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "PingTests")
    }

    func testCanWriteToDatabasePathInvalidPath() {
        let customDataPath = ""
        XCTAssertFalse(canWriteToDatabasePath(customDataPath))
    }

    func testCanWriteToDatabasePathValidPath() {
        let customDataPath = "valid_db_path"
        XCTAssertTrue(canWriteToDatabasePath(customDataPath))
    }

    func testGenerateGleanStoragePathHasCorrectSlug() {
        let customDataPath = "test_glean_data"
        let url = generateGleanStoragePath(customDataPath)
        XCTAssertEqual(customDataPath, url.lastPathComponent)
    }
}
