/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Turn a string into an error, so that it can be thrown as an exception.
///
/// This should only be used in tests.
extension String: Error {
    public var errorDescription: String? { return self }
}

/// Checks to see if a string matches a regex
///
/// - returns: true if the string matches the regex
extension String {
    func matches(_ regex: String) -> Bool {
        return self.range(of: regex, options: .regularExpression, range: nil, locale: nil) != nil
    }
}

/// Helper function to retrive the application's Documents directory for persistent file storage
///
/// - returns: `String` representation of the path to the Documents directory
func getDocumentsDirectory() -> URL {
    let paths = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask)
    let documentsDirectory = paths[0]
    return documentsDirectory.appendingPathComponent("glean_data")
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

/// Create a temporary array of C-compatible (null-terminated) strings to pass over FFI.
///
/// The strings are deallocated after the closure returns.
///
/// - parameters:
///     * args: The array of strings to use
///     * body: The closure that gets an array of C-compatible strings
public func withArrayOfCStrings<R>(
    _ args: [String],
    _ body: ([UnsafePointer<CChar>?]) -> R
) -> R {
    var cStrings = args.map { UnsafePointer(strdup($0)) }
    defer {
        cStrings.forEach { free(UnsafeMutableRawPointer(mutating: $0)) }
    }
    return body(cStrings)
}

/// This struct creates a Boolean with atomic or synchronized access.
///
/// This makes use of synchronization tools from Grand Central Dispatch (GCD)
/// in order to synchronize access.
struct AtomicBoolean {
    private var semaphore = DispatchSemaphore(value: 1)
    private var val: Bool
    var value: Bool {
        get {
            semaphore.wait()
            let tmp = val
            semaphore.signal()
            return tmp
        }
        set {
            semaphore.wait()
            val = newValue
            semaphore.signal()
        }
    }

    init(_ initialValue: Bool = false) {
        val = initialValue
    }
}
