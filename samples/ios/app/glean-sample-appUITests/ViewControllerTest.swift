/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import Swifter
import XCTest

// swiftlint:disable force_cast
// REASON: Used in below test cases to cause errors if data is missing
class ViewControllerTest: XCTestCase {
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

    func setupServer(expectPingType: String) -> HttpServer {
        return mockServer(expectPingType: expectPingType) { json in
            self.lastPingJson = json
            // Fulfill test's expectation once we parsed the incoming data.
            self.expectation?.fulfill()
        }
    }

    func checkCustomCounterData(expectedValue: UInt64) {
        let metrics = lastPingJson!["metrics"] as! [String: Any]
        let counters = metrics["counter"] as! [String: Any]
        let value = counters["custom.counter"] as! UInt64
        XCTAssertEqual(value, expectedValue)
    }

    func testViewControllerInteraction() {
        let server = setupServer(expectPingType: "sample")

        app.launchArguments = ["USE_MOCK_SERVER", "\(try! server.port())"]
        app.launch()

        let sendButton = app.buttons["Send"]
        let recordButton = app.buttons["Record"]

        // Send the sample ping
        expectation = expectation(description: "Completed upload")
        sendButton.tap()

        waitForExpectations(timeout: 10.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        checkCustomCounterData(expectedValue: 1)

        // Record a bit more data
        recordButton.tap()
        recordButton.tap()
        recordButton.tap()

        // Send the sample ping
        expectation = expectation(description: "Completed upload")
        sendButton.tap()

        waitForExpectations(timeout: 10.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        checkCustomCounterData(expectedValue: 4)

        server.stop()
    }
}
