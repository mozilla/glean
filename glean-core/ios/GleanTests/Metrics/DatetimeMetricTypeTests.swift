/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class DatetimeMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testDatetimeSavesToStorage() {
        let datetimeMetric = DatetimeMetricType(
            category: "telemetry",
            name: "datetime_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        let timeZone = TimeZone(identifier: "America/Los_Angeles")
        let value = DateComponents(
            calendar: Calendar.current,
            timeZone: timeZone,
            year: 2004, month: 12, day: 9, hour: 8, minute: 3, second: 29
        )
        datetimeMetric.set(components: value)
        XCTAssertEqual("2004-12-09T08:03-08:00", try datetimeMetric.testGetValueAsString())

        let timeZone2 = TimeZone(abbreviation: "GMT")
        let value2 = DateComponents(
            calendar: Calendar.current,
            timeZone: timeZone2,
            year: 1993, month: 2, day: 23, hour: 9, minute: 5, second: 43
        )
        datetimeMetric.set(components: value2)
        XCTAssertEqual("1993-02-23T09:05+00:00", try datetimeMetric.testGetValueAsString())

        // A date prior to the UNIX epoch
        let timeZone3 = TimeZone(abbreviation: "GMT-12")
        let value3 = DateComponents(
            calendar: Calendar.current,
            timeZone: timeZone3,
            year: 1969, month: 8, day: 20, hour: 20, minute: 17, second: 3
        )
        datetimeMetric.set(components: value3)
        XCTAssertEqual("1969-08-20T20:17-12:00", try datetimeMetric.testGetValueAsString())
    }

    func testDatetimeMustNotRecordIfDisabled() {
        let datetimeMetric = DatetimeMetricType(
            category: "telemetry",
            name: "datetime_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(datetimeMetric.testHasValue())

        datetimeMetric.set()

        XCTAssertFalse(datetimeMetric.testHasValue(), "Datetimes must not be recorded if they are disabled")
    }

    func testDatetimeGetValueThrowsExceptionIfNothingIsStored() {
        let datetimeMetric = DatetimeMetricType(
            category: "telemetry",
            name: "datetime_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try datetimeMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testDatetimeSavesToSecondaryPings() {
        let datetimeMetric = DatetimeMetricType(
            category: "telemetry",
            name: "datetime_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        let timeZone = TimeZone(identifier: "America/Los_Angeles")
        let value = DateComponents(
            calendar: Calendar.current,
            timeZone: timeZone,
            year: 2004,
            month: 12,
            day: 9,
            hour: 8,
            minute: 3,
            second: 29
        )
        datetimeMetric.set(components: value)
        XCTAssertTrue(datetimeMetric.testHasValue("store2"))
        XCTAssertEqual("2004-12-09T08:03-08:00", try datetimeMetric.testGetValueAsString("store2"))
    }
}
