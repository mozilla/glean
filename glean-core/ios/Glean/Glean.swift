/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// The main Glean API.
///
/// This is exposed through the global [Glean.shared] object.
public class Glean {
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

    public func initialize(configuration: Configuration = Configuration()) {
        self.configuration = configuration

        handle = withFfiConfiguration(
            dataDir: getDocumentsDirectory(),
            packageName: Bundle.main.bundleIdentifier!,
            uploadEnabled: uploadEnabled,
            configuration: configuration
        ) { cfg in
            var cfg = cfg
            return glean_initialize(&cfg)
        }
        initialized = true
    }

    public func setUploadEnabled(_ enabled: Bool) {
        uploadEnabled = enabled

        if isInitialized() {
            glean_set_upload_enabled(handle, enabled ? 1 : 0)
        }
    }

    public func getUploadEnabled() -> Bool {
        if isInitialized() {
            return glean_is_upload_enabled(handle) != 0
        } else {
            return uploadEnabled
        }
    }

    internal func isInitialized() -> Bool {
        return handle != 0
    }

    /// Handle background event and send appropriate pings
    internal func handleBackgroundEvent() {
        // sendPings()
    }
}
