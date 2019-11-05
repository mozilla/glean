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
    var server: HttpServer?

    override func setUp() {
        // In UI tests it is usually best to stop immediately when a failure occurs.
        continueAfterFailure = false

        // UI tests must launch the application that they test.
        // Doing this in setup will make sure it happens for each test method.
        app = XCUIApplication()
    }

    override func tearDown() {
        server?.stop()
    }

    func setupServer(expectPingType: String) {
        server = HttpServer()

        server!["/submit/:appid/:ping/:schema/:pinguuid"] = { request in
            let pingName = request.params[":ping"]!
            if pingName == expectPingType {
                let body = String(bytes: request.body, encoding: .utf8)!
                let data = body.data(using: .utf8)!
                print("Received data: \(body)")
                let json = try! JSONSerialization.jsonObject(with: data, options: []) as? [String: Any]
                self.lastPingJson = json

                // Fulfill test's expectation once we parsed the incoming data.
                self.expectation?.fulfill()
            }
            return HttpResponse.ok(.text("OK"))
        }
        // For logging purposes:
        server!.middleware.append { request in
            print("Middleware: \(request.address ?? "unknown address") -> \(request.method) -> \(request.path)")
            return nil
        }

        try! server!.start(9080)
    }

    func checkCustomCounterData(expectedValue: UInt64) {
        let pingInfo = lastPingJson!["ping_info"] as! [String: Any]
        XCTAssertEqual(pingInfo["ping_type"] as! String, "sample")

        let metrics = lastPingJson!["metrics"] as! [String: Any]
        let counters = metrics["counter"] as! [String: Any]
        let value = counters["custom.counter"] as! UInt64
        XCTAssertEqual(value, expectedValue)

    }

    func testViewControllerInteraction() {
        setupServer(expectPingType: "sample")

        app.launchArguments = ["USE_MOCK_SERVER"]
        app.launch()

        let sendButton = app.buttons["Send"]
        let recordButton = app.buttons["Record"]

        // Send the sample ping
        expectation = expectation(description: "Completed upload")
        sendButton.tap()

        waitForExpectations(timeout: 5.0) { error in
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

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        checkCustomCounterData(expectedValue: 4)
    }
}
