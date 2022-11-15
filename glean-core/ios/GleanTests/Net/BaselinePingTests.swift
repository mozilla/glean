/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import OHHTTPStubsSwift
import XCTest

final class BaselinePingTests: XCTestCase {
    var expectation: XCTestExpectation?

    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanTests")
    }

    override func tearDown() {
        Glean.shared.testDestroyGleanHandle()
        expectation = nil
        tearDownStubs()
    }

    func testSendingOfForegroundBaselinePing() {
        stubServerReceive { _, json in
            // Check for the "dirty_startup" flag
            let pingInfo = json?["ping_info"] as? [String: Any]
            XCTAssertEqual("active", pingInfo?["reason"] as? String)

            // We may get error metrics in foreground pings,
            // so 'metrics' may exist.
            let metrics = json?["metrics"] as? [String: Any]
            if metrics != nil {
                // Since we are only expecting error metrics,
                // let's check that this is all we got.
                XCTAssertEqual(metrics?.count, 1, "metrics has more keys than expected: \(JSONStringify(metrics!))")
                let labeledCounters = metrics?["labeled_counter"] as? [String: Any]
                labeledCounters!.forEach { key, _ in
                    XCTAssertTrue(
                        key.starts(with: "glean.error") || key.starts(with: "glean.validation"),
                        "Should only see glean.* counters, saw \(key)"
                    )
                }
            }

            DispatchQueue.main.async {
                // let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        // Set up the expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Baseline Ping Received")

        // Set the last time the "metrics" ping was sent to now. This is required for us to not
        // send a metrics pings the first time we initialize Glean and to keep it from interfering
        // with these tests.
        let now = Date()
        Glean.shared.metricsPingScheduler!.updateSentDate(now)

        // Resetting Glean doesn't trigger lifecycle events in tests so we must call the method
        // invoked by the lifecycle observer directly.
        Glean.shared.handleForegroundEvent()
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

    /*
     FIXME: This test causes crashes in subsequent tests,
     probably because of some race condition triggered by restarting Glean.
    func testSendingOfBaselinePingWithDirtyFlag() {
        // Set the dirty flag
        gleanSetDirtyFlag(true)

        // Set up the test stub based on the default telemetry endpoint
        stubServerReceive { pingType, json in
            XCTAssertEqual("baseline", pingType)
            XCTAssert(json != nil)

            // Check for the "dirty_startup" flag
            let pingInfo = json!["ping_info"] as! [String: Any]
            let reason = pingInfo["reason"] as! String
            if reason == "active" {
                // Skip initial "active" ping.
                // Glean is initialized ahead of this test and thus we might get one.
                return
            }

            XCTAssertEqual("dirty_startup", reason, "Expected a dirty_startup, got \(reason)")

            // 'metrics' will exist and include exactly one valid metric.
            // No errors should be reported.
            let metrics = json!["metrics"] as? [String: Any]
            if metrics != nil {
                if metrics!.count > 1 {
                    XCTAssertEqual(metrics?.count, 1, "metrics has more keys than expected: \(JSONStringify(metrics!))")
                    let labeledCounters = metrics?["labeled_counter"] as? [String: Any]
                    labeledCounters!.forEach { key, _ in
                        XCTAssertTrue(
                            key.starts(with: "glean.error") || key.starts(with: "glean.validation"),
                            "Should only see glean.* counters, saw \(key)"
                        )
                    }
                }
            }

            DispatchQueue.main.async {
                // let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        // Set up the expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Baseline Ping Received")

        // Set the last time the "metrics" ping was sent to now. This is required for us to not
        // send a metrics pings the first time we initialize Glean and to keep it from interfering
        // with these tests.
        let now = Date()
        Glean.shared.metricsPingScheduler.updateSentDate(now)
        // Restart Glean and don't clear the stores and then await the expectation
        Glean.shared.resetGlean(clearStores: false)
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }
    */

    // swiftlint:disable force_cast
    func testSendingOfStartupBaselinePingWithAppLifetimeMetric() {
        // Set the dirty flag.
        gleanSetDirtyFlag(true)

        let stringMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "app_lifetime",
            sendInPings: ["baseline"],
            lifetime: .application,
            disabled: false
        ))
        stringMetric.set("HELLOOOOO!")

        // Set up the test stub based on the default telemetry endpoint
        stubServerReceive { pingType, json in
            XCTAssertEqual("baseline", pingType)
            XCTAssert(json != nil)

            // Check for the "dirty_startup" flag
            let pingInfo = json!["ping_info"] as! [String: Any]
            let reason = pingInfo["reason"] as! String
            if reason == "active" {
                // Skip initial "active" ping.
                // Glean is initialized ahead of this test and thus we might get one.
                return
            }

            XCTAssertEqual("dirty_startup", reason)

            // Ensure there is only the expected locale string metric
            let metrics = json?["metrics"] as? [String: Any]
            let strings = metrics?["string"] as? [String: Any]
            let metric = strings?["telemetry.app_lifetime"] as? String
            XCTAssertEqual("HELLOOOOO!", metric)

            DispatchQueue.main.async {
                // let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        expectation = expectation(description: "baseline ping received")

        // Restart glean and don't clear the stores.
        // This should trigger a baseline ping with a "dirty_startup" reason.
        Glean.shared.resetGlean(clearStores: false)
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }
    // swiftlint:enable force_cast
}
