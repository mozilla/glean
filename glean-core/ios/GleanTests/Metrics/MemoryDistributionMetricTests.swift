/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class MemoryDistributionTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "MemoryDistributionTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testTiminingDistributionSavesToStorage() {
        let metric = MemoryDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ), .kilobyte
        )

        // Accumulate a few values
        for i in Int64(1) ... 3 {
            metric.accumulate(i)
        }

        let kb = Int64(1024)

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        let snapshot = metric.testGetValue()!
        let sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)

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
        let metric = MemoryDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ), .gigabyte
        )

        metric.accumulate(2048)

        let snapshot = metric.testGetValue()!

        // Check the sum
        XCTAssertEqual(1 << 40, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[(1 << 40) - 1])
        // Check that an error was recorded
        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidValue))
    }

    func testMemoryDistributionMustNotRecordIfDisabled() {
        let metric = MemoryDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true
            ), .kilobyte
        )

        metric.accumulate(1)
        XCTAssertNil(metric.testGetValue())
    }

    func testMemoryDistributionGetValueReturnsNilIfNothingIsStored() {
        let metric = MemoryDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
            ), .kilobyte
        )

        XCTAssertNil(metric.testGetValue())
    }

    func testMemoryDistributionSavesToSecondaryPings() {
        // Define a memory distribution metric which will be stored in multiple stores
        let metric = MemoryDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "memory_distribution",
            sendInPings: ["store1", "store2", "store3"],
            lifetime: .application,
            disabled: false
            ), .kilobyte
        )

        // Accumulate a few values
        for i in Int64(1) ... 3 {
            metric.accumulate(i)
        }

        // Check that data was properly recorded in the second ping.
        var snapshot = metric.testGetValue("store2")!

        // Check the sum
        XCTAssertEqual(6144, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        XCTAssertEqual(1, snapshot.values[1023])
        // Check that the 2L fell into the second value bucket
        XCTAssertEqual(1, snapshot.values[2047])
        // Check that the 3L fell into the third value bucket
        XCTAssertEqual(1, snapshot.values[3024])

        // Check that data was properly recorded in the second ping.
        snapshot = metric.testGetValue("store3")!

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
