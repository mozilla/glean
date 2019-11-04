/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import Swifter
import XCTest

// swiftlint:disable force_cast
// REASON: Used in below test cases to cause errors if data is missing
class BaselinePingTest: XCTestCase {
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

    func testValidateBaselinePing() {
        setupServer(expectPingType: "baseline")
        expectation = expectation(description: "Completed upload")

        app.launchArguments = ["USE_MOCK_SERVER"]
        app.launch()

        // Wait for 1 second: this should guarantee we have some valid duration in the
        // ping.
        sleep(1)

        // Trigger baseline ping by putting app into the background
        XCUIDevice.shared.press(XCUIDevice.Button.home)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let pingInfo = lastPingJson!["ping_info"] as! [String: Any]
        XCTAssertEqual(pingInfo["ping_type"] as! String, "baseline")

        let metrics = lastPingJson!["metrics"] as! [String: Any]

        // Make sure we have a 'duration' field with a reasonable value: it should be >= 1, since
        // we slept for 1000ms.
        let timespans = metrics["timespan"] as! [String: Any]
        let duration = timespans["glean.baseline.duration"] as! [String: Any]
        let durationValue = duration["value"] as! UInt64
        XCTAssertTrue(durationValue >= 1)

        // Make sure there's no errors.
        let errors = metrics["labeled_counter"] as? [String: Any]

        for (id, _) in errors ?? [:] {
            XCTAssert(id.starts(with: "glean.error."))
        }
    }
}
