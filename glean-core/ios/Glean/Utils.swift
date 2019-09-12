/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Helper function to retrive the application's Documents directory for persistent file storage
///
/// - returns: `String` representation of the path to the Documents directory
func getDocumentsDirectory() -> String {
    let paths = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
    let documentsDirectory = paths[0]
    return documentsDirectory.appendingPathComponent("glean_data").absoluteString
}

/// Create a temporary FFI configuration for the span of the closure.
///
/// We need to ensure strings exist across the FFI call, so we `strdup` them and clean up afterwards.
func withFfiConfiguration<R>(
    dataDir: String,
    packageName: String,
    uploadEnabled: Bool,
    configuration: Configuration,
    _ body: (FfiConfiguration) -> R
) -> R {
    let dataDir = strdup(dataDir)
    let packageName = strdup(packageName)

    var maxEventsPtr: UnsafeMutablePointer<Int32>?
    if let maxEvents = configuration.maxEvents {
        maxEventsPtr = UnsafeMutablePointer<Int32>.allocate(capacity: 1)
        maxEventsPtr!.initialize(to: maxEvents)
    }

    defer {
        free(dataDir)
        free(packageName)
        maxEventsPtr?.deallocate()
    }

    let cfg = FfiConfiguration(
        data_dir: dataDir,
        package_name: packageName,
        upload_enabled: uploadEnabled ? 1 : 0,
        max_events: maxEventsPtr
    )
    return body(cfg)
}
