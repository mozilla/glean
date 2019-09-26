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

    // swiftlint:disable function_parameter_count
    // REASON: Used in tests
    private func testDatetime(
        metric: DatetimeMetricType,
        testString: String,
        timeZone: TimeZone,
        year: Int,
        month: Int,
        day: Int,
        hour: Int,
        minute: Int,
        second: Int
    ) {
        let value = DateComponents(
            calendar: Calendar.current,
            timeZone: timeZone,
            year: year,
            month: month,
            day: day,
            hour: hour,
            minute: minute,
            second: second
        )
        metric.set(components: value)
        XCTAssertEqual(testString, try metric.testGetValueAsString())
        let date1 = Date.fromISO8601String(dateString: testString, precision: metric.timeUnit)
        XCTAssertEqual(date1, try metric.testGetValue())
    }

    func testDatetimeSavesToStorage() {
        let datetimeMetric = DatetimeMetricType(
            category: "telemetry",
            name: "datetime_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        testDatetime(metric: datetimeMetric,
                     testString: "2004-12-09T08:03-08:00",
                     timeZone: TimeZone(identifier: "America/Los_Angeles")!,
                     year: 2004, month: 12, day: 9, hour: 8, minute: 3, second: 29)

        testDatetime(metric: datetimeMetric,
                     testString: "1993-02-23T09:05+00:00",
                     timeZone: TimeZone(abbreviation: "GMT")!,
                     year: 1993, month: 2, day: 23, hour: 9, minute: 5, second: 43)

        testDatetime(metric: datetimeMetric,
                     testString: "1969-08-20T20:17-12:00",
                     timeZone: TimeZone(abbreviation: "GMT-12")!,
                     year: 1969, month: 8, day: 20, hour: 20, minute: 17, second: 3)
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
        let testString = "2004-12-09T08:03-08:00"
        let date = Date.fromISO8601String(dateString: testString, precision: datetimeMetric.timeUnit)
        XCTAssertEqual(date, try datetimeMetric.testGetValue("store2"))
        XCTAssertEqual(testString, try datetimeMetric.testGetValueAsString("store2"))
    }

    func testDateExtensionfromISO8601String() {
        // Bad date strings return null
        let badDate = Date.fromISO8601String(dateString: "Not a date string", precision: .minute)
        XCTAssertNil(badDate)

        // Create a an array of tuple values to facilitate testing each precision
        let dateStrings: [(String, TimeUnit)] = [
            ("2004-12-09T08:03:01.150-08:00", .nanosecond),
            ("2004-12-09T08:03:01.150-08:00", .microsecond),
            ("2004-12-09T08:03:01.150-08:00", .millisecond),
            ("2004-12-09T08:03:01-08:00", .second),
            ("2004-12-09T08:03-08:00", .minute),
            ("2004-12-09T08-08:00", .hour),
            ("2004-12-09-08:00", .day)
        ]

        for (dateString, precision) in dateStrings {
            let date = Date.fromISO8601String(dateString: dateString, precision: precision)
            // Make sure it is not nil
            XCTAssertNotNil(date)

            // Validate pieces
            let components = Calendar.current.dateComponents(in: TimeZone(abbreviation: "GMT-8")!, from: date!)
            switch precision {
            case .nanosecond:
                // This is 150000095 due to floating point imprecision and that our support ISO8601
                // doesn't really handle nano and micro seconds
                XCTAssertEqual(150_000_095, components.nanosecond ?? 0)
                fallthrough
            case .microsecond:
                XCTAssertEqual(150_000, (components.nanosecond ?? 0) / 1000)
                fallthrough
            case .millisecond:
                XCTAssertEqual(150, (components.nanosecond ?? 0) / 1_000_000)
                fallthrough
            case .second:
                XCTAssertEqual(1, components.second ?? 0)
                fallthrough
            case .minute:
                XCTAssertEqual(3, components.minute ?? 0)
                fallthrough
            case .hour:
                XCTAssertEqual(8, components.hour ?? 0)
                fallthrough
            case .day:
                XCTAssertEqual(9, components.day ?? 0)
                XCTAssertEqual(12, components.month ?? 0)
                XCTAssertEqual(2004, components.year ?? 0)
            }
        }
    }
}
