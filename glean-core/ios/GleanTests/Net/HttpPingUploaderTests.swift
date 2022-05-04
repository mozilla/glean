/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// REASON: This test is long because of setup boilerplate
class HttpPingUploaderTests: XCTestCase {
    var expectation: XCTestExpectation?
    private let testPath = "/some/random/path/not/important"
    private let testPing = "{ \"ping\": \"test\" }"

    override func tearDown() {
        // Reset expectations
        expectation = nil
        tearDownStubs()
    }

    func testHTTPStatusCode() {
        // We are explicitly setting the test mode to true here to force the uploader to not
        // run in the background, which can make this test take a long time.
        var testValue: UploadResult?
        stubServerReceive { _, json in
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")
        }

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader(configuration: Configuration(), testingMode: true)
        httpPingUploader.upload(path: testPath, data: Data(testPing.utf8), headers: [:]) { result in
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

    func testRequestParameters() {
        // Build a request.
        // We specify a single additional header here.
        // In usual code they are generated on the Rust side.
        let request = HttpPingUploader(configuration: Configuration())
            .buildRequest(path: testPath, data: Data(testPing.utf8), headers: ["X-Test-Only": "Glean"])

        XCTAssertEqual(
            request?.url?.path,
            testPath,
            "Request path set correctly"
        )
        XCTAssertEqual(
            request?.httpMethod,
            "POST",
            "Request method set correctly"
        )
        XCTAssertEqual(
            request?.httpBody,
            Data(testPing.utf8),
            "Request body set correctly"
        )
        XCTAssertEqual(
            request?.httpShouldHandleCookies,
            false,
            "Request cookie policy set correctly"
        )
        XCTAssertEqual(
            request?.value(forHTTPHeaderField: "X-Test-Only"),
            "Glean",
            "Request header set correctly"
        )
    }
}
