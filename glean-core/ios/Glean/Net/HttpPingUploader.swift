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

    /// Initialize the HTTP Ping uploader from a Glean configuration object
    /// and an optional directory name.
    ///
    /// If the path is `nil` the default name will be used.
    ///
    /// - parameters:
    ///     * configuration: The Glean configuration to use.
    public init(configuration: Configuration) {
        self.config = configuration
    }

    /// Launch a new ping uploader on the background thread.
    ///
    /// This function doesn't block.
    static func launch(configuration: Configuration) {
        Dispatchers.shared.launchConcurrent {
            HttpPingUploader(configuration: configuration).process()
        }
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
    func upload(path: String, data: Data, headers: [String: String], callback: @escaping (UploadResult) -> Void) {
        // Build the request and create an async upload operation and launch it through the
        // Dispatchers
        if let request = buildRequest(path: path, data: data, headers: headers) {
            let uploadOperation = PingUploadOperation(request: request, data: data, callback: callback)
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
    func buildRequest(path: String, data: Data, headers: [String: String]) -> URLRequest? {
        if let url = URL(string: config.serverEndpoint + path) {
            var request = URLRequest(url: url)
            for (field, value) in headers {
                request.addValue(value, forHTTPHeaderField: field)
            }
            request.timeoutInterval = TimeInterval(Constants.connectionTimeout)
            request.httpMethod = "POST"
            request.httpShouldHandleCookies = false

            // NOTE: We're using `URLSession.uploadTask` in `PingUploadOperation`,
            // which ignores the `httpBody` and instead takes the body payload as a parameter
            // to add to the request.
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

        return nil
    }

    /// This function gets an upload task from Glean and, if told so, uploads the data.
    ///
    /// It will report back the task status to Glean, which will take care of deleting pending ping files.
    /// It will continue upload as long as it can fetch new tasks.
    func process() {
        // Limits are enforced by glean-core to avoid an inifinite loop here.
        // Whenever a limit is reached, this binding will receive `.done` and step out.
        while true {
            var incomingTask = FfiPingUploadTask()
            glean_get_upload_task(&incomingTask)
            let task = incomingTask.toPingUploadTask()

            switch task {
            case let .upload(request):
                self.upload(path: request.path, data: request.body, headers: request.headers) { result in
                    glean_process_ping_upload_response(&incomingTask, result.toFfi())
                }
            case .wait(let time):
                sleep(UInt32(time) / 1000)
                continue
            case .done:
                return
            }
        }
    }
}
