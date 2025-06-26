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

/// This is a scheduler used to handle ping uploading by processing the pings from glean-core and using the
/// uploader specified in the Glean `Configuration`.
///
/// This will typically be invoked by the appropriate scheduling mechanism to trigger uploading a ping to the server.
public class PingUploadScheduler {
    let httpUploader: PingUploader
    let httpEndpoint: String

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        // Since ping file names are UUIDs, this matches UUIDs for filtering purposes
        static let logTag = "glean/PingUploadScheduler"
        // For this error, the ping will be retried later
        static let recoverableErrorStatusCode: UInt16 = 500
        // For this error, the ping data will be deleted and no retry happens
        static let unrecoverableErrorStatusCode: UInt16 = 400
    }

    private let logger = Logger(tag: Constants.logTag)

    /// Initialize the ping scheduler from a Glean configuration object
    ///
    /// - parameters:
    ///     * configuration: The Glean `Configuration` to use which contains the endpoint and http uploader
    public init(configuration: Configuration) {
        if let httpUploader = configuration.httpClient {
            self.httpUploader = httpUploader
        } else {
            self.httpUploader = HttpPingUploader(configuration: configuration)
        }

        self.httpEndpoint = configuration.serverEndpoint
    }

    /// This function gets an upload task from Glean and, if told so, uploads the data using the http uploader
    ///
    /// It will report back the task status to Glean, which will take care of deleting pending ping files.
    /// It will continue upload as long as it can fetch new tasks.
    func process() {
        if !stateRunAllowed.value {
            self.logger.info("Not allowed to continue running. Bye!")
        }

        Dispatchers.shared.launchAsync {
            var backgroundTaskId: UIBackgroundTaskIdentifier = .invalid

            // Begin the background task and save the id. We will reuse this same background task
            // for all the ping uploads
            backgroundTaskId = UIApplication.shared.beginBackgroundTask(
                withName: "Glean Upload Task"
            ) {
                // End the background task if we run out of time
                if backgroundTaskId != .invalid {
                    UIApplication.shared.endBackgroundTask(backgroundTaskId)
                    backgroundTaskId = .invalid
                }
            }

            while true {
                // Limits are enforced by glean-core to avoid an infinite loop here.
                // Whenever a limit is reached, this binding will receive `.done` and step out.
                switch gleanGetUploadTask() {
                case let .upload(request):
                    var body = Data(capacity: request.body.count)
                    body.append(contentsOf: request.body)
                    let capableRequest = CapablePingUploadRequest(
                        PingUploadRequest(
                            request: request,
                            endpoint: self.httpEndpoint
                        )
                    )
                    self.httpUploader.upload(request: capableRequest) { result in
                        if gleanProcessPingUploadResponse(
                            request.documentId,
                            result
                        ) == .end {
                            return
                        }
                    }
                case .wait(let time):
                    sleep(UInt32(time) / 1000)
                case .done:
                    return
                }
            }

            if backgroundTaskId != .invalid {
                UIApplication.shared.endBackgroundTask(backgroundTaskId)
                backgroundTaskId = .invalid
            }
        }
    }
}
