/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class MemoryDistributionTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testTiminingDistributionSavesToStorage() {
        let metric = MemoryDistributionMetricType(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            memoryUnit: .kilobyte
        )

        // Accumulate a few values
        for i in UInt64(1) ... 3 {
            metric.accumulate(i)
        }

        let kb = UInt64(1024)

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        XCTAssert(metric.testHasValue())
        let snapshot = try! metric.testGetValue()
        XCTAssertEqual(3, snapshot.count)

        // Check the sum
        XCTAssertEqual(1 * kb + 2 * kb + 3 * kb, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1023])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2047])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3024])
    }

    func testMemoryDistributionValuesAreTruncatedTo1Tb() {
        let metric = MemoryDistributionMetricType(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            memoryUnit: .gigabyte
        )

        metric.accumulate(2048)

        let snapshot = try! metric.testGetValue()

        // Check the sum
        XCTAssertEqual(1 << 40, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[(1 << 40) - 1])
        // Check that an error was recorded
        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidValue))
    }

    func testMemoryDistributionMustNotRecordIfDisabled() {
        let metric = MemoryDistributionMetricType(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true,
            memoryUnit: .kilobyte
        )

        metric.accumulate(1)
        XCTAssertFalse(metric.testHasValue())
    }

    func testMemoryDistributionGetValueThrowsExceptionIfNothingIsStored() {
        let metric = MemoryDistributionMetricType(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            memoryUnit: .kilobyte
        )

        XCTAssertThrowsError(try metric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testMemoryDistributionSavesToSecondaryPings() {
        // Define a memory distribution metric which will be stored in multiple stores
        let metric = MemoryDistributionMetricType(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1", "store2", "store3"],
            lifetime: .application,
            disabled: false,
            memoryUnit: .kilobyte
        )

        // Accumulate a few values
        for i in UInt64(1) ... 3 {
            metric.accumulate(i)
        }

        // Check that data was properly recorded in the second ping.
        XCTAssert(metric.testHasValue("store2"))
        var snapshot = try! metric.testGetValue("store2")

        // Check the sum
        XCTAssertEqual(6144, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1023])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2047])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3024])

        // Check that data was properly recorded in the second ping.
        XCTAssert(metric.testHasValue("store3"))
        snapshot = try! metric.testGetValue("store3")

        // Check the sum
        XCTAssertEqual(6144, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1023])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2047])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3024])
    }
}
