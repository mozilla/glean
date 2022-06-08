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

        XCTAssertNil(quantityMetric.testGetValue())

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssertEqual(1, quantityMetric.testGetValue())

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
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

        XCTAssertNil(quantityMetric.testGetValue())

        quantityMetric.set(1)

        XCTAssertNil(quantityMetric.testGetValue(), "Quantities must not be recorded if they are disabled")
    }

    func testCounterGetValueReturnsNilIfNothingIsStored() {
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
        XCTAssertEqual(1, quantityMetric.testGetValue("store2"))

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
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
        XCTAssertEqual(1, quantityMetric.testGetValue("store1"))

        quantityMetric.set(-10)
        // Check that the metric was NOT recorded.
        XCTAssertEqual(1, quantityMetric.testGetValue("store1"))
        XCTAssertEqual(1, quantityMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
