/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// This class represents a ping uploader via HTTP.
///
/// This will typically be invoked by the appropriate scheduling mechanism to upload a ping to the server.
public class HttpPingUploader {
    var config: Configuration

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        // Since ping file names are UUIDs, this matches UUIDs for filtering purposes
        static let logTag = "glean/HttpPingUploader"
        static let connectionTimeout = 10000

        // For this error, the ping will be retried later
        static let recoverableErrorStatusCode: UInt16 = 500
        // For this error, the ping data will be deleted and no retry happens
        static let unrecoverableErrorStatusCode: UInt16 = 400
    }

    private let logger = Logger(tag: Constants.logTag)

    public init(configuration: Configuration) {
        self.config = configuration
    }

    /// A function to aid in logging the ping to the console via `NSLog`.
    ///
    /// - parameters:
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    func logPing(path: String, data: String) {
        logger.debug("Glean ping to URL: \(path)\n\(data)")
    }

    /// Synchronously upload a ping to Mozilla servers.
    ///
    /// - parameters:
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    ///     * callback: A callback to return the success/failure of the upload
    ///
    /// Note that the `X-Client-Type`: `Glean` and `X-Client-Version`: <SDK version>
    /// headers are added to the HTTP request in addition to the UserAgent. This allows
    /// us to easily handle pings coming from Glean on the legacy Mozilla pipeline.
    func upload(path: String, data: String, headers: [String: String], callback: @escaping (UploadResult) -> Void) {
        if config.logPings {
            logPing(path: path, data: data)
        }

        // Build the request and create an async upload operation and launch it through the
        // Dispatchers
        if let request = buildRequest(path: path, data: data, headers: headers) {
            let uploadOperation = PingUploadOperation(request: request, data: Data(data.utf8), callback: callback)
            Dispatchers.shared.launchConcurrent(operation: uploadOperation)
        }
    }

    /// Internal function that builds the request used for uploading the pings.
    ///
    /// - parameters:
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    ///     * callback: A callback to return the success/failure of the upload
    ///
    /// - returns: Optional `URLRequest` object with the configured headings set.
    func buildRequest(path: String, data: String, headers: [String: String]) -> URLRequest? {
        if let url = URL(string: config.serverEndpoint + path) {
            var request = URLRequest(url: url)
            for (field, value) in headers {
                request.addValue(value, forHTTPHeaderField: field)
            }
            request.timeoutInterval = TimeInterval(Constants.connectionTimeout)
            request.httpMethod = "POST"
            request.httpBody = Data(data.utf8)
            request.httpShouldHandleCookies = false

            if let tag = config.pingTag {
                request.addValue(tag, forHTTPHeaderField: "X-Debug-ID")
            }

            return request
        }

        return nil
    }

    /// This function gets an upload task from Glean and, if told so, uploads the data.
    ///
    /// It will report back the task status to Glean, which will take care of deleting pending ping files.
    /// It will continue upload as long as it can fetch new tasks.
    func process() {
        while true {
            let incomingTask = glean_get_upload_task()
            let task = incomingTask.toPingUploadTask()

            switch task {
            case let .upload(request):
                self.upload(path: request.path, data: request.body, headers: request.headers) { result in
                    let status: UInt16

                    switch result {
                    case let .httpResponse(statusCode):
                        status = statusCode
                    case .unrecoverableFailure:
                        status = Constants.unrecoverableErrorStatusCode
                    case .recoverableFailure:
                        status = Constants.recoverableErrorStatusCode
                    }

                    glean_process_ping_upload_response(incomingTask, status)
                }
            case .wait:
                continue
            case .done:
                return
            }
        }
    }
}
