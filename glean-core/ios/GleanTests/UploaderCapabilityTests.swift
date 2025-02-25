/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// REASON: This test is long because of setup boilerplate
class UploaderCapabilityTests: XCTestCase {
    var expectation: XCTestExpectation?

    private let testOhttpRequest = PingRequest(
        documentId: "12345",
        path: "/some/random/path/not/important",
        body: [UInt8]("{ \"ping\": \"test\" }".utf8),
        headers: [:],
        bodyHasInfoSections: true,
        pingName: "testPing",
        uploaderCapabilities: ["ohttp"]
    )

    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "UploaderCapabilityTests", clearStores: true)
    }

    override func tearDown() {
        // Reset expectations
        expectation = nil
        tearDownStubs()
    }

    /// Launch a new ping uploader on the background thread.
    ///
    /// This function doesn't block.
    private func getUploader(_ capabilities: [String] = []) -> HttpPingUploader {
        // Build a URLSession with no-caching suitable for uploading our pings
        let config: URLSessionConfiguration = .default
        config.requestCachePolicy = NSURLRequest.CachePolicy.reloadIgnoringLocalCacheData
        config.urlCache = nil
        let session = URLSession(configuration: config)
        return HttpPingUploader.init(configuration: Configuration(), session: session, capabilities: capabilities)
    }

    func testUploaderIncapable() {
        let httpPingUploader = self.getUploader()
        httpPingUploader.upload(request: testOhttpRequest) { result in
            XCTAssertEqual(
                .incapable(unused: 0),
                result,
                "upload should return .incapable when capabilities don't match"
            )
        }
    }

    func testUploaderCapable() {
        // We are explicitly setting the test mode to true here to force the uploader to not
        // run in the background, which can make this test take a long time.
        var testValue: UploadResult?
        stubServerReceive { _, json in
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")
        }
        expectation = expectation(description: "Completed upload")

        let ohttpCapableUploader = self.getUploader(["ohttp"])
        ohttpCapableUploader.upload(request: testOhttpRequest) { result in
            testValue = result
            self.expectation?.fulfill()
        }

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        // `UploadResult` is not `Equatable`, so instead of implementing that we just unpack it
        if case let .httpStatus(statusCode) = testValue {
            XCTAssertEqual(200, statusCode, "`upload()` returns the expected HTTP status code")
        } else {
            let value = String(describing: testValue)
            XCTAssertTrue(false, "`upload()` returns the expected HTTP status code. Was: \(value)")
        }
    }
}
