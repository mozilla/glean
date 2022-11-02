/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class represents a ping uploader via HTTP.
///
/// This will typically be invoked by the appropriate scheduling mechanism to upload a ping to the server.
public class HttpPingUploader {
    var config: Configuration
    var testingMode: Bool

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
    public init(configuration: Configuration, testingMode: Bool = false) {
        self.config = configuration
        self.testingMode = testingMode
    }

    /// Launch a new ping uploader on the background thread.
    ///
    /// This function doesn't block.
    static func launch(configuration: Configuration, _ testingMode: Bool = false) {
        Dispatchers.shared.launchAsync {
            HttpPingUploader(configuration: configuration, testingMode: testingMode).process()
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
            if self.testingMode {
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
        }
    }

    /// Internal function that builds the request used for uploading the pings.
    ///
    /// - parameters:
    ///     * path: The URL path to append to the server address
    ///     * data: The serialized text data to send
    ///     * headers: Map of headers from Glean to annotate ping with
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
        let task = gleanGetUploadTask()

        switch task {
        case let .upload(request):
            var body = Data(capacity: request.body.count)
            body.append(contentsOf: request.body)
            self.upload(path: request.path, data: body, headers: request.headers) { result in
                let action = gleanProcessPingUploadResponse(request.documentId, result)
                switch action {
                case .next:
                    // launch a new iteration.
                    Dispatchers.shared.launchAsync {
                        HttpPingUploader(configuration: self.config, testingMode: self.testingMode).process()
                    }
                case .end:
                    return
                }

            }
        case .wait(let time):
            sleep(UInt32(time) / 1000)
            // launch a new iteration.
            Dispatchers.shared.launchAsync {
                HttpPingUploader(configuration: self.config, testingMode: self.testingMode).process()
            }
        case .done:
            return
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
        let statusCode = Int32(httpResponse?.statusCode ?? 0)

        if error != nil {
            // Upload failed on the client-side. We should try again.
            callback(.recoverableFailure(unused: 0))
        } else {
            // HTTP status codes are handled on the Rust side
            callback(.httpStatus(code: statusCode))
        }
    }
}
