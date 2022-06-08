/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class StringMetricTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "StringMetricTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testStringSavesToStorage() {
        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(stringMetric.testGetValue())

        stringMetric.set("value")

        XCTAssertEqual("value", stringMetric.testGetValue())

        stringMetric.set("overridenValue")
        XCTAssertEqual("overridenValue", stringMetric.testGetValue())
    }

    func testStringMustNotRecordIfDisabled() {
        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(stringMetric.testGetValue())

        stringMetric.set("value")

        XCTAssertNil(stringMetric.testGetValue(), "Strings must not be recorded if they are disabled")
    }

    func testStringGetValueReturnsNilIfNothingIsStored() {
        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(stringMetric.testGetValue())
    }

    func testStringSavesToSecondaryPings() {
        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        stringMetric.set("value")

        XCTAssertEqual("value", stringMetric.testGetValue("store2"))

        stringMetric.set("overridenValue")

        XCTAssertEqual("overridenValue", stringMetric.testGetValue("store2"))
    }

    func testLongStringRecordsAnError() {
        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "string_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        stringMetric.set(String(repeating: "0123456789", count: 11))

        XCTAssertEqual(1, stringMetric.testGetNumRecordedErrors(.invalidOverflow))
    }
}
