/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class PingTests: XCTestCase {
    var expectation: XCTestExpectation?
    var lastPingJson: [String: Any]?

    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    override func tearDown() {
        lastPingJson = nil
        expectation = nil
    }

    private func setupHttpResponseStub(statusCode: Int32 = 200) {
        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
            let json = try! JSONSerialization.jsonObject(with: body!, options: []) as? [String: Any]
            XCTAssert(json != nil)
            self.lastPingJson = json

            // Fulfill test's expectation once we parsed the incoming data.
            self.expectation?.fulfill()

            // Ensure a response so that the uploader does its job.
            return OHHTTPStubsResponse(
                jsonObject: [],
                statusCode: statusCode,
                headers: ["Content-Type": "application/json"]
            )
        }
    }

    func testSendingOfCustomPings() {
        let customPing = Ping(name: "custom", includeClientId: true, sendIfEmpty: false)

        let counter = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["custom"],
            lifetime: .application,
            disabled: false
        )

        setupHttpResponseStub()
        expectation = expectation(description: "Completed upload")

        counter.add()
        XCTAssert(counter.testHasValue())

        customPing.submit()

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let pingInfo = lastPingJson?["ping_info"] as? [String: Any]
        XCTAssertEqual(pingInfo?["ping_type"] as? String, "custom")

        let clientInfo = lastPingJson?["client_info"] as? [String: Any]
        XCTAssertNotNil(clientInfo?["client_id"] as? String)
    }

    func testSendingOfCustomPingsWithoutClientId() {
        let customPing = Ping(name: "custom", includeClientId: false, sendIfEmpty: false)

        let counter = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["custom"],
            lifetime: .application,
            disabled: false
        )

        setupHttpResponseStub()
        expectation = expectation(description: "Completed upload")

        counter.add()
        XCTAssert(counter.testHasValue())

        customPing.submit()

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let pingInfo = lastPingJson?["ping_info"] as? [String: Any]
        XCTAssertEqual(pingInfo?["ping_type"] as? String, "custom")

        let clientInfo = lastPingJson?["client_info"] as? [String: Any]
        XCTAssertNil(clientInfo?["client_id"] as? String)
    }

    func testSendingPingWithUnknownNameIsANoOp() {
        let counter = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["unknown", "baseline"],
            lifetime: .application,
            disabled: false
        )

        counter.add()
        XCTAssert(counter.testHasValue())

        setupHttpResponseStub()
        // Fail if the server receives data
        expectation = expectation(description: "Completed unexpected upload")
        expectation?.isInverted = true

        Glean.shared.submitPingsByName(pingNames: ["unknown"])

        // We wait for a timeout to happen, as we don't expect any data to be sent.
        waitForExpectations(timeout: 5.0) { _ in
            XCTAssert(true, "Test didn't time out when it should")
        }
    }

    func testRegistryShouldContainBuiltinPings() {
        XCTAssert(Glean.shared.testHasPingType("metrics"))
        XCTAssert(Glean.shared.testHasPingType("events"))
        XCTAssert(Glean.shared.testHasPingType("baseline"))
    }
}
