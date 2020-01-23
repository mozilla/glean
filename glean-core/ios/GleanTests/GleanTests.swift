/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class GleanTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
        Glean.shared.enableTestingMode()
    }

    override func tearDown() {
        Glean.shared.setUploadEnabled(true)
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
}
