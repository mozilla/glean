/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import OHHTTPStubsSwift
import XCTest

final class RidealongPingTests: XCTestCase {
    var expectation: XCTestExpectation?

    override func tearDown() {
        Glean.shared.testDestroyGleanHandle()
        expectation = nil
        tearDownStubs()
    }

    func testSendRidealongPingWithBaseline() {

        let configuration = Configuration(pingSchedule: ["baseline": ["ridealong"]])
        resetGleanDiscardingInitialPings(testCase: self, tag: "RidealongPingTests", configuration: configuration)

        // Register ping _after_ Glean has been initialized to avoid this being sent multiple times.
        _ = Ping<NoReasonCodes>(
            name: "ridealong",
            includeClientId: true,
            sendIfEmpty: true,
            preciseTimestamps: true,
            includeInfoSections: true,
            enabled: true,
            schedulesPings: [],
            reasonCodes: [],
            followsCollectionEnabled: true
        )

        // We receive a baseline ping, and a ridealong ping.
        // The order might vary.
        var pingsToReceive = ["baseline", "ridealong"]

        stubServerReceive { pingType, _ in
            XCTAssertTrue(!pingsToReceive.isEmpty, "No more pings expected")
            XCTAssertTrue(pingsToReceive.contains(pingType), "Expected ping types: \(pingsToReceive), got \(pingType)")
            pingsToReceive.removeAll(where: { $0 == pingType })

            if pingsToReceive.isEmpty {
                DispatchQueue.main.async {
                    // let the response get processed before we mark the expectation fulfilled
                    self.expectation?.fulfill()
                }
            }
        }

        // Set up the expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Pings Received")

        Glean.shared.submitPingByName("baseline")

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }
}
