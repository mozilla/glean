/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class StringMetricTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testStringSavesToStorage() {
        let stringMetric = StringMetricType(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(stringMetric.testHasValue())

        stringMetric.set("value")

        XCTAssert(stringMetric.testHasValue())
        XCTAssertEqual("value", try stringMetric.testGetValue())

        stringMetric.set("overridenValue")
        XCTAssert(stringMetric.testHasValue())
        XCTAssertEqual("overridenValue", try stringMetric.testGetValue())
    }

    func testStringMustNotRecordIfDisabled() {
        let stringMetric = StringMetricType(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(stringMetric.testHasValue())

        stringMetric.set("value")

        XCTAssertFalse(stringMetric.testHasValue(), "Strings must not be recorded if they are disabled")
    }

    func testStringGetValueThrowsExceptionIfNothingIsStored() {
        let stringMetric = StringMetricType(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try stringMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testStringSavesToSecondaryPings() {
        let stringMetric = StringMetricType(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        stringMetric.set("value")

        XCTAssert(stringMetric.testHasValue("store2"))
        XCTAssertEqual("value", try stringMetric.testGetValue("store2"))

        stringMetric.set("overridenValue")

        XCTAssert(stringMetric.testHasValue("store2"))
        XCTAssertEqual("overridenValue", try stringMetric.testGetValue("store2"))
    }

    func testLongStringRecordsAnError() {
        let stringMetric = StringMetricType(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        stringMetric.set(String(repeating: "0123456789", count: 11))

        XCTAssertEqual(1, stringMetric.testGetNumRecordedErrors(.invalidOverflow))
    }
}
