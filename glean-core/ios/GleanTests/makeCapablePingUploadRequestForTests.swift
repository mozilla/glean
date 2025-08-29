/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean

/// Builds a `CapablePingUploadRequest` through the internal APIs so we can test public APIs
struct CapablePingUploadRequestBuilder {
    func makeCapablePingUploadRequestForTests(
        url: String,
        path: String,
        data: [UInt8],
        headers: HeadersList,
        capabilities: [String]
    ) -> CapablePingUploadRequest {
        let testOhttpRequest = PingRequest(
            documentId: "12345",
            path: path,
            body: data,
            headers: headers,
            bodyHasInfoSections: true,
            pingName: "testPing",
            uploaderCapabilities: capabilities
        )
        let request = PingUploadRequest(
            request: testOhttpRequest,
            endpoint: url
        )
        return CapablePingUploadRequest(request)
    }
}
