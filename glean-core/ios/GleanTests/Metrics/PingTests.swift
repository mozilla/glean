/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class PingTests: XCTestCase {
    var expectation: XCTestExpectation?
    var lastPingJson: [String: Any]?

    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "PingTests")
    }

    override func tearDown() {
        lastPingJson = nil
        expectation = nil
        tearDownStubs()
    }

    private func setupHttpResponseStub(_ expectedPingType: String) {
        stubServerReceive { pingType, json in
            XCTAssertEqual(pingType, expectedPingType, "Wrong ping type received")
            XCTAssert(json != nil)

            self.lastPingJson = json

            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }
    }

    func testSendingOfCustomPings() {
        let customPing = Ping<NoReasonCodes>(
            name: "custom",
            includeClientId: true,
            sendIfEmpty: false,
            reasonCodes: []
        )

        let counter = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["custom"],
            lifetime: .application,
            disabled: false
        ))

        setupHttpResponseStub("custom")
        expectation = expectation(description: "Completed upload")

        counter.add()
        XCTAssertNotNil(counter.testGetValue())

        var callbackWasCalled = false
        customPing.testBeforeNextSubmit { reason in
            XCTAssertNil(reason, "Unexpected reason for custom ping submitted")
            XCTAssertEqual(1, counter.testGetValue(), "Unexpected value for counter in custom ping")
            callbackWasCalled = true
        }

        customPing.submit()
        XCTAssert(callbackWasCalled, "Expected callback to be called by now.")

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let clientInfo = lastPingJson?["client_info"] as? [String: Any]
        XCTAssertEqual("iOS", clientInfo?["os"] as? String)
        XCTAssertEqual(UIDevice.current.systemVersion, clientInfo?["os_version"] as? String)
        XCTAssertNotNil(clientInfo?["client_id"] as? String)
        XCTAssertNotNil(clientInfo?["device_model"] as? String)
        XCTAssertNotNil(clientInfo?["device_manufacturer"] as? String)
        XCTAssertNotNil(clientInfo?["locale"] as? String)
    }

    func testSendingOfCustomPingsWithoutClientId() {
        let customPing = Ping<NoReasonCodes>(
            name: "custom",
            includeClientId: false,
            sendIfEmpty: false,
            reasonCodes: []
        )

        let counter = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["custom"],
            lifetime: .application,
            disabled: false
        ))

        setupHttpResponseStub("custom")
        expectation = expectation(description: "Completed upload")

        counter.add()
        XCTAssertNotNil(counter.testGetValue())

        customPing.submit()

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let clientInfo = lastPingJson?["client_info"] as? [String: Any]
        XCTAssertNil(clientInfo?["client_id"] as? String)
    }

    func testSendingPingWithUnknownNameIsANoOp() {
        let counter = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["unknown", "baseline"],
            lifetime: .application,
            disabled: false
        ))

        counter.add()
        XCTAssertNotNil(counter.testGetValue())

        setupHttpResponseStub("INVALID")
        // Fail if the server receives data
        expectation = expectation(description: "Completed unexpected upload")
        expectation?.isInverted = true

        Glean.shared.submitPingByName("unknown")

        // We wait for a timeout to happen, as we don't expect any data to be sent.
        waitForExpectations(timeout: 5.0) { _ in
            XCTAssert(true, "Test didn't time out when it should")
        }
    }

    /*
    func testRegistryShouldContainBuiltinPings() {
        XCTAssert(Glean.shared.testHasPingType("metrics"))
        XCTAssert(Glean.shared.testHasPingType("events"))
        XCTAssert(Glean.shared.testHasPingType("baseline"))
    }
    */

    func testPingWithReasonCodes() {
        enum CustomReasonCodes: Int, ReasonCodes {
            case wasTested = 0

            public func index() -> Int {
                return self.rawValue
            }
        }

        let customPing = Ping<CustomReasonCodes>(
            name: "custom2",
            includeClientId: true,
            sendIfEmpty: true,
            reasonCodes: ["was_tested"]
        )

        setupHttpResponseStub("custom2")
        expectation = expectation(description: "Completed upload")

        var callbackWasCalled = false
        customPing.testBeforeNextSubmit { reason in
            XCTAssertEqual(CustomReasonCodes.wasTested, reason, "Unexpected reason for custom ping submitted")
            callbackWasCalled = true
        }

        customPing.submit(reason: .wasTested)
        XCTAssert(callbackWasCalled, "Expected callback to be called by now.")

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }
}
