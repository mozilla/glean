/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class TimingDistributionTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testTiminingDistributionSavesToStorage() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            timeUnit: .nanosecond
        )

        // Accumulate a few values
        for _ in 1 ... 3 {
            let id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        XCTAssert(metric.testHasValue())
        let snapshot = try! metric.testGetValue()
        XCTAssertEqual(3, snapshot.count)
    }

    func testTimingDistributionMustNotRecordIfDisabled() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true,
            timeUnit: .nanosecond
        )

        XCTAssertFalse(metric.testHasValue())

        // Attempt to store the timespan using set
        let id = metric.start()
        metric.stopAndAccumulate(id)

        // Check that nothing was recorded.
        XCTAssertFalse(metric.testHasValue(), "TimingDistributions must not be recorded if they are disabled")
    }

    func testTimingDistributionGetValueThrowsExceptionIfNothingIsStored() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try metric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testTimingDistributionSavesToSecondaryPings() {
        // Define a timing distribution metric which will be stored in multiple stores
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1", "store2", "store3"],
            lifetime: .application,
            disabled: false
        )

        // Accumulate a few values
        for _ in 1 ... 3 {
            let id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        // We can only check the count, as we don't control the time.
        XCTAssert(metric.testHasValue("store2"))
        var snapshot = try! metric.testGetValue("store2")
        XCTAssertEqual(3, snapshot.count)

        XCTAssert(metric.testHasValue("store3"))
        snapshot = try! metric.testGetValue("store3")
        XCTAssertEqual(3, snapshot.count)
    }

    func testTimingDistributionMustNotRecordIfCanceled() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            timeUnit: .nanosecond
        )

        XCTAssertFalse(metric.testHasValue())

        // Attempt to store the timespan using set
        let id = metric.start()
        metric.cancel(id)

        // Check that nothing was recorded.
        XCTAssertFalse(metric.testHasValue(), "TimingDistributions must not be recorded if canceled")
    }

    func testStoppingNonexistentTimerRecordsAnError() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            timeUnit: .nanosecond
        )

        metric.stopAndAccumulate(0)

        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidState))
    }

    func testMeasureFunctionCorrectlyStoresValues() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false,
            timeUnit: .nanosecond
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
        XCTAssert(metric.testHasValue())
        let snapshot = try! metric.testGetValue()
        XCTAssertEqual(3, snapshot.count)
    }

    func testMeasureFunctionThrows() {
        let metric = TimingDistributionMetricType(
            category: "telemetry",
            name: "timing_distribution",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .nanosecond
        )

        XCTAssertFalse(metric.testHasValue())

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

        XCTAssertFalse(metric.testHasValue())
    }
}
