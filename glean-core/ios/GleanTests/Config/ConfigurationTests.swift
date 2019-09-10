/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import XCTest
@testable import Glean

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
            Configuration.DEFAULT_TELEMETRY_ENDPOINT,
            "Default endpoint is set"
        )
        XCTAssertEqual(
            config?.userAgent,
            Configuration.DEFAULT_USER_AGENT,
            "Default UserAgent is set"
        )
        XCTAssertEqual(
            config?.connectionTimeout,
            Configuration.DEFAULT_CONNECTION_TIMEOUT,
            "Default connection timeout is set"
        )
        XCTAssertEqual(
            config?.readTimeout,
            Configuration.DEFAULT_READ_TIMEOUT,
            "Default read timeout is set"
        )
        XCTAssertEqual(
            config?.logPings,
            Configuration.DEFAULT_LOG_PINGS,
            "Default log pings is set"
        )
        // TODO test config?.httpClient
        XCTAssertNil(
            config?.maxEvents,
            "Default max events are set"
        )
        XCTAssertNil(
            config?.pingTag,
            "Default pingTag is set"
        )
        XCTAssertNil(
            config?.channel,
            "Default channel is set"
        )
    }
}
