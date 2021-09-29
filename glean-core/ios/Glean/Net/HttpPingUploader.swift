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
        // Build the request and create an async upload operation using a background URLSession
        if let request = self.buildRequest(path: path, data: data, headers: headers) {
            // Try to write the temporary file which the URLSession will use for transfer. If this fails
            // then there is no need to create the URLSessionConfiguration or URLSession.
            let tmpFile = URL.init(fileURLWithPath: NSTemporaryDirectory(),
                                   isDirectory: true).appendingPathComponent("\(path.split(separator: "/").last!)")
            do {
                try data.write(to: tmpFile, options: .noFileProtection)
            } catch {
                // Since we cannot write the file, there is no need to continue and schedule an
                // upload task. So instead we log the error and return.
                self.logger.error("\(error)")
                return
            }

            // Build a URLSession with no-caching suitable for uploading our pings
            let config: URLSessionConfiguration
            if Dispatchers.shared.testingMode {
                // For test mode, we want the URLSession to send things ASAP, rather than in the background
                config = URLSessionConfiguration.default
            } else {
                // For normal use cases, we will take advantage of the background URLSessionConfiguration
                // which will pass the data to the OS as a file and the OS will then handle the request
                // in a separate process
                config = URLSessionConfiguration.background(withIdentifier: path)
            }
            config.sessionSendsLaunchEvents = false // We don't need to notify the app when we are done.
            config.requestCachePolicy = NSURLRequest.CachePolicy.reloadIgnoringLocalCacheData
            config.isDiscretionary = false
            config.urlCache = nil
            let session = URLSession(configuration: config,
                                     delegate: SessionResponseDelegate(callback),
                                     delegateQueue: Dispatchers.shared.serialOperationQueue)

            // Create an URLSessionUploadTask to upload our ping in the background and handle the
            // server responses.
            let uploadTask = session.uploadTask(with: request, fromFile: tmpFile)

            uploadTask.countOfBytesClientExpectsToSend = 1024 * 1024
            uploadTask.countOfBytesClientExpectsToReceive = 512

            // Start the upload task
            uploadTask.resume()

            // Since we won't be reusing this session, we can call `finishTasksAndInvalidate` which
            // should allow our upload task to complete and then invalidate the session and release
            // the strong reference to the delegate.
            session.finishTasksAndInvalidate()
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

/// An object that can be assigned as a session delegate and will hold the callback with which to respond to
/// glean-core with the network response
private class SessionResponseDelegate: NSObject, URLSessionTaskDelegate {
    private let callback: (UploadResult) -> Void

    init(_ callback: @escaping (UploadResult) -> Void) {
        self.callback = callback
    }

    public func urlSession(_ session: URLSession, task: URLSessionTask, didCompleteWithError error: Error?) {
        let httpResponse = task.response as? HTTPURLResponse
        let statusCode = UInt32(httpResponse?.statusCode ?? 0)

        if let error = error {
            // Upload failed on the client-side. We should try again.
            callback(.recoverableFailure(error))
        } else {
            // HTTP status codes are handled on the Rust side
            callback(.httpResponse(statusCode))
        }
    }
}

enum UploadResult {
    /// A HTTP response code.
    ///
    /// This can still indicate an error, depending on the status code.
    case httpResponse(UInt32)

    /// An unrecoverable upload failure.
    ///
    /// A possible cause might be a malformed URL.
    case unrecoverableFailure(Error)

    /// A recoverable failure.
    ///
    /// During upload something went wrong,
    /// e.g. the network connection failed.
    /// The upload should be retried at a later time.
    case recoverableFailure(Error)

    func toFfi() -> UInt32 {
        switch self {
        case let .httpResponse(status):
            return UInt32(UPLOAD_RESULT_HTTP_STATUS) | status
        case .unrecoverableFailure:
            return UInt32(UPLOAD_RESULT_UNRECOVERABLE)
        case .recoverableFailure:
            return UInt32(UPLOAD_RESULT_RECOVERABLE)
        }
    }
}
