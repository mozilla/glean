/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class DeletionRequestPingTests: XCTestCase {
    var expectation: XCTestExpectation?
    var lastPingJson: [String: Any]?

    private func setupHttpResponseStub(_ expectedPingType: String) {
        stubServerReceive { pingType, json in
            XCTAssertEqual(pingType, expectedPingType, "Wrong ping type received")

            XCTAssert(json != nil)
            self.lastPingJson = json

            // Fulfill test's expectation once we parsed the incoming data.
            DispatchQueue.main.async {
                // Let the response get processed before we mark the expectation fulfilled
                self.expectation?.fulfill()
            }
        }
    }

    override func tearDown() {
        lastPingJson = nil
        expectation = nil
        OHHTTPStubs.removeAllStubs()
    }

    func testDeletionRequestPingsAreSentWhenUploadDisabled() {
        Glean.shared.resetGlean(clearStores: true)

        setupHttpResponseStub("deletion-request")
        expectation = expectation(description: "Completed upload")

        Glean.shared.setUploadEnabled(false)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let clientInfo = lastPingJson!["client_info"] as! [String: Any]
        let clientId = clientInfo["client_id"] as! String
        XCTAssertNotEqual(clientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")
    }

    func testPendingDeletionRequestPingsAreSentOnStartup() {
        let glean = Glean.shared
        glean.testDestroyGleanHandle()
        glean.enableTestingMode()

        // Create directory for pending deletion-request pings
        let pendingDeletionRequestDir = getGleanDirectory().appendingPathComponent("deletion_request")
        try! FileManager.default.createDirectory(
            atPath: pendingDeletionRequestDir.path,
            withIntermediateDirectories: true,
            attributes: nil
        )

        // Write a deletion-request ping file
        let pingId = "b4e4ede0-8716-4691-a3fa-493c56c5be2d"
        let submitPath = "/submit/org-mozilla-samples-gleancore/deletion-request/1/\(pingId)"
        // swiftlint:disable line_length
        // REASON: This is inline JSON
        let json = "{\"ping_info\": {\"ping_type\": \"deletion-request\"}, \"client_info\": {\"client_id\": \"test-only\"}}"
        // swiftlint:enable line_length
        let content = "\(submitPath)\n\(json)"
        let pingFile = pendingDeletionRequestDir.appendingPathComponent(pingId)
        FileManager.default.createFile(
            atPath: pingFile.relativePath,
            contents: content.data(using: .utf8),
            attributes: nil
        )

        setupHttpResponseStub("deletion-request")
        expectation = expectation(description: "Completed upload")

        // Init Glean.
        glean.initialize(uploadEnabled: false)

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        let clientInfo = lastPingJson!["client_info"] as! [String: Any]
        let clientId = clientInfo["client_id"] as! String
        XCTAssertEqual(clientId, "test-only")
    }
}
