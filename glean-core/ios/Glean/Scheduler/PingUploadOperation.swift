/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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

/// Represents an `Operation` encapsulating an HTTP request that uploads a
/// ping to the server. This implements the recommended pieces for execution
/// on a concurrent queue per the documentation for the `Operation` class
/// found [here](https://developer.apple.com/documentation/foundation/operation)
class PingUploadOperation: GleanOperation {
    var uploadTask: URLSessionUploadTask?
    let request: URLRequest
    let data: Data?
    let callback: (UploadResult) -> Void

    var backgroundTaskId = UIBackgroundTaskIdentifier.invalid

    /// Create a new PingUploadOperation
    ///
    /// - parameters:
    ///     * request: The `URLRequest` used to upload the ping to the server
    ///     * callback: The callback that the underlying data task returns results through
    init(request: URLRequest, data: Data?, callback: @escaping (UploadResult) -> Void) {
        self.request = request
        self.data = data
        self.callback = callback
    }

    /// Handles cancelling the underlying data task
    public override func cancel() {
        uploadTask?.cancel()
        super.cancel()
    }

    /// Starts the data task to upload the ping to the server
    override func start() {
        if self.isCancelled {
            finish(true)
            return
        }

        // Build a URLSession with no-caching suitable for uploading our pings
        let config = URLSessionConfiguration.default
        config.requestCachePolicy = NSURLRequest.CachePolicy.reloadIgnoringLocalCacheData
        config.urlCache = nil
        let session = URLSession(configuration: config)

        // This asks the OS for more time when going to background in order to allow for background
        // uploading of the pings.
        backgroundTaskId = UIApplication.shared.beginBackgroundTask(withName: "Glean Upload Task") {
            // End the task if time expires
            UIApplication.shared.endBackgroundTask(self.backgroundTaskId)
            self.backgroundTaskId = .invalid
        }

        // Create an URLSessionUploadTask to upload our ping in the background and handle the
        // server responses.
        uploadTask = session.uploadTask(with: request, from: data) { _, response, error in
            let httpResponse = response as? HTTPURLResponse
            let statusCode = UInt32(httpResponse?.statusCode ?? 0)

            if let error = error {
                // Upload failed on the client-side. We should try again.
                self.callback(.recoverableFailure(error))
            } else {
                // HTTP status codes are handled on the Rust side
                self.callback(.httpResponse(statusCode))
            }

            self.executing(false)
            self.finish(true)

            // End background task assertion to let the OS know we are done with our tasks
            UIApplication.shared.endBackgroundTask(self.backgroundTaskId)
            self.backgroundTaskId = UIBackgroundTaskIdentifier.invalid
        }

        executing(true)
        main()
    }

    override func main() {
        uploadTask?.resume()
    }
}
