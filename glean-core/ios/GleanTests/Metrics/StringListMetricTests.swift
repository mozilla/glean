/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class StringListMetricTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testStringSavesToStorageByFirstAddingThenSetting() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        // Record by adding values
        stringListMetric.add("value1")
        stringListMetric.add("value2")
        stringListMetric.add("value3")

        XCTAssert(stringListMetric.testHasValue())
        let snapshot = try! stringListMetric.testGetValue()
        XCTAssertEqual(3, snapshot.count)
        XCTAssertEqual("value1", snapshot[0])
        XCTAssertEqual("value2", snapshot[1])
        XCTAssertEqual("value3", snapshot[2])

        // Use set() to see that the first list is replaced by the new list
        stringListMetric.set(["other1", "other2", "other3"])
        // Check that data was properly recorded.
        let snapshot2 = try! stringListMetric.testGetValue()
        XCTAssertEqual(3, snapshot2.count)
        XCTAssertEqual("other1", snapshot2[0])
        XCTAssertEqual("other2", snapshot2[1])
        XCTAssertEqual("other3", snapshot2[2])
    }

    func testStringSavesToStorageByFirstSettingThenAdding() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        // Record by setting the list
        stringListMetric.set(["value1", "value2", "value3"])

        XCTAssert(stringListMetric.testHasValue())
        let snapshot = try! stringListMetric.testGetValue()
        XCTAssertEqual(3, snapshot.count)
        XCTAssertEqual("value1", snapshot[0])
        XCTAssertEqual("value2", snapshot[1])
        XCTAssertEqual("value3", snapshot[2])

        // Use add() to append to the list
        stringListMetric.add("added1")
        // Check that data was properly recorded.
        let snapshot2 = try! stringListMetric.testGetValue()
        XCTAssertEqual(4, snapshot2.count)
        XCTAssertEqual("value1", snapshot2[0])
        XCTAssertEqual("value2", snapshot2[1])
        XCTAssertEqual("value3", snapshot2[2])
        XCTAssertEqual("added1", snapshot2[3])
    }

    func testStringMustNotRecordIfDisabled() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(stringListMetric.testHasValue())

        stringListMetric.set(["value1", "value2", "value3"])
        XCTAssertFalse(stringListMetric.testHasValue(), "Strings must not be recorded if they are disabled")

        stringListMetric.add("value4")
        XCTAssertFalse(stringListMetric.testHasValue(), "Strings must not be recorded if they are disabled")
    }

    func testStringGetValueThrowsExceptionIfNothingIsStored() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try stringListMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testStringSavesToSecondaryPings() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        stringListMetric.add("value1")
        stringListMetric.add("value2")
        stringListMetric.add("value3")

        let snapshot = try! stringListMetric.testGetValue("store2")
        XCTAssertEqual(3, snapshot.count)
        XCTAssertEqual("value1", snapshot[0])
        XCTAssertEqual("value2", snapshot[1])
        XCTAssertEqual("value3", snapshot[2])

        // Use set() to see that the first list is replaced by the new list
        stringListMetric.set(["other1", "other2", "other3"])
        // Check that data was properly recorded.
        let snapshot2 = try! stringListMetric.testGetValue("store2")
        XCTAssertEqual(3, snapshot2.count)
        XCTAssertEqual("other1", snapshot2[0])
        XCTAssertEqual("other2", snapshot2[1])
        XCTAssertEqual("other3", snapshot2[2])
    }

    func testLongStringListsAreTruncated() {
        let stringListMetric = StringListMetricType(
            category: "telemetry",
            name: "string_list_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        for n in 0 ... 20 {
            stringListMetric.add(String(format: "value%02d", n))
        }

        XCTAssertEqual(20, try stringListMetric.testGetValue().count)
        XCTAssertEqual(1, stringListMetric.testGetNumRecordedErrors(.invalidValue))
    }
}
