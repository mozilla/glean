/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class GleanTests: XCTestCase {
    var expectation: XCTestExpectation?

    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
        Glean.shared.enableTestingMode()
    }

    override func tearDown() {
        Glean.shared.setUploadEnabled(true)
        expectation = nil
    }

    func testInitializeGlean() {
        // Glean is already initialized by the `setUp()` function
        XCTAssert(Glean.shared.isInitialized(), "Glean should be initialized")
        XCTAssert(Glean.shared.getUploadEnabled(), "Upload is enabled by default")
    }

    func testExperimentRecording() {
        Glean.shared.setExperimentActive(
            experimentId: "experiment_test",
            branch: "branch_a",
            extra: nil
        )
        Glean.shared.setExperimentActive(
            experimentId: "experiment_api",
            branch: "branch_b",
            extra: ["test_key": "value"]
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_test"),
            "Experiment must be active"
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_api"),
            "Experiment must be active"
        )

        Glean.shared.setExperimentInactive(experimentId: "experiment_test")
        XCTAssertFalse(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_test"),
            "Experiment must not be active"
        )
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_api"),
            "Experiment must be active"
        )

        let experimentData = Glean.shared.testGetExperimentData(experimentId: "experiment_api")
        XCTAssertEqual(
            "branch_b",
            experimentData?.branch,
            "Experiment must have expected branch"
        )
        XCTAssertEqual(
            "value",
            experimentData?.extra["test_key"],
            "Experiment extra must have expected key/value"
        )
    }

    func testExperimentRecordingBeforeGleanInit() {
        // This test relies on Glean not being initialized and the task queueing to be on
        Glean.shared.testDestroyGleanHandle()
        Dispatchers.shared.setTaskQueuing(enabled: true)

        Glean.shared.setExperimentActive(
            experimentId: "experiment_set_preinit",
            branch: "branch_a",
            extra: nil
        )
        Glean.shared.setExperimentActive(
            experimentId: "experiment_preinit_disabled",
            branch: "branch_a",
            extra: nil
        )

        // Deactivate the second experiment
        Glean.shared.setExperimentInactive(experimentId: "experiment_preinit_disabled")

        // This will reset Glean and flush the queued tasks
        Glean.shared.resetGlean(clearStores: false)

        // Verify the tasks were executed
        XCTAssertTrue(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_set_preinit"),
            "Experiment must be active"
        )
        XCTAssertFalse(
            Glean.shared.testIsExperimentActive(experimentId: "experiment_preinit_disabled"),
            "Experiment must not be active"
        )
    }

    func testSendingOfStartupBaselinePing() {
        // Set the dirty flag
        glean_set_dirty_flag(true.toByte())

        // Set up the test stub based on the default telemetry endpoint
        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
            let json = try! JSONSerialization.jsonObject(with: body!, options: []) as? [String: Any]
            XCTAssert(json != nil)

            // Check for the "dirty_startup" flag
            let pingInfo = json?["ping_info"] as? [String: Any]
            XCTAssertEqual("dirty_startup", pingInfo?["reason"] as? String)

            // Ensure there is only the expected locale string metric
            let metrics = json?["metrics"] as? [String: Any]
            let strings = metrics?["string"] as? [String: Any]
            XCTAssertEqual(1, strings?.count, "Must contain only the expected metric")
            let locale = strings?["glean.baseline.locale"] as? String
            XCTAssertNotNil(locale, "Locale is not nil")

            // We should not have a duration for a ping with the "dirty_startup" flag
            XCTAssertNil(metrics?["timespan"])

            DispatchQueue.main.async {
                // let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }

            return OHHTTPStubsResponse(
                jsonObject: [],
                statusCode: 200,
                headers: ["Content-Type": "application/json"]
            )
        }

        // Set up the expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Baseline Ping Received")

        // Restart Glean and don't clear the stores and then await the expectation
        Glean.shared.resetGlean(clearStores: false)
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }
    }
}
