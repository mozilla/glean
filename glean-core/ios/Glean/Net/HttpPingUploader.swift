/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// `true` to allow the uploader to process pings.
/// Note that this does not mean that an uploader is actually running.
/// It will be invoked when a ping is submitted.
///
/// `false` to stop the uploader from starting new uploads.
/// An already running uploader will finish work and then stop.
var stateRunAllowed: AtomicBoolean = AtomicBoolean(false)

// TODO(bug 1816403): Move this and the associated global state
// into a singleton instance of `HttpPingUploader`.
func shutdownUploader() {
    stateRunAllowed.value = false
}

func startUploader() {
    stateRunAllowed.value = true
}

/// This class represents a ping uploader via HTTP.
///
/// This will typically be invoked by the appropriate scheduling mechanism to upload a ping to the server.
public class HttpPingUploader {
    var config: Configuration
    var session: URLSession

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
    /// and a URLSession
    ///
    /// - parameters:
    ///     * configuration: The Glean `Configuration` to use.
    ///     * session: A `URLSession` that will be reused to upload pings
    public init(configuration: Configuration, session: URLSession) {
        self.config = configuration
        self.session = session
    }

    /// Launch a new instance of a HttpPingUploader that requests additional time to run in the background
    /// in order to give Glean time to send pings when the app is closing.
    ///
    /// Also responsible for creating a session that will be reused for uploading all of the pings on this execution
    ///
    /// This function doesn't block.
    static func launch(configuration: Configuration) {
        Dispatchers.shared.launchAsync {
            var backgroundTaskId: UIBackgroundTaskIdentifier = .invalid

            // Begin the background task and save the id. We will reuse this same background task
            // for all the ping uploads
            backgroundTaskId = UIApplication.shared.beginBackgroundTask(withName: "Glean Upload Task") {
                // End the background task if we run out of time
                if backgroundTaskId != .invalid {
                    UIApplication.shared.endBackgroundTask(backgroundTaskId)
                    backgroundTaskId = .invalid
                }
            }

            // Build a URLSession with no-caching suitable for uploading our pings
            let config: URLSessionConfiguration = .default
            config.requestCachePolicy = NSURLRequest.CachePolicy.reloadIgnoringLocalCacheData
            config.urlCache = nil
            let session = URLSession(configuration: config)

            HttpPingUploader(configuration: configuration, session: session).process()

            if backgroundTaskId != .invalid {
                UIApplication.shared.endBackgroundTask(backgroundTaskId)
                backgroundTaskId = .invalid
            }
        }
    }

    /// Synchronously upload a ping to Mozilla servers.
    ///
    /// - parameters:
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    ///     * headers: Map of headers from Glean to annotate ping with
    ///     * callback: A callback to return the success/failure of the upload
    func upload(path: String, data: Data, headers: [String: String], callback: @escaping (UploadResult) -> Void) {
        // Build the request and create upload operation using a URLSession
        if let request = self.buildRequest(path: path, data: data, headers: headers) {
            // Create an URLSessionUploadTask to upload our ping and handle the
            // server responses.
            let uploadTask = session.uploadTask(with: request, from: data) { _, response, error in

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
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    ///     * headers: Map of headers from Glean to annotate ping with
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
        if !stateRunAllowed.value {
            self.logger.info("Not allowed to continue running. Bye!")
        }

        while true {
            // Limits are enforced by glean-core to avoid an infinite loop here.
            // Whenever a limit is reached, this binding will receive `.done` and step out.
            switch gleanGetUploadTask() {
            case let .upload(request):
                var body = Data(capacity: request.body.count)
                body.append(contentsOf: request.body)
                self.upload(path: request.path, data: body, headers: request.headers) { result in
                    if gleanProcessPingUploadResponse(request.documentId, result) == .end {
                        return
                    }
                }
            case .wait(let time):
                sleep(UInt32(time) / 1000)
            case .done:
                return
            }
        }
    }
}
