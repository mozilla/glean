/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class CounterMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "CounterMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testCounterSavesToStorage() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(counterMetric.testGetValue())

        // Add to the counter a couple of times with a little delay.  The first call will check
        // calling add() without parameters to test increment by 1.
        counterMetric.add()

        // Check that the count was incremented and properly recorded.
        XCTAssertEqual(1, counterMetric.testGetValue())

        counterMetric.add(10)
        // Check that count was incremented and properly recorded.  This second call will check
        // calling add() with 10 to test increment by other amount
        XCTAssertEqual(11, counterMetric.testGetValue())
    }

    func testCounterMustNotRecordIfDisabled() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(counterMetric.testGetValue())

        counterMetric.add(1)

        XCTAssertNil(counterMetric.testGetValue(), "Counters must not be recorded if they are disabled")
    }

    func testCounterGetValueReturnsNilIfNothingIsStored() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(counterMetric.testGetValue())
    }

    func testCounterSavesToSecondaryPings() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        counterMetric.add(1)
        XCTAssertEqual(1, counterMetric.testGetValue("store2"))

        counterMetric.add(10)
        XCTAssertEqual(11, counterMetric.testGetValue("store2"))
    }

    func testNegativeValuesAreNotCounted() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        // Increment to 1 (initial value)
        counterMetric.add()

        // Check that the count was incremented
        XCTAssertEqual(1, counterMetric.testGetValue("store1"))

        counterMetric.add(-10)
        // Check that count was NOT incremented.
        XCTAssertEqual(1, counterMetric.testGetValue("store1"))
        XCTAssertEqual(1, counterMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
