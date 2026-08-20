/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class CustomDistributionTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "CustomDistributionTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testTiminingDistributionSavesToStorage() {
        let metric = CustomDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "custom_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ),
            0,
            100,
            100,
            .linear
        )

        // Accumulate a few values
        metric.accumulateSamples([1, 2, 3])

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        let snapshot = metric.testGetValue()!
        let sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)

        // Check the sum
        XCTAssertEqual(1 + 2 + 3, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3])
    }

    func testCustomDistributionMustNotRecordIfDisabled() {
        let metric = CustomDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "custom_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true
            ), 0, 100, 100, .linear
        )

        metric.accumulateSamples([1])
        XCTAssertNil(metric.testGetValue())
    }

    func testCustomDistributionGetValueReturnsNilIfNothingIsStored() {
        let metric = CustomDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "custom_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
            ), 0, 100, 100, .linear
        )

        XCTAssertNil(metric.testGetValue())
    }

    func testCustomDistributionSavesToSecondaryPings() {
        // Define a custom distribution metric which will be stored in multiple stores
        let metric = CustomDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "custom_distribution",
            sendInPings: ["store1", "store2", "store3"],
            lifetime: .application,
            disabled: false
            ), 0, 100, 100, .linear
        )

        // Accumulate a few values
        metric.accumulateSamples([1, 2, 3])

        // Check that data was properly recorded in the second ping.
        var snapshot = metric.testGetValue("store2")!

        // Check the sum
        XCTAssertEqual(1+2+3, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3])

        // Check that data was properly recorded in the second ping.
        snapshot = metric.testGetValue("store3")!

        // Check the sum
        XCTAssertEqual(1+2+3, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3])
    }
}
