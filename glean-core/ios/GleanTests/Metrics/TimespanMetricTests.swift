/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class TimespanMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testTimespanSavesToStorage() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .millisecond
        )

        XCTAssertFalse(metric.testHasValue())

        // Record a timespan
        metric.start()
        metric.stop()

        // Check that the count was incremented and properly recorded.
        XCTAssert(metric.testHasValue())
        XCTAssert(try metric.testGetValue() >= 0)
    }

    func testTimespanMustNotRecordIfDisabled() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true,
            timeUnit: .millisecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        // Let's also call cancel() to make sure it's a no-op.
        metric.cancel()

        XCTAssertFalse(metric.testHasValue(), "Timespan must not be recorded if they are disabled")
    }

    func testTimespanMutCorrectlyCancel() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .millisecond
        )

        // Record a timespan.
        metric.start()
        metric.cancel()
        metric.stop()

        XCTAssertFalse(metric.testHasValue(), "Timespan must not be recorded if they are disabled")
    }

    func testTimespanGetValueThrowsExceptionIfNothingIsStored() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .millisecond
        )

        XCTAssertThrowsError(try metric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testTimespanSavesToSecondaryPings() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        XCTAssert(metric.testHasValue("store2"))
        XCTAssert(try metric.testGetValue("store2") >= 0)
    }

    func testTimespanSetRawNanos() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false,
            timeUnit: .second
        )
        let timespanNanos: UInt64 = 6 * 1_000_000_000

        metric.setRawNanos(timespanNanos)
        XCTAssertEqual(6, try metric.testGetValue())
    }

    func testTimespanSetRawNanosFollowedByOtherApi() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false,
            timeUnit: .second
        )
        let timespanNanos: UInt64 = 6 * 1_000_000_000

        metric.setRawNanos(timespanNanos)
        XCTAssertEqual(6, try metric.testGetValue())

        metric.start()
        metric.stop()
        XCTAssertEqual(6, try metric.testGetValue())
    }

    func testTimespanSetRawNanosDoesNotOverwrite() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false,
            timeUnit: .second
        )
        let timespanNanos: UInt64 = 6 * 1_000_000_000

        metric.start()
        metric.stop()
        let value = try! metric.testGetValue()

        metric.setRawNanos(timespanNanos)

        XCTAssertEqual(value, try metric.testGetValue())
    }

    func testTimespanSetRawNanosDoesNothingWhenTimerIsRunning() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false,
            timeUnit: .nanosecond
        )
        let timespanNanos: UInt64 = 6 * 1_000_000_000

        metric.start()
        metric.setRawNanos(timespanNanos)
        metric.stop()

        XCTAssertNotEqual(timespanNanos, try metric.testGetValue())
    }

    func testTimespanRecordsAnErrorIfStartedTwice() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false,
            timeUnit: .nanosecond
        )

        metric.start()
        metric.start()
        metric.stop()

        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidState))
    }

    func testMeasureFunctionCorrectlySavesValues() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .millisecond
        )

        XCTAssertFalse(metric.testHasValue())

        // Create a test function that returns a value so we can measure
        // it and check that it returns the correct value from the
        // measure function.
        func testFunc(value: Bool) -> Bool {
            return value
        }

        // Perform the measurement
        let testValue = metric.measure {
            testFunc(value: true)
        }

        // Ensure the return value of the test function is the one
        // returned by the `measure` function
        XCTAssertTrue(testValue)

        // Check that the count was incremented and properly recorded.
        XCTAssert(metric.testHasValue())
        XCTAssert(try metric.testGetValue() >= 0)
    }

    func testMeasureFunctionThrows() {
        let metric = TimespanMetricType(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false,
            timeUnit: .millisecond
        )

        XCTAssertFalse(metric.testHasValue())

        // Create a test function that throws an exception.
        func testFunc() throws {
            throw "invalid"
        }

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

        XCTAssertFalse(metric.testHasValue())
    }
}
