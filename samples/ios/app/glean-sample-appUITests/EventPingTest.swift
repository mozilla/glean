/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import Swifter
import XCTest

// swiftlint:disable force_cast
// REASON: Used in below test cases to cause errors if data is missing
class EventPingTest: XCTestCase {
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

    // We launch the app, tap the record button a couple of times,
    // then restart the app, which should trigger an event ping.
    func testValidateEventPing() {
        let server = setupServer(expectPingType: "events")
        let port = try! server.port()
        expectation = expectation(description: "Completed upload (event ping)")

        app.launchArguments = ["USE_MOCK_SERVER", "\(port)"]
        app.launch()

        let recordButton = app.buttons["Record"]

        // 3 taps, quickly
        recordButton.tap()
        recordButton.tap()
        recordButton.tap()

        // one tap after a while
        sleep(1)
        recordButton.tap()

        // We need to send the ping to clear out old values.
        let sendButton = app.buttons["Send"]
        sendButton.tap()

        // Trigger the event ping by putting app into the background
        XCUIDevice.shared.press(XCUIDevice.Button.home)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let pingInfo = lastPingJson!["ping_info"] as! [String: Any]
        let reason = pingInfo["reason"] as! String
        XCTAssertEqual("inactive", reason, "Should have gotten a inactive events ping")

        let events = lastPingJson!["events"] as! [[String: Any]]
        // 4 taps total, per button tap we record 2 events.
        let expectedCount = 4 * 2
        XCTAssertEqual(expectedCount, events.count, "Events ping should have all button-tap events")

        let firstEvent = events[0]
        XCTAssertEqual(0, firstEvent["timestamp"] as! Int, "First event should be at timestamp 0")

        for i in 1...(expectedCount-1) {
            let earlier = events[i-1]["timestamp"] as! Int
            let this = events[i]["timestamp"] as! Int
            XCTAssert(earlier <= this, "Events should be ordered monotonically non-decreasing")
        }

        // 2 events per tap,
        // we want the last event of the third tap,
        // and the first event of the fourth tap.
        let lastOfThree = (3 * 2) - 1
        let firstOfFour = lastOfThree + 1
        let notLast = events[lastOfThree]["timestamp"] as! Int
        let last = events[firstOfFour]["timestamp"] as! Int
        let diff = last - notLast
        // Sleeping and tapping the button has a delay of ~600ms,
        // so we account for a tiny bit more here.
        XCTAssert(diff >= 1000 && diff <= 2000,
                  "Last event should be a second after the second-to-last event (actual diff: \(diff)")

        server.stop()
    }
}
