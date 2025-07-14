/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import XCTest

@testable import Glean

// REASON: This test is long because of setup boilerplate
class HttpPingUploaderTests: XCTestCase {
    var expectation: XCTestExpectation?
    private let testPath = "/some/random/path/not/important"
    private let testPing = "{ \"ping\": \"test\" }"
    private let testRequest = CapablePingUploadRequest(
        PingUploadRequest(
            request: PingRequest(
                documentId: "12345",
                path: "/some/random/path/not/important",
                body: [UInt8]("{ \"ping\": \"test\" }".utf8),
                headers: [:],
                bodyHasInfoSections: true,
                pingName: "testPing",
                uploaderCapabilities: []
            ),
            endpoint: Configuration.Constants.defaultTelemetryEndpoint
        )
    )

    override func setUp() {
        resetGleanDiscardingInitialPings(
            testCase: self,
            tag: "HttpPingUploaderTests",
            clearStores: true
        )
    }

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

        // Build a URLSession with no-caching suitable for uploading our pings

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(request: testRequest) { result in
            testValue = result
            self.expectation?.fulfill()
        }

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        // `UploadResult` is not `Equatable`, so instead of implementing that we just unpack it
        if case let .httpStatus(statusCode) = testValue {
            XCTAssertEqual(
                200,
                statusCode,
                "`upload()` returns the expected HTTP status code"
            )
        } else {
            let value = String(describing: testValue)
            XCTAssertTrue(
                false,
                "`upload()` returns the expected HTTP status code. Was: \(value)"
            )
        }
    }

    func testRequestParameters() {
        // Build a request.
        // We specify a single additional header here.
        // In usual code they are generated on the Rust side.
        let request = HttpPingUploader()
            .buildRequest(
                url: testPath,
                data: Data(testPing.utf8),
                headers: ["X-Test-Only": "Glean"]
            )

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

    func testUploaderReturnsIncapableWhenCapabilitesUnsupported() {
        var testValue: UploadResult?
        let testOhttpRequest = PingRequest(
            documentId: "12345",
            path: "/some/random/path/not/important",
            body: [UInt8]("{ \"ping\": \"test\" }".utf8),
            headers: [:],
            bodyHasInfoSections: true,
            pingName: "testPing",
            uploaderCapabilities: ["ohttp", "os2/warp", "sega-genesis"]
        )

        stubServerReceive { _, json in
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")
        }

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(
            request: CapablePingUploadRequest(
                PingUploadRequest(
                    request: testOhttpRequest,
                    endpoint: Configuration().serverEndpoint
                )
            )
        ) { result in
            testValue = result
            self.expectation?.fulfill()
        }

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertEqual(
            .incapable(unused: 0),
            testValue,
            "upload should return .incapable when capabilities don't match"
        )
    }

    func testUploaderReturnsRequestWhenCapbilitiesSupported() {
        var testValue: UploadResult?
        let testOhttpRequest = PingRequest(
            documentId: "12345",
            path: "/some/random/path/not/important",
            body: [UInt8]("{ \"ping\": \"test\" }".utf8),
            headers: [:],
            bodyHasInfoSections: true,
            pingName: "testPing",
            uploaderCapabilities: ["ohttp", "os2/warp", "sega-genesis"]
        )

        stubServerReceive { _, json in
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")
        }

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.capabilities = ["ohttp", "os2/warp", "sega-genesis"]
        httpPingUploader.upload(
            request: CapablePingUploadRequest(
                PingUploadRequest(
                    request: testOhttpRequest,
                    endpoint: Configuration().serverEndpoint
                )
            )
        ) { result in
            testValue = result
            self.expectation?.fulfill()
        }

        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertEqual(
            .httpStatus(code: 200),
            testValue,
            "upload should return .incapable when capabilities don't match"
        )
    }
}
