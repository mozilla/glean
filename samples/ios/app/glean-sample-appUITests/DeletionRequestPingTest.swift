/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import Swifter
import XCTest

// swiftlint:disable force_cast
// REASON: Used in below test cases to cause errors if data is missing
class DeletionRequestPingTest: XCTestCase {
    var app: XCUIApplication!
    var expectation: XCTestExpectation?
    var lastPingJson: [String: Any]?

    override func setUp() {
        // In UI tests it is usually best to stop immediately when a failure occurs.
        continueAfterFailure = false

        // UI tests must launch the application that they test.
        // Doing this in setup will make sure it happens for each test method.
        app = XCUIApplication()
    }

    override func tearDown() {
        self.lastPingJson = nil
        self.expectation = nil
    }

    func setupServer(expectPingType: String, port: Int = 0) -> HttpServer {
        return mockServer(expectPingType: expectPingType, port: UInt16(port)) { json in
            self.lastPingJson = json
            // Fulfill test's expectation once we parsed the incoming data.
            self.expectation?.fulfill()
        }
    }

    func testDeletionRequestPing() {
        var server = setupServer(expectPingType: "deletion-request")
        expectation = expectation(description: "Completed upload")
        let port = try! server.port()

        app.launchArguments = ["USE_MOCK_SERVER", "\(port)"]
        app.launch()

        app.switches.firstMatch.tap()

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        var clientInfo = lastPingJson!["client_info"] as! [String: Any]
        let clientId = clientInfo["client_id"] as! String
        XCTAssertNotEqual(clientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")

        // Test deletion-request legacy id payload
        let metrics = lastPingJson!["metrics"] as! [String: Any]
        let uuids = metrics["uuid"] as! [String: Any]
        let legacyId = uuids["legacy_ids.client_id"] as! String
        XCTAssertEqual("01234567-89ab-cdef-0123-456789abcdef", legacyId)

        server.stop()

        // Try re-enabling and waiting for next baseline ping
        server = setupServer(expectPingType: "baseline", port: port)
        expectation = expectation(description: "Completed upload")

        app.switches.firstMatch.tap()

        // Trigger baseline ping by putting app into the background
        XCUIDevice.shared.press(XCUIDevice.Button.home)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        clientInfo = lastPingJson!["client_info"] as! [String: Any]
        let newClientId = clientInfo["client_id"] as! String
        XCTAssertNotEqual(newClientId, clientId)
        XCTAssertNotEqual(newClientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")

        server.stop()
    }
}
