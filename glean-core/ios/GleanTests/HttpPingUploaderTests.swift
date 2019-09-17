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
        logPings: true,
        pingTag: "Tag"
    )

    private func setupHttpResponseStub(statusCode: Int32 = 200) {
        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
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

    private func getAndClearPingDirectory() -> URL {
        // Get the ping directory
        let pingDir = HttpPingUploader.getOrCreatePingDirectory()

        // Clear the directory to ensure we start fresh
        // Verify all the files were removed, including the bad ones
        do {
            let directoryContents = try FileManager.default.contentsOfDirectory(
                atPath: pingDir.absoluteString
            )
            for file in directoryContents {
                try FileManager.default.removeItem(
                    atPath: pingDir.appendingPathComponent(file).absoluteString
                )
            }
        } catch {
            // Do nothing
        }

        return pingDir
    }

    func test2XX() {
        var testValue = false
        setupHttpResponseStub(statusCode: 200)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader(configuration: testConfig)
        httpPingUploader.upload(path: testPath, data: testPing) { success, _ in
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

        let httpPingUploader = HttpPingUploader(configuration: testConfig)
        httpPingUploader.upload(path: testPath, data: testPing) { success, _ in
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

        let httpPingUploader = HttpPingUploader(configuration: testConfig)
        httpPingUploader.upload(path: testPath, data: testPing) { success, _ in
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

        let httpPingUploader = HttpPingUploader(configuration: testConfig)
        httpPingUploader.upload(path: testPath, data: testPing) { success, _ in
            testValue = success
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        XCTAssertFalse(testValue, "`upload()` returns failure")
    }

    func testRequestParameters() {
        let request = HttpPingUploader(configuration: testConfig)
            .buildRequest(path: testPath, data: testPing)

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

    func testProcessingOfPingFiles() {
        // Get the ping directory
        let pingDir = getAndClearPingDirectory()

        // Write some simulated ping files into the directory for testing
        let fileContents = "\(testPath)\n\(testPing)\n".data(using: .utf8)
        let expectedFilesUploaded = 3
        for _ in 0 ..< expectedFilesUploaded {
            let fileName = UUID().description
            let file = pingDir.appendingPathComponent(fileName)

            if !FileManager.default.createFile(
                atPath: file.absoluteString,
                contents: fileContents,
                attributes: nil
            ) {
                print("Argh!!")
            }
        }

        // Now let's write a ping with a non-conforming filename to ensure that they get
        // handled by the processor correctly
        let badFileName = "BadFileName"
        let badFile = pingDir.appendingPathComponent(badFileName)
        FileManager.default.createFile(
            atPath: badFile.absoluteString,
            contents: fileContents,
            attributes: nil
        )

        // Now set up our test server
        var countFilesUploaded = 0

        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
            let json = try! JSONSerialization.jsonObject(with: body!, options: []) as? [String: Any]
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")

            countFilesUploaded += 1
            if expectedFilesUploaded == countFilesUploaded {
                DispatchQueue.main.async {
                    // let the response get processed before we mark the expectation fulfilled
                    self.expectation?.fulfill()
                }
            }

            return OHHTTPStubsResponse(
                jsonObject: [],
                statusCode: 200,
                headers: ["Content-Type": "application/json"]
            )
        }

        // Set our expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Completed upload")

        // Trigger file processing and wait for the expectation to be fulfilled.  This will cause
        // a test failure if the expectation times out.
        HttpPingUploader(configuration: testConfig).process()
        wait(for: [expectation!], timeout: TimeInterval(5.0))
    }
}
