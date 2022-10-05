/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class TimingDistributionTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "TimingDistributionMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testTiminingDistributionSavesToStorage() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
        ), .nanosecond)

        XCTAssertNil(metric.testGetValue())

        // Accumulate a few values
        for _ in 1 ... 3 {
            let id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        let snapshot = metric.testGetValue()!
        let sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)
        XCTAssertEqual(3, snapshot.count)
    }

    func testTimingDistributionMustNotRecordIfDisabled() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true
        ), .nanosecond
        )

        XCTAssertNil(metric.testGetValue())

        // Attempt to store the timespan using set
        let id = metric.start()
        metric.stopAndAccumulate(id)

        // Check that nothing was recorded.
        XCTAssertNil(metric.testGetValue(), "TimingDistributions must not be recorded if they are disabled")
    }

    func testTimingDistributionGetValueReturnsNilIfNothingIsStored() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), .second)

        XCTAssertNil(metric.testGetValue())
    }

    func testTimingDistributionSavesToSecondaryPings() {
        // Define a timing distribution metric which will be stored in multiple stores
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1", "store2", "store3"],
            lifetime: .application,
            disabled: false
        ), .second)

        // Accumulate a few values
        for _ in 1 ... 3 {
            let id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        var snapshot = metric.testGetValue("store2")!
        var sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)
        XCTAssertEqual(3, snapshot.count)

        snapshot = metric.testGetValue("store3")!
        sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)
        XCTAssertEqual(3, snapshot.count)
    }

    func testTimingDistributionMustNotRecordIfCanceled() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ), .nanosecond
        )

        XCTAssertNil(metric.testGetValue())

        // Attempt to store the timespan using set
        let id = metric.start()
        metric.cancel(id)

        // Check that nothing was recorded.
        XCTAssertNil(metric.testGetValue(), "TimingDistributions must not be recorded if canceled")
    }

    func testStoppingNonexistentTimerRecordsAnError() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ), .nanosecond
        )

        metric.stopAndAccumulate(TimerId(id: 0))

        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidState))
    }

    func testMeasureFunctionCorrectlyStoresValues() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
            ), .nanosecond
        )

        func testFunc(value: Bool) -> Bool {
            return value
        }

        // Accumulate a few values
        for _ in 1 ... 3 {
            let testValue = metric.measure {
                testFunc(value: true)
            }

            // Make sure that the `measure` function returns the value from
            // the measured function
            XCTAssertTrue(testValue)
        }

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        let snapshot = metric.testGetValue()!
        let sum = snapshot.values.values.reduce(0, +)
        XCTAssertEqual(3, sum)
        XCTAssertEqual(3, snapshot.count)
    }

    func testMeasureFunctionThrows() {
        let metric = TimingDistributionMetricType(CommonMetricData(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
            ), .nanosecond
        )

        XCTAssertNil(metric.testGetValue())

        // Create a test function that throws an exception.
        func testFunc() throws {
            throw "invalid"
        }

        // Measure a few times. Nothing should be recorded.
        for _ in 1 ... 3 {
            // Perform the measurement
            do {
                _ = try metric.measure {
                    try testFunc()
                }

                // The function throws, so this is unreachable
                XCTAssert(false)
            } catch {
                // intentionally left empty
            }
        }

        XCTAssertNil(metric.testGetValue())
    }
}
