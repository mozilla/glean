/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class GleanDebugUtilityTests: XCTestCase {
    var expectation: XCTestExpectation?

    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanDebugUtilityTests")
    }

    override func tearDown() {
        expectation = nil
        tearDownStubs()
    }

    func testSmokeHandleCustomUrlLogPings() {
        // The exact values are not accessible to Swift,
        // so all we do is check that this doesn't crash.

        // Test toggle true
        var url = URL(string: "test://glean?logPings=true")
        Glean.shared.handleCustomUrl(url: url!)

        // Test invalid value doesn't cause setting to toggle
        url = URL(string: "test://glean?logPings=Not-a-bool")
        Glean.shared.handleCustomUrl(url: url!)

        // Test toggle false
        url = URL(string: "test://glean?logPings=false")
        Glean.shared.handleCustomUrl(url: url!)

        // Test invalid value doesn't cause setting to toggle
        url = URL(string: "test://glean?logPings=Not-a-bool")
        Glean.shared.handleCustomUrl(url: url!)

        // This should NOT set the logPings to true or false because it doesn't
        // match the required host "glean".
        url = URL(string: "test://not-glean?logPings=true")
        Glean.shared.handleCustomUrl(url: url!)
    }

    func testHandleCustomUrlSendPing() {
        expectation = expectation(description: "Ping sent")
        // This test will be sending one each of baseline, events, and metrics pings
        // so we set the expected count to 3 and set it to assert for overfulfill in order
        // to test that unknown pings aren't being sent.
        expectation!.expectedFulfillmentCount = 3
        expectation!.assertForOverFulfill = true
        let expectedPings = ["baseline", "events", "metrics"]
        stubServerReceive { pingType, _ in
            XCTAssertTrue(expectedPings.contains(pingType), "\(pingType) should be valid")

            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        // Create a dummy event and a dummy metric so that the
        // respective pings will be sent
        let event = EventMetricType<NoExtras>(CommonMetricData(
            category: "ui",
            name: "click",
            sendInPings: ["events"],
            lifetime: .ping,
            disabled: false
        ), ["object_id", "other"])
        event.record()

        let metric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))
        metric.add()

        // Send the baseline ping via the custom URL
        var url = URL(string: "test://glean?sendPing=baseline")
        Glean.shared.handleCustomUrl(url: url!)

        // Send the events ping via the custom URL
        url = URL(string: "test://glean?sendPing=events")
        Glean.shared.handleCustomUrl(url: url!)

        // Send the metrics ping via the custom URL
        url = URL(string: "test://glean?sendPing=metrics")
        Glean.shared.handleCustomUrl(url: url!)

        // Sending a non-registered ping does nothing, if it did it would cause
        // the assert on overfulfull to trigger
        url = URL(string: "test://glean?sendPing=no-such-ping")
        Glean.shared.handleCustomUrl(url: url!)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for baseline ping: \(error!)")
        }
    }
}
