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

    private var initialized: Bool = false
    private var handle: UInt64 = 0
    private var uploadEnabled: Bool = true
    private var configuration: Configuration? = nil

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
            dataDir: getDocumentsDirectory(),
            packageName: Bundle.main.bundleIdentifier!,
            uploadEnabled: uploadEnabled,
            configuration: configuration
        ) { cfg in
            var cfg = cfg
            return glean_initialize(&cfg)
        }
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
    internal func isInitialized() -> Bool {
        return handle != 0
    }

    /// Handle background event and send appropriate pings
    internal func handleBackgroundEvent() {
        // sendPings()
    }
}
