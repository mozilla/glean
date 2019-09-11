/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

class HttpPingUploaderTests: XCTestCase {
    var expectation: XCTestExpectation?
    private let testPath = "/some/random/path/not/important"
    private let testPing = "{ \"ping\": \"test\" }"
    private let testConfig = Configuration(
        userAgent: "Glean/Test 25.0.2",
        connectionTimeout: 3050,
        logPings: true,
        pingTag: "Tag"
    )

    private func setupHttpResponseStub(statusCode: Int32 = 200) {
        stub(condition: isHost("incoming.telemetry.mozilla.org")) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
            // swiftlint:disable force_try
            let json = try! JSONSerialization.jsonObject(with: body!, options: []) as? [String: Any]
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")

            return OHHTTPStubsResponse(
                jsonObject: [],
                statusCode: statusCode,
                headers: ["Content-Type": "application/json"]
            )
        }
    }

    func test2XX() {
        var testValue = false
        setupHttpResponseStub(statusCode: 200)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(path: testPath, data: testPing, config: testConfig) { success, _ in
            testValue = success
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertTrue(testValue, "`upload()` returns success")
    }

    func test3XX() {
        var testValue = true
        setupHttpResponseStub(statusCode: 300)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(path: testPath, data: testPing, config: testConfig) { success, _ in
            testValue = success
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertFalse(testValue, "`upload()` returns failure")
    }

    func test4XX() {
        var testValue = false
        setupHttpResponseStub(statusCode: 400)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(path: testPath, data: testPing, config: testConfig) { success, _ in
            testValue = success
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertTrue(testValue, "`upload()` returns success")
    }

    func test5XX() {
        var testValue = true
        setupHttpResponseStub(statusCode: 500)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader()
        httpPingUploader.upload(path: testPath, data: testPing, config: testConfig) { success, _ in
            testValue = success
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertFalse(testValue, "`upload()` returns failure")
    }

    func testRequestParameters() {
        let request = HttpPingUploader().buildRequest(path: testPath, data: testPing, config: testConfig)

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
            request?.value(forHTTPHeaderField: "User-Agent"),
            testConfig.userAgent,
            "Request User-Agent set correctly"
        )
        XCTAssertEqual(
            request?.timeoutInterval,
            TimeInterval(testConfig.connectionTimeout),
            "Request timeout value set correctly"
        )
        XCTAssertEqual(
            request?.httpBody,
            Data(testPing.utf8),
            "Request body set correctly"
        )
        XCTAssertEqual(
            request?.value(forHTTPHeaderField: "Content-Type"),
            "application/json; charset=utf-8",
            "Request Content-Type set correctly"
        )
        XCTAssertEqual(
            request?.httpShouldHandleCookies,
            false,
            "Request cookie policy set correctly"
        )
        XCTAssertEqual(
            request?.value(forHTTPHeaderField: "X-Client-Type"),
            "Glean",
            "Request X-Client-Type set correctly"
        )
        XCTAssertEqual(
            request?.value(forHTTPHeaderField: "X-Client-Version"),
            Configuration.getGleanVersion(),
            "Request X-Client-Version set correctly"
        )
    }
}
