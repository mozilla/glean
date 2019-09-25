/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class BooleanMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testBooleanSavesToStorage() {
        let booleanMetric = BooleanMetricType(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(booleanMetric.testHasValue())

        // Record two booleans of the same type, with a little delay.
        booleanMetric.set(true)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testHasValue())
        XCTAssertTrue(try booleanMetric.testGetValue())

        booleanMetric.set(false)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testHasValue())
        XCTAssertFalse(try booleanMetric.testGetValue())
    }

    func testBooleanMustNotRecordIfDisabled() {
        let booleanMetric = BooleanMetricType(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(booleanMetric.testHasValue())

        booleanMetric.set(true)

        XCTAssertFalse(booleanMetric.testHasValue(), "Booleans must not be recorded if they are disabled")
    }

    func testBooleanGetValueThrowsExceptionIfNothingIsStored() {
        let booleanMetric = BooleanMetricType(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try booleanMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testBooleanSavesToSecondaryPings() {
        let booleanMetric = BooleanMetricType(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        // Record two booleans of the same type, with a little delay.
        booleanMetric.set(true)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testHasValue("store2"))
        XCTAssertTrue(try booleanMetric.testGetValue("store2"))

        booleanMetric.set(false)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testHasValue("store2"))
        XCTAssertFalse(try booleanMetric.testGetValue("store2"))
    }
}
