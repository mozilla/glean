/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import OHHTTPStubsSwift
import XCTest

private typealias GleanInternalMetrics = GleanMetrics.GleanInternalMetrics

// swiftlint:disable type_body_length
class GleanTests: XCTestCase {
    var expectation: XCTestExpectation?

    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanTests")
    }

    override func tearDown() {
        Glean.shared.testDestroyGleanHandle()
        expectation = nil
        tearDownStubs()
    }

    func testInitializeGlean() {
        // Glean is already initialized by the `setUp()` function
        XCTAssert(Glean.shared.isInitialized(), "Glean should be initialized")
    }

    func testExperimentRecording() {
        Glean.shared.setExperimentActive(
            "experiment_test",
            branch: "branch_a",
            extra: nil
        )
        Glean.shared.setExperimentActive(
            "experiment_api",
            branch: "branch_b",
            extra: ["test_key": "value"]
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive("experiment_test"),
            "Experiment must be active"
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive("experiment_api"),
            "Experiment must be active"
        )

        Glean.shared.setExperimentInactive("experiment_test")
        XCTAssertFalse(
            Glean.shared.testIsExperimentActive("experiment_test"),
            "Experiment must not be active"
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive("experiment_api"),
            "Experiment must be active"
        )

        let experimentData = Glean.shared.testGetExperimentData("experiment_api")
        XCTAssertEqual(
            "branch_b",
            experimentData?.branch,
            "Experiment must have expected branch"
        )
        XCTAssertEqual(
            "value",
            experimentData?.extra?["test_key"],
            "Experiment extra must have expected key/value"
        )
    }

    func testExperimentRecordingBeforeGleanInit() {
        // This test relies on Glean not being initialized and the task queueing to be on
        Glean.shared.testDestroyGleanHandle()

        Glean.shared.setExperimentActive(
            "experiment_set_preinit",
            branch: "branch_a",
            extra: nil
        )
        Glean.shared.setExperimentActive(
            "experiment_preinit_disabled",
            branch: "branch_a",
            extra: nil
        )

        // Deactivate the second experiment
        Glean.shared.setExperimentInactive("experiment_preinit_disabled")

        // This will reset Glean and flush the queued tasks
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanTests", clearStores: false)

        // Verify the tasks were executed
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive("experiment_set_preinit"),
            "Experiment must be active"
        )
        XCTAssertFalse(
            Glean.shared.testIsExperimentActive("experiment_preinit_disabled"),
            "Experiment must not be active"
        )
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
                // let's check that this is all we got (plus the `validation.first_run_hour`).
                XCTAssertEqual(metrics?.count, 2, "metrics has more keys than expected: \(JSONStringify(metrics!))")
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
                let datetimes = metrics!["datetime"] as! [String: Any]
                XCTAssertTrue(datetimes.keys.contains("glean.validation.first_run_hour"),
                              "Datetime should have first_run_hour: \(datetimes)")

                if metrics!.count > 1 {
                    // Since we are only expecting error metrics,
                    // let's check that this is all we got (plus the `validation.first_run_hour`).
                    XCTAssertEqual(metrics?.count, 2, "metrics has more keys than expected: \(JSONStringify(metrics!))")
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

    func testSendingDeletionPingIfDisabledOutsideOfRun() {
        stubServerReceive { pingType, _ in
            // Since we are starting Glean with upload disabled, the only ping we
            // should see is the deletion request ping
            XCTAssertEqual("deletion-request", pingType)

            DispatchQueue.main.async {
                // let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        // Set up the expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Deletion Request Received")

        // Now reset Glean with uploadEnabled = false and not clearing the stores to
        // trigger the deletion request ping. Since `uploadEnabled` is `false`, only
        // the deletion-request ping should be generated.
        Glean.shared.resetGlean(clearStores: false, uploadEnabled: false)
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

    func testNotSendingDeletionRequestIfUnchangedOutsideOfRun() {
        XCTAssert(Glean.shared.isInitialized(), "Glean should be initialized")

        // Set up the test stub based on the default telemetry endpoint
        stubServerReceive { _, _ in
            XCTFail("Should not have recieved any ping")
        }

        // Set up the expectation that will NOT be fulfilled by the stub above.  If it is
        // then it will trigger an assertion due to the `assertForOverFulfill` property.
        expectation = expectation(description: "Deletion Request Received")

        // So we can wait for expectations below, we will go ahead and fulfill the
        // expectation.  We want to assert if the ping is triggered and over fulfills it
        // from the stub above.
        expectation?.fulfill()

        // Reset Glean with uploadEnabled = false
        Glean.shared.resetGlean(clearStores: true, uploadEnabled: false)

        // Now reset Glean with uploadEnabled = false again without clearing the stores to
        // make sure we don't trigger the deletion request ping.  If it does, then we will
        // have overfulfilled the expectation which will trigger a test assertion.
        Glean.shared.resetGlean(clearStores: false, uploadEnabled: false)
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

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

    func testGleanIsNotInitializedFromOtherProcesses() {
        // Check to see if Glean is initialized
        XCTAssert(Glean.shared.isInitialized())

        // Set the control variable to false to simulate that we are not running
        // in the main process
        Glean.shared.isMainProcess = false

        expectation = expectation(description: "GleanTests: Ping Received")
        // We are using OHHTTPStubs combined with an XCTestExpectation in order to capture
        // outgoing network requests and prevent actual requests being made from tests.
        stubServerReceive { _, _ in
            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        // Invert the expectation so that it will assert if it gets fulfilled. Since Glean
        // is simulating being initialized on another process, we should not get any pings
        // since init should fail.
        expectation?.isInverted = true
        // Restart Glean
        Glean.shared.resetGlean(clearStores: false)
        waitForExpectations(timeout: 2.0) { error in
            XCTAssertNil(error, "Received a ping upload when we shouldn't have: \(error!)")
        }

        // Check to see if Glean is initialized
        XCTAssertFalse(Glean.shared.isInitialized())

        // Reset variable so as to not interfere with other tests.
        Glean.shared.isMainProcess = true
    }

    func testSettingDebugViewTagBeforeInitialization() {
        // This test relies on Glean not being initialized
        Glean.shared.testDestroyGleanHandle()

        XCTAssert(Glean.shared.setDebugViewTag("valid-tag"))

        // Restart glean
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanTest", clearStores: false)

        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let request = data as NSURLRequest
            XCTAssertEqual(request.value(forHTTPHeaderField: "X-Debug-ID"), "valid-tag")

            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }

            return HTTPStubsResponse(
                jsonObject: [],
                statusCode: 200,
                headers: ["Content-Type": "application/json"]
            )
        }

        expectation = expectation(description: "Completed upload")

        // Resetting Glean doesn't trigger pings in tests so we must call the method
        // directly to invoke a ping to be created
        Glean.shared.submitPingByName("baseline")

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

    func testSettingSourceTagsBeforeInitialization() {
        // This test relies on Glean not being initialized
        Glean.shared.testDestroyGleanHandle()

        XCTAssert(Glean.shared.setSourceTags(["valid-tag", "tag-valid"]))

        // Restart glean, disposing of any pings from startup that might interfere with the test
        resetGleanDiscardingInitialPings(testCase: self, tag: "GleanTest", clearStores: false)

        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let request = data as NSURLRequest
            XCTAssertEqual(request.value(forHTTPHeaderField: "X-Source-Tags"), "valid-tag,tag-valid")

            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }

            return HTTPStubsResponse(
                jsonObject: [],
                statusCode: 200,
                headers: ["Content-Type": "application/json"]
            )
        }

        expectation = expectation(description: "Completed upload")

        // We only want to submit the baseline ping, so we sumbit it by name
        Glean.shared.submitPingByName("baseline")

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

    func testFlippingUploadEnabledRespectsOrderOfEvents() {
        // This test relies on Glean not being initialized
        Glean.shared.testDestroyGleanHandle()

        // We expect only a single ping later
        stubServerReceive { pingType, _ in
            if pingType == "baseline" {
                // Ignore initial "active" baseline ping
                return
            }

            XCTAssertEqual("deletion-request", pingType)

            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }

        let customPing = Ping<NoReasonCodes>(
            name: "custom",
            includeClientId: true,
            sendIfEmpty: false,
            reasonCodes: []
        )

        let counter = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter_metric",
            sendInPings: ["custom"],
            lifetime: .application,
            disabled: false
        ))

        expectation = expectation(description: "Completed upload")

        // Set the last time the "metrics" ping was sent to now. This is required for us to not
        // send a metrics pings the first time we initialize Glean and to keep it from interfering
        // with these tests.
        let now = Date()
        MetricsPingScheduler(true).updateSentDate(now)
        // Restart glean
        Glean.shared.resetGlean(clearStores: false)

        // Glean might still be initializing. Disable upload.
        Glean.shared.setUploadEnabled(false)

        // Set data and try to submit a custom ping.
        counter.add(1)
        customPing.submit()

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }

    func testForegroundCounter() {
        // Glean is started by the test framework.
        // That already triggers the first foreground event.

        // Put it in the background
        Glean.shared.handleBackgroundEvent()

        // Bring it back
        Glean.shared.handleForegroundEvent()

        let foregroundCounter = GleanMetrics.GleanValidation.foregroundCount
        XCTAssert(foregroundCounter.testHasValue())
        XCTAssertEqual(2, foregroundCounter.testGetValue())
    }

    func testPassingInExplicitBuildInfo() {
        Glean.shared.testDestroyGleanHandle()

        Glean.shared.initialize(uploadEnabled: true, buildInfo: stubBuildInfo("2020-11-06T11:30:50+0000"))
        let expected = Date.fromISO8601String(dateString: "2020-11-06T11:30:50+00:00", precision: .second)
        XCTAssertEqual(
            expected,
            GleanInternalMetrics.buildDate.testGetValue()
        )
    }
}
