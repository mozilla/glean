/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import OHHTTPStubs
import XCTest

// REASON: This test is long because of setup boilerplate
class HttpPingUploaderTests: XCTestCase {
    var expectation: XCTestExpectation?
    private let testPath = "/some/random/path/not/important"
    private let testPing = "{ \"ping\": \"test\" }"
    private let testConfig = Configuration(
        userAgent: "Glean/Test 25.0.2",
        logPings: true,
        pingTag: "Tag"
    )

    override func tearDown() {
        // Reset expectations
        expectation = nil
    }

    private func setupHttpResponseStub(statusCode: Int32 = 200) {
        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
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

    func getOrCreatePingDirectory(_ pingDirectory: String) -> URL {
        let dataPath = getGleanDirectory().appendingPathComponent(pingDirectory)

        if !FileManager.default.fileExists(atPath: dataPath.relativePath) {
            do {
                try FileManager.default.createDirectory(
                    atPath: dataPath.relativePath,
                    withIntermediateDirectories: true,
                    attributes: nil
                )
            } catch {
                print(error.localizedDescription)
            }
        }

        return dataPath
    }

    private func getAndClearPingDirectory(withDirectory pingDirectory: String? = nil) -> URL {
        // Get the ping directory.
        //
        // This is the same pending pings directory as defined in `glean-core/src/lib.rs`.
        // We want to test interoperation between the Swift and Rust parts.
        let pingDir = getOrCreatePingDirectory(pingDirectory ?? "pending_pings")

        // Clear the directory to ensure we start fresh
        // Verify all the files were removed, including the bad ones
        do {
            let directoryContents = try FileManager.default.contentsOfDirectory(
                atPath: pingDir.relativePath
            )
            for file in directoryContents {
                try FileManager.default.removeItem(
                    atPath: pingDir.appendingPathComponent(file).relativePath
                )
            }
        } catch {
            // Do nothing
        }

        return pingDir
    }

    func testHTTPStatusCode() {
        var testValue: UploadResult?
        setupHttpResponseStub(statusCode: 200)

        expectation = expectation(description: "Completed upload")

        let httpPingUploader = HttpPingUploader(configuration: testConfig)
        httpPingUploader.upload(path: testPath, data: Data(testPing.utf8), headers: [:]) { result in
            testValue = result
            self.expectation?.fulfill()
        }
        waitForExpectations(timeout: 5.0) { error in
            XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
        }

        // `UploadResult` is not `Equatable`, so instead of implementing that we just unpack it
        if case let .httpResponse(statusCode) = testValue {
            XCTAssertEqual(200, statusCode, "`upload()` returns the expected HTTP status code")
        } else {
            XCTAssertTrue(false, "`upload()` returns the expected HTTP status code")
        }
    }

    func testRequestParameters() {
        // Build a request.
        // We specify a single additional header here.
        // In usual code they are generated on the Rust side.
        let request = HttpPingUploader(configuration: testConfig)
            .buildRequest(path: testPath, data: Data(testPing.utf8), headers: ["X-Client-Type": "Glean"])

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
            request?.value(forHTTPHeaderField: "X-Client-Type"),
            "Glean",
            "Request X-Client-Type set correctly"
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
                atPath: file.relativePath,
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
            atPath: badFile.relativePath,
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

        // Reset Glean to trigger refetching pending pings
        Glean.shared.resetGlean(clearStores: true)

        // Trigger file processing and wait for the expectation to be fulfilled.  This will cause
        // a test failure if the expectation times out.
        HttpPingUploader(configuration: testConfig).process()
        wait(for: [expectation!], timeout: TimeInterval(5.0))
    }

    func testProcessingOfPingFilesAlternativeDirectory() {
        // This is the same deletion request ping directory as defined in `glean-core/src/lib.rs`.
        // We want to test interoperation between the Swift and Rust parts.
        let alternativePingdir = "deletion_request"
        let pingDir = getAndClearPingDirectory(withDirectory: alternativePingdir)

        // Write some simulated ping files into the directory for testing
        let fileContents = "\(testPath)\n\(testPing)\n".data(using: .utf8)
        let fileName = UUID().description
        let file = pingDir.appendingPathComponent(fileName)

        if !FileManager.default.createFile(
            atPath: file.relativePath,
            contents: fileContents,
            attributes: nil
        ) {
            XCTAssert(false, "Failed to write file \(file.relativePath)")
        }

        // Now let's write a ping with a non-conforming filename to ensure that they get
        // handled by the processor correctly
        let badFileName = "BadFileName"
        let badFile = pingDir.appendingPathComponent(badFileName)
        FileManager.default.createFile(
            atPath: badFile.relativePath,
            contents: fileContents,
            attributes: nil
        )

        // Now set up our test server
        let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
        stub(condition: isHost(host)) { data in
            let body = (data as NSURLRequest).ohhttpStubs_HTTPBody()
            let json = try! JSONSerialization.jsonObject(with: body!, options: []) as? [String: Any]
            XCTAssert(json != nil)
            XCTAssertEqual(json?["ping"] as? String, "test")

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

        // Set our expectation that will be fulfilled by the stub above
        expectation = expectation(description: "Completed upload")

        // Reset Glean to trigger refetching pending pings
        Glean.shared.resetGlean(clearStores: true)

        // Trigger file processing and wait for the expectation to be fulfilled.  This will cause
        // a test failure if the expectation times out.
        HttpPingUploader(configuration: testConfig).process()
        wait(for: [expectation!], timeout: TimeInterval(5.0))
    }
}
