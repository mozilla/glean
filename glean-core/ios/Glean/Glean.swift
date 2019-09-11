//
//  Glean.swift
//  Glean
//
//  Created by Jan-Erik Rediger on 27.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import Foundation

public class Glean {
    public static let shared = Glean()

    private var initialized: Bool = false
    private var handle: UInt64 = 0
    private var uploadEnabled: Bool = true

    private init() {
        // intentionally left blank
    }

    deinit {
        self.handle = 0
        self.initialized = false
    }

    public func initialize(configuration _: Configuration) {
        var cfg = FfiConfiguration(
            data_dir: "/tmp",
            package_name: "ios",
            upload_enabled: uploadEnabled ? 1 : 0,
            max_events: nil
        )
        handle = glean_initialize(&cfg)
        initialized = true
    }

    public func setUploadEnabled(enabled: Bool) {
        uploadEnabled = enabled

        if isInitialized() {
            glean_set_upload_enabled(handle, enabled ? 1 : 0)
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
