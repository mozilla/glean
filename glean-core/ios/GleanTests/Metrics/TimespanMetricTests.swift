/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class TimespanMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "TimespanMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testTimespanSavesToStorage() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ),
            .millisecond
        )

        XCTAssertNil(metric.testGetValue())

        // Record a timespan
        metric.start()
        metric.stop()

        // Check that the count was incremented and properly recorded.
        XCTAssert(metric.testGetValue()! >= 0)
    }

    func testTimespanMustNotRecordIfDisabled() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ), .millisecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        // Let's also call cancel() to make sure it's a no-op.
        metric.cancel()

        XCTAssertNil(metric.testGetValue(), "Timespan must not be recorded if they are disabled")
    }

    func testTimespanMutCorrectlyCancel() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), .millisecond
        )

        // Record a timespan.
        metric.start()
        metric.cancel()
        metric.stop()

        XCTAssertNil(metric.testGetValue(), "Timespan must not be recorded if they are disabled")
    }

    func testTimespanGetValueReturnsNilIfNothingIsStored() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), .millisecond
        )

        XCTAssertNil(metric.testGetValue())
    }

    func testTimespanSavesToSecondaryPings() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .second)

        // Record a timespan.
        metric.start()
        metric.stop()

        XCTAssert(metric.testGetValue("store2")! >= 0)
    }

    func testTimespanSetRawNanos() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .second
        )
        let timespanNanos: Int64 = 6 * 1_000_000_000

        metric.setRawNanos(timespanNanos)
        XCTAssertEqual(6, metric.testGetValue())
    }

    func testTimespanSetRawNanosFollowedByOtherApi() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .second)
        let timespanNanos: Int64 = 6 * 1_000_000_000

        metric.setRawNanos(timespanNanos)
        XCTAssertEqual(6, metric.testGetValue())

        metric.start()
        metric.stop()
        XCTAssertEqual(6, metric.testGetValue())
    }

    func testTimespanSetRawNanosDoesNotOverwrite() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .second)
        let timespanNanos: Int64 = 6 * 1_000_000_000

        metric.start()
        metric.stop()
        let value = metric.testGetValue()!

        metric.setRawNanos(timespanNanos)

        XCTAssertEqual(value, metric.testGetValue())
    }

    func testTimespanSetRawNanosDoesNothingWhenTimerIsRunning() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .nanosecond)
        let timespanNanos: Int64 = 6 * 1_000_000_000

        metric.start()
        metric.setRawNanos(timespanNanos)
        metric.stop()

        XCTAssertNotEqual(timespanNanos, metric.testGetValue())
    }

    func testTimespanRecordsAnErrorIfStartedTwice() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ), .nanosecond)

        metric.start()
        metric.start()
        metric.stop()

        XCTAssertEqual(1, metric.testGetNumRecordedErrors(.invalidState))
    }

    func testMeasureFunctionCorrectlySavesValues() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), .millisecond)

        XCTAssertNil(metric.testGetValue())

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
        XCTAssert(metric.testGetValue()! >= 0)
    }

    func testMeasureFunctionThrows() {
        let metric = TimespanMetricType(CommonMetricData(
            category: "telemetry",
            name: "timespan_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), .millisecond)

        XCTAssertNil(metric.testGetValue())

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

        XCTAssertNil(metric.testGetValue())
    }
}
