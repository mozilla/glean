/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class QuantityMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "QuantityMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testCounterSavesToStorage() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertFalse(quantityMetric.testHasValue())

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue())
        XCTAssertEqual(1, quantityMetric.testGetValue())

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
        XCTAssert(quantityMetric.testHasValue())
        XCTAssertEqual(10, quantityMetric.testGetValue())
    }

    func testCounterMustNotRecordIfDisabled() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertFalse(quantityMetric.testHasValue())

        quantityMetric.set(1)

        XCTAssertFalse(quantityMetric.testHasValue(), "Quantities must not be recorded if they are disabled")
    }

    func testCounterGetValueThrowsExceptionIfNothingIsStored() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(quantityMetric.testGetValue())
    }

    func testCounterSavesToSecondaryPings() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue("store2"))
        XCTAssertEqual(1, quantityMetric.testGetValue("store2"))

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
        XCTAssert(quantityMetric.testHasValue("store2"))
        XCTAssertEqual(10, quantityMetric.testGetValue("store2"))
    }

    func testNegativeValuesAreNotCounted() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue("store1"))
        XCTAssertEqual(1, quantityMetric.testGetValue("store1"))

        quantityMetric.set(-10)
        // Check that the metric was NOT recorded.
        XCTAssert(quantityMetric.testHasValue("store1"))
        XCTAssertEqual(1, quantityMetric.testGetValue("store1"))
        XCTAssertEqual(1, quantityMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
