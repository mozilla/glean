/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import XCTest

/// Making sure the right `PingUploader` APIs are public
final class PingUploaderTests: XCTestCase {
    let capabilities = ["ohttp"]
    let url = "http://example.com"
    let path = "/hello"
    let data = [UInt8]("{ \"ping\": \"test\" }".utf8)
    let headers: HeadersList = [:]

    func testPingRequestAccess_withCapableAccess() throws {
        let subject = createSubject()
        let request = try XCTUnwrap(subject.capable(capabilities))

        // This data is accessible so customs `PingUploader` can build requests
        XCTAssertEqual(url + path, request.url)
        XCTAssertEqual(data, request.data)
        XCTAssertEqual(headers, request.headers)
    }

    func testPingRequestAccess_withoutCapableAccess() throws {
        let subject = createSubject()
        XCTAssertNil(subject.capable(["dummy"]))
    }

    func createSubject() -> CapablePingUploadRequest {
        let builder = CapablePingUploadRequestBuilder()
        let subject = builder.makeCapablePingUploadRequestForTests(
            url: url,
            path: path,
            data: data,
            headers: headers,
            capabilities: capabilities
        )
        return subject
    }
}
