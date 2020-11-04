/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

extension Bool {
    /// Convert a bool to its byte equivalent.
    func toByte() -> UInt8 {
        return self ? 1 : 0
    }
}

extension UInt8 {
    /// Convert a byte to its Bool equivalen.
    func toBool() -> Bool {
        return self != 0
    }
}

/// Turn a string into an error, so that it can be thrown as an exception.
///
/// This should only be used in tests.
extension String: Error {
    /// The string itself is the error description.
    public var errorDescription: String? { return self }
}

/// Format specifiers for different ISO8601 timestamps
let dateFormatPatterns: [TimeUnit: String] = [
    .nanosecond: "yyyy-MM-dd'T'HH:mm:ss.SSSZZZZZ",
    .microsecond: "yyyy-MM-dd'T'HH:mm:ss.SSSZZZZZ",
    .millisecond: "yyyy-MM-dd'T'HH:mm:ss.SSSZZZZZ",
    .second: "yyyy-MM-dd'T'HH:mm:ssZZZZZ",
    .minute: "yyyy-MM-dd'T'HH:mmZZZZZ",
    .hour: "yyyy-MM-dd'T'HHZZZZZ",
    .day: "yyyy-MM-ddZZZZZ"
]

extension Date {
    /// Convenience function to convert ISO8601 string to a Date.
    ///
    /// Note that passing in a `dateString` that has more precision than the `precision` parameter will
    /// result in a `nil` value being returned.  Passing in a `dateString` with less precision than `precision`
    /// will still result in a valid `Date` being returned.
    ///
    /// - parameters:
    ///     * dateString: The `String` representing the date to convert, i.e.: `2004-12-09T08:03-08:00`
    ///     * precision: The `TimeUnit` precision to use for selecting the correct format to parse against
    static func fromISO8601String(dateString: String, precision: TimeUnit) -> Date? {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = dateFormatPatterns[precision]
        return dateFormatter.date(from: dateString)
    }

    /// Convenience function to convert a Date to an ISO8601 string
    ///
    /// - returns: An ISO8601 `String` representing the current value of the `Date` object
    func toISO8601String(precision: TimeUnit) -> String {
        let dateFormatter = DateFormatter()
        dateFormatter.dateFormat = dateFormatPatterns[precision]
        return dateFormatter.string(from: self)
    }

    /// Overloads the operator so that subtraction between two dates results in a TimeInterval representing
    /// the difference between them
    static func - (lhs: Date, rhs: Date) -> TimeInterval {
        return lhs.timeIntervalSinceReferenceDate - rhs.timeIntervalSinceReferenceDate
    }
}

extension String {
    /// Create a string from a Rust-allocated char pointer and deallocate the char pointer.
    public init(freeingGleanString rustString: UnsafeMutablePointer<CChar>) {
        defer { glean_str_free(rustString) }
        self.init(cString: rustString)
    }

    /// Checks to see if a string matches a regex
    ///
    /// - returns: true if the string matches the regex
    func matches(_ regex: String) -> Bool {
        return self.range(of: regex, options: .regularExpression, range: nil, locale: nil) != nil
    }

    /// Conveniently convert a string path to a file URL
    ///
    /// - returns: File `URL` represeting the path contained in the string
    var fileURL: URL {
        return URL(fileURLWithPath: self)
    }

    /// Gets the last path components, such as the file name from a string path
    ///
    /// - returns: `String` representing the last path component
    var lastPathComponent: String {
        return fileURL.lastPathComponent
    }
}

/// Helper function to retrive the application's Application Support directory for persistent file storage
///
/// - returns: `URL` of the Application Support directory
func getGleanDirectory() -> URL {
    let paths = FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask)
    let documentsDirectory = paths[0]
    return documentsDirectory.appendingPathComponent("glean_data")
}

// swiftlint:disable function_parameter_count
/// Create a temporary FFI configuration for the span of the closure.
///
/// We need to ensure strings exist across the FFI call, so we `strdup` them and clean up afterwards.
func withFfiConfiguration<R>(
    dataDir: String,
    packageName: String,
    languageBindingName: String,
    uploadEnabled: Bool,
    configuration: Configuration,
    _ body: (FfiConfiguration) -> R
) -> R {
    let dataDir = strdup(dataDir)
    let packageName = strdup(packageName)
    let languageBindingName = strdup(languageBindingName)

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
        language_binding_name: languageBindingName,
        upload_enabled: uploadEnabled.toByte(),
        max_events: maxEventsPtr,
        delay_ping_lifetime_io: false.toByte()
    )
    return body(cfg)
}

// swiftlint:enable function_parameter_count

/// Create a temporary array of C-compatible (null-terminated) strings to pass over FFI.
///
/// The strings are deallocated after the closure returns.
///
/// - parameters:
///     * args: The array of strings to use.
//              If `nil` no output array will be allocated and `nil` will be passed to `body`.
///     * body: The closure that gets an array of C-compatible strings
func withArrayOfCStrings<R>(
    _ args: [String]?,
    _ body: ([UnsafePointer<CChar>?]?) -> R
) -> R {
    if let args = args {
        let cStrings = args.map { UnsafePointer(strdup($0)) }
        defer {
            cStrings.forEach { free(UnsafeMutableRawPointer(mutating: $0)) }
        }
        return body(cStrings)
    } else {
        return body(nil)
    }
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

/// Get a timestamp in nanos.
///
/// This is a monotonic clock.
func timestampNanos() -> UInt64 {
    var info = mach_timebase_info()
    guard mach_timebase_info(&info) == KERN_SUCCESS else { return 0 }
    let currentTime = mach_absolute_time()
    let nanos = currentTime * UInt64(info.numer) / UInt64(info.denom)
    return nanos
}

/// Gets a gecko-compatible locale string (e.g. "es-ES")
// If the locale can't be determined on the system, the value is "und",
// to indicate "undetermined".
///
/// - returns: a locale string that supports custom injected locale/languages.
func getLocaleTag() -> String {
    if NSLocale.current.languageCode == nil {
        return "und"
    } else {
        if NSLocale.current.regionCode == nil {
            return NSLocale.current.languageCode!
        } else {
            return "\(NSLocale.current.languageCode!)-\(NSLocale.current.regionCode!)"
        }
    }
}

/// Gather information about the running application
struct AppInfo {
    /// The application's identifier name
    public static var name: String {
        return Bundle.main.bundleIdentifier!
    }

    /// The application's display version string
    public static var displayVersion: String {
        return Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String ?? "Unknown"
    }

    /// The application's build ID
    public static var buildId: String {
        return Bundle.main.infoDictionary?["CFBundleVersion"] as? String ?? "Unknown"
    }
}
