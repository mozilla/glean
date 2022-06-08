/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class UuidMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "UuidMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testUuidSavesToStorage() {
        let uuidMetric = UuidMetricType(CommonMetricData(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        // Check that there is no UUID recorded
        XCTAssertNil(uuidMetric.testGetValue())

        // Record two UUID's of the same type, with a little delay
        let uuid = uuidMetric.generateAndSet()

        // Check that the data was properly recorded
        XCTAssertEqual(uuid, uuidMetric.testGetValue())

        let uuid2 = UUID(uuidString: "ce2adeb8-843a-4232-87a5-a099ed1e7bb3")!
        uuidMetric.set(uuid2)

        // Check that the data was properly recorded
        XCTAssertEqual(uuid2, uuidMetric.testGetValue())
    }

    func testUuidMustNotRecordIfDisabled() {
        let uuidMetric = UuidMetricType(CommonMetricData(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(uuidMetric.testGetValue())

        _ = uuidMetric.generateAndSet()

        XCTAssertNil(uuidMetric.testGetValue(), "UUIDs must not be recorded if they are disabled")
    }

    func testUuidGetValueReturnsNilIfNothingIsStored() {
        let uuidMetric = UuidMetricType(CommonMetricData(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(uuidMetric.testGetValue())
    }

    func testUuidSavesToSecondaryPings() {
        let uuidMetric = UuidMetricType(CommonMetricData(
            category: "telemetry",
            name: "uuid_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        // Record two UUID's of the same type, with a little delay
        let uuid = uuidMetric.generateAndSet()

        // Check that the data was properly recorded
        XCTAssertEqual(uuid, uuidMetric.testGetValue("store2"))

        let uuid2 = UUID(uuidString: "ce2adeb8-843a-4232-87a5-a099ed1e7bb3")!
        uuidMetric.set(uuid2)

        // Check that the data was properly recorded
        XCTAssertEqual(uuid2, uuidMetric.testGetValue("store2"))
    }
}
