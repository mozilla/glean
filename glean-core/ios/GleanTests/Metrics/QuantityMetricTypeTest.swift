/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class QuantityMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testCounterSavesToStorage() {
        let quantityMetric = QuantityMetricType(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(quantityMetric.testHasValue())

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue())
        XCTAssertEqual(1, try quantityMetric.testGetValue())

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
        XCTAssert(quantityMetric.testHasValue())
        XCTAssertEqual(10, try quantityMetric.testGetValue())
    }

    func testCounterMustNotRecordIfDisabled() {
        let quantityMetric = QuantityMetricType(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(quantityMetric.testHasValue())

        quantityMetric.add(1)

        XCTAssertFalse(quantityMetric.testHasValue(), "Quantities must not be recorded if they are disabled")
    }

    func testCounterGetValueThrowsExceptionIfNothingIsStored() {
        let quantityMetric = QuantityMetricType(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try quantityMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testCounterSavesToSecondaryPings() {
        let quantityMetric = QuantityMetricType(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue("store2"))
        XCTAssertEqual(1, try quantityMetric.testGetValue("store2"))

        quantityMetric.set(10)
        // Check that the metric was properly overwritten.
        XCTAssert(quantityMetric.testHasValue("store2"))
        XCTAssertEqual(10, try quantityMetric.testGetValue("store2"))
    }

    func testNegativeValuesAreNotCounted() {
        let quantityMetric = QuantityMetricType(
            category: "telemetry",
            name: "quantity_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        quantityMetric.set(1)

        // Check that the metric was properly recorded.
        XCTAssert(quantityMetric.testHasValue("store1"))
        XCTAssertEqual(1, try quantityMetric.testGetValue("store1"))

        quantityMetric.set(-10)
        // Check that the metric was NOT recorded.
        XCTAssert(quantityMetric.testHasValue("store1"))
        XCTAssertEqual(1, try quantityMetric.testGetValue("store1"))
        XCTAssertEqual(1, quantityMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
