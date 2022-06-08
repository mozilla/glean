/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class BooleanMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "BooleanMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testBooleanSavesToStorage() {
        let booleanMetric = BooleanMetricType(CommonMetricData(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(booleanMetric.testGetValue())

        // Record two booleans of the same type, with a little delay.
        booleanMetric.set(true)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testGetValue()!)

        booleanMetric.set(false)
        // Check that data was properly recorded.
        XCTAssertFalse(booleanMetric.testGetValue()!)
    }

    func testBooleanMustNotRecordIfDisabled() {
        let booleanMetric = BooleanMetricType(CommonMetricData(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(booleanMetric.testGetValue())

        booleanMetric.set(true)

        XCTAssertNil(booleanMetric.testGetValue(), "Booleans must not be recorded if they are disabled")
    }

    func testBooleanGetValueReturnsNilIfNothingIsStored() {
        let booleanMetric = BooleanMetricType(CommonMetricData(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(booleanMetric.testGetValue())
    }

    func testBooleanSavesToSecondaryPings() {
        let booleanMetric = BooleanMetricType(CommonMetricData(
            category: "telemetry",
            name: "boolean_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        // Record two booleans of the same type, with a little delay.
        booleanMetric.set(true)
        // Check that data was properly recorded.
        XCTAssertTrue(booleanMetric.testGetValue("store2")!)

        booleanMetric.set(false)
        // Check that data was properly recorded.
        XCTAssertFalse(booleanMetric.testGetValue("store2")!)
    }
}
