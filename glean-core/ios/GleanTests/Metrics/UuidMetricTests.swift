/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class UuidMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testUuidSavesToStorage() {
        let uuidMetric = UuidMetricType(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        // Check that there is no UUID recorded
        XCTAssertFalse(uuidMetric.testHasValue())

        // Record two UUID's of the same type, with a little delay
        let uuid = uuidMetric.generateAndSet()

        // Check that the data was properly recorded
        XCTAssertTrue(uuidMetric.testHasValue())
        XCTAssertEqual(uuid, try uuidMetric.testGetValue())

        let uuid2 = UUID(uuidString: "ce2adeb8-843a-4232-87a5-a099ed1e7bb3")!
        uuidMetric.set(uuid2)

        // Check that the data was properly recorded
        XCTAssertTrue(uuidMetric.testHasValue())
        XCTAssertEqual(uuid2, try uuidMetric.testGetValue())
    }

    func testUuidMustNotRecordIfDisabled() {
        let uuidMetric = UuidMetricType(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(uuidMetric.testHasValue())

        _ = uuidMetric.generateAndSet()

        XCTAssertFalse(uuidMetric.testHasValue(), "UUIDs must not be recorded if they are disabled")
    }

    func testUuidGetValueThrowsExceptionIfNothingIsStored() {
        let uuidMetric = UuidMetricType(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try uuidMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testUuidSavesToSecondaryPings() {
        let uuidMetric = UuidMetricType(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        // Record two UUID's of the same type, with a little delay
        let uuid = uuidMetric.generateAndSet()

        // Check that the data was properly recorded
        XCTAssertTrue(uuidMetric.testHasValue("store2"))
        XCTAssertEqual(uuid, try uuidMetric.testGetValue("store2"))

        let uuid2 = UUID(uuidString: "ce2adeb8-843a-4232-87a5-a099ed1e7bb3")!
        uuidMetric.set(uuid2)

        // Check that the data was properly recorded
        XCTAssertTrue(uuidMetric.testHasValue("store2"))
        XCTAssertEqual(uuid2, try uuidMetric.testGetValue("store2"))
    }
}
