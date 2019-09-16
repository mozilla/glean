/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// The main Glean API.
///
/// This is exposed through the global `Glean.shared` object.
public class Glean {
    /// The main Glean object.
    ///
    /// Before any data collection can take place, the Glean SDK **must** be initialized from the application.
    ///
    /// ```swift
    /// Glean.shared.setUploadEnabled(true)
    /// Glean.shared.initialize()
    /// ```
    public static let shared = Glean()

    var handle: UInt64 = 0
    private var initialized: Bool = false
    private var uploadEnabled: Bool = true
    private var configuration: Configuration?
    static var backgroundTaskId = UIBackgroundTaskIdentifier.invalid

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/Glean"
    }

    private init() {
        // intentionally left private, no external user can instantiate a new global object.

        // Enable logging in the Rust library
        glean_enable_logging()
    }

    deinit {
        self.handle = 0
        self.initialized = false
    }

    /// Initialize the Glean SDK.
    ///
    /// This should only be initialized once by the application, and not by
    /// libraries using the Glean SDK. A message is logged to error and no
    /// changes are made to the state if initialize is called a more than
    /// once.
    ///
    /// A LifecycleObserver will be added to send pings when the application goes
    /// into the background.
    ///
    /// - parameters:
    ///     * configuration: A Glean `Configuration` object with global settings.
    public func initialize(configuration: Configuration = Configuration()) {
        if self.isInitialized() {
            NSLog("Glean: Glean should not be initialized multiple times")
            return
        }

        self.configuration = configuration

        self.handle = withFfiConfiguration(
            dataDir: getDocumentsDirectory().absoluteString,
            packageName: Bundle.main.bundleIdentifier!,
            uploadEnabled: uploadEnabled,
            configuration: configuration
        ) { cfg in
            var cfg = cfg
            return glean_initialize(&cfg)
        }

        // Signal Dispatcher that init is complete
        Dispatchers.shared.flushQueuedInitialTasks()

        self.initialized = true
    }

    /// Enable or disable Glean collection and upload.
    ///
    /// Metric collection is enabled by default.
    ///
    /// When uploading is disabled, metrics aren't recorded at all and no data
    /// is uploaded.
    ///
    /// When disabling, all pending metrics, events and queued pings are cleared.
    ///
    /// When enabling, the core Glean metrics are recreated.
    ///
    /// - parameters:
    ///     * enabled: When true, enable metric collection.
    public func setUploadEnabled(_ enabled: Bool) {
        uploadEnabled = enabled

        if isInitialized() {
            glean_set_upload_enabled(handle, enabled ? 1 : 0)
        }
    }

    /// Get whether or not Glean is allowed to record and upload data.
    public func getUploadEnabled() -> Bool {
        if isInitialized() {
            return glean_is_upload_enabled(handle) != 0
        } else {
            return uploadEnabled
        }
    }

    /// Returns true if the Glean SDK has been initialized.
    func isInitialized() -> Bool {
        return handle != 0
    }

    /// Handle background event and send appropriate pings
    func handleBackgroundEvent() {
        // This asks the OS for more time when going to background to perform critical tasks.
        // This will be signaled complete when `sendPingsByName` is done.
        Glean.backgroundTaskId = UIApplication.shared.beginBackgroundTask(withName: "Glean Upload Task") {
            // End the task if time expires
            UIApplication.shared.endBackgroundTask(Glean.backgroundTaskId)
            Glean.backgroundTaskId = UIBackgroundTaskIdentifier.invalid
        }

        self.sendPingsByName(pingNames: ["baseline", "events"])

        // End task assertion
        UIApplication.shared.endBackgroundTask(Glean.backgroundTaskId)
        Glean.backgroundTaskId = UIBackgroundTaskIdentifier.invalid
    }

    /// Send a list of pings by name
    ///
    /// - parameters:
    ///     * pingNames: List of ping names to send
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// If the ping currently contains no content, it will not be assembled and
    /// queued for sending.
    func sendPingsByName(pingNames: [String]) {
        _ = Dispatchers.shared.launch {
            if !self.isInitialized() {
                NSLog("\(Constants.logTag) : Glean must be initialized before sending pings")
                return
            }

            if !self.getUploadEnabled() {
                NSLog("\(Constants.logTag) : Glean must be enabled before sending pings")
                return
            }

            withArrayOfCStrings(pingNames) { pingNames in
                let sentPing = glean_send_pings_by_name(
                    self.handle,
                    pingNames,
                    Int32(pingNames.count),
                    self.configuration!.logPings ? 1 : 0
                )

                if sentPing != 0 {
                    if let config = self.configuration {
                        HttpPingUploader(configuration: config).process()
                    }
                }
            }
        }
    }

    /// Test-only method to destroy the owned glean-core handle.
    func testDestroyGleanHandle() {
        if !isInitialized() {
            // We don't need to destroy the Glean handle: it wasn't initialized.
            return
        }

        glean_destroy_glean(handle)
        handle = 0
    }

    /// TEST ONLY FUNCTION.
    ///
    /// Enable test mode.
    ///
    /// This makes all asynchronous work synchronous so we can test the results of the
    /// API synchronously.
    func enableTestingMode() {
        Dispatchers.shared.setTestingMode(enabled: true)
    }

    /// TEST ONLY FUNCTION.
    ///
    /// Resets the Glean state and trigger init again.
    ///
    /// - parameters:
    ///     * configuration: the `Configuration` to init Glean with
    ///     * clearStores: if true, clear the contents of all stores
    func resetGlean(configuration: Configuration = Configuration(), clearStores: Bool) {
        enableTestingMode()

        if isInitialized() && clearStores {
            // Clear all the stored data.
            glean_test_clear_all_stores(handle)
        }

        // Init Glean.
        testDestroyGleanHandle()
        setUploadEnabled(true)
        initialize(configuration: configuration)
    }
}
