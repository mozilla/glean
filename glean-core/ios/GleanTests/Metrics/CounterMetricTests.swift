/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class CounterMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testCounterSavesToStorage() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(counterMetric.testHasValue())

        // Add to the counter a couple of times with a little delay.  The first call will check
        // calling add() without parameters to test increment by 1.
        counterMetric.add()

        // Check that the count was incremented and properly recorded.
        XCTAssert(counterMetric.testHasValue())
        XCTAssertEqual(1, try counterMetric.testGetValue())

        counterMetric.add(10)
        // Check that count was incremented and properly recorded.  This second call will check
        // calling add() with 10 to test increment by other amount
        XCTAssert(counterMetric.testHasValue())
        XCTAssertEqual(11, try counterMetric.testGetValue())
    }

    func testCounterMustNotRecordIfDisabled() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(counterMetric.testHasValue())

        counterMetric.add(1)

        XCTAssertFalse(counterMetric.testHasValue(), "Counters must not be recorded if they are disabled")
    }

    func testCounterGetValueThrowsExceptionIfNothingIsStored() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try counterMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testCounterSavesToSecondaryPings() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        counterMetric.add(1)

        XCTAssert(counterMetric.testHasValue("store2"))
        XCTAssertEqual(1, try counterMetric.testGetValue("store2"))

        counterMetric.add(10)

        XCTAssert(counterMetric.testHasValue("store2"))
        XCTAssertEqual(11, try counterMetric.testGetValue("store2"))
    }

    func testNegativeValuesAreNotCounted() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        // Increment to 1 (initial value)
        counterMetric.add()

        // Check that the count was incremented
        XCTAssert(counterMetric.testHasValue("store1"))
        XCTAssertEqual(1, try counterMetric.testGetValue("store1"))

        counterMetric.add(-10)
        // Check that count was NOT incremented.
        XCTAssert(counterMetric.testHasValue("store1"))
        XCTAssertEqual(1, try counterMetric.testGetValue("store1"))
        XCTAssertEqual(1, counterMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
