/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class ConfigurationTests: XCTestCase {
    private var config: Configuration?

    override func setUp() {
        config = Configuration()
    }

    override func tearDown() {
        config = nil
    }

    func testInit() {
        XCTAssertEqual(
            config?.serverEndpoint,
            Configuration.Constants.defaultTelemetryEndpoint,
            "Default endpoint is set"
        )
        XCTAssertNil(
            config?.maxEvents,
            "Default max events are set"
        )
        XCTAssertNil(
            config?.channel,
            "Default channel is set"
        )
    }
}
