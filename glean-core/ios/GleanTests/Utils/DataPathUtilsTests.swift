/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class DataPathMetricTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "PingTests")
    }

    func testCannotWriteToInvalidDatabasePath() {
        let customDataPath = ""
        XCTAssertFalse(canWriteToDatabasePath(customDataPath))
    }

    func testCanWriteToValidDatabasePath() {
        let paths = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)
        let documentsDirectory = paths[0]
        let dataPath = documentsDirectory.appendingPathComponent("valid_db_path").relativePath
        XCTAssertTrue(canWriteToDatabasePath(dataPath))
    }
}
