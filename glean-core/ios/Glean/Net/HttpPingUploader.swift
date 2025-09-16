/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class represents a ping uploader via HTTP.
///
/// This will typically be invoked by the appropriate scheduling mechanism to upload a ping to the server.
public class HttpPingUploader: PingUploader {
    var session: URLSession
    var capabilities: [String] = []

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        // Since ping file names are UUIDs, this matches UUIDs for filtering purposes
        static let logTag = "glean/HttpPingUploader"
        static let connectionTimeout = 10
    }

    private let logger = Logger(tag: Constants.logTag)

    /// Initialize the HTTP Ping uploader from a Glean configuration object
    public init() {
        // Build a URLSession with no-caching suitable for uploading our pings
        let sessionConfig: URLSessionConfiguration = .default
        sessionConfig.requestCachePolicy =
            NSURLRequest.CachePolicy.reloadIgnoringLocalCacheData
        sessionConfig.urlCache = nil
        self.session = URLSession(configuration: sessionConfig)
    }

    /// Synchronously upload a ping to Mozilla servers.
    ///
    /// - parameters:
    ///     * request: A `CapablePingUploadRequest` containing the information needed to perform the upload
    ///     * callback: A callback to return the success/failure of the upload to the scheduler
    public func upload(
        request: CapablePingUploadRequest,
        callback: @escaping (UploadResult) -> Void
    ) {
        // This default reference HTTP uploader implementation does not support any capabilities
        // and so we return `.incapable` if a ping requires capabilites.
        // See https://bugzilla.mozilla.org/show_bug.cgi?id=1950143 for more info.
        guard let request = request.capable(self.capabilities) else {
            logger.info(
                "Glean rejected ping upload due to unsupported capabilities"
            )
            callback(.incapable(unused: 0))
            return
        }
        // Build the request and create upload operation using a URLSession
        var body = Data(capacity: request.data.count)
        body.append(contentsOf: request.data)
        if let request = self.buildRequest(
            url: request.url,
            data: body,
            headers: request.headers
        ) {
            // Create an URLSessionUploadTask to upload our ping and handle the
            // server responses.
            let uploadTask = session.uploadTask(with: request, from: body) { _, response, error in

                if let httpResponse = response as? HTTPURLResponse {
                    let statusCode = Int32(httpResponse.statusCode)

                    if error != nil {
                        // Upload failed on the client-side. We should try again.
                        callback(.recoverableFailure(unused: 0))
                    } else {
                        // HTTP status codes are handled on the Rust side
                        callback(.httpStatus(code: statusCode))
                    }
                }
            }

            uploadTask.countOfBytesClientExpectsToSend = 1024 * 1024
            uploadTask.countOfBytesClientExpectsToReceive = 512

            // Start the upload task
            uploadTask.resume()
        }
    }

    /// Internal function that builds the request used for uploading the pings.
    ///
    /// - parameters:
    ///     * url: The URL, including the path, to use for the destination of the ping
    ///     * data: The serialized text data to send
    ///     * headers: Map of headers from Glean to annotate ping with
    ///
    /// - returns: Optional `URLRequest` object with the configured headings set.
    func buildRequest(
        url: String,
        data: Data,
        headers: [String: String]
    ) -> URLRequest? {
        guard let url = URL(string: url) else {
            return nil
        }

        var request = URLRequest(url: url)
        for (field, value) in headers {
            request.addValue(value, forHTTPHeaderField: field)
        }
        request.timeoutInterval = TimeInterval(Constants.connectionTimeout)
        request.httpMethod = "POST"
        request.httpShouldHandleCookies = false

        // NOTE: We're using `URLSession.uploadTask` which ignores the `httpBody` and
        // instead takes the body payload as a parameter to add to the request.
        // However in tests we're using OHHTTPStubs to stub out the HTTP upload.
        // It has the known limitation that it doesn't simulate data upload,
        // because the underlying protocol doesn't expose a hook for that.
        // By setting `httpBody` here the data is still attached to the request,
        // so OHHTTPStubs sees it.
        // It shouldn't be too bad memory-wise and not duplicate the data in memory.
        // This should only be a reference and Swift keeps track of all the places it's needed.
        //
        // See https://github.com/AliSoftware/OHHTTPStubs#known-limitations.
        request.httpBody = data

        return request
    }
}
