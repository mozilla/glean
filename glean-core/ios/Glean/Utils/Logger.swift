/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// This is a simple Logger class to unify and simplify log messaging
/// across the application
class Logger {
    // This holds the log tag for reuse by the instance
    private let tag: String

    // Represents the different supported log levels and the raw value
    // that is used as a start character for the log message in order
    // to be consistent with the Android log output.
    enum LogLevel: String {
        case debug = "D"
        case info = "I"
        case warn = "W"
        case error = "E"
    }

    /// Creates a new logger instance with the specified tag value
    ///
    /// - parameters:
    ///     * tag: `String` value used to tag log messages
    init(tag: String) {
        self.tag = tag
    }

    /// Output a debug log message
    ///
    /// - parameters:
    ///     * message: The message to log
    func debug(_ message: String) {
        log(message: message, level: .debug)
    }

    /// Output an info log message
    ///
    /// - parameters:
    ///     * message: The message to log
    func info(_ message: String) {
        log(message: message, level: .info)
    }

    /// Output a warning log message
    ///
    /// - parameters:
    ///     * message: The message to log
    func warn(_ message: String) {
        log(message: message, level: .warn)
    }

    /// Output an error log message
    ///
    /// - parameters:
    ///     * message: The message to log
    func error(_ message: String) {
        log(message: message, level: .error)
    }

    /// Private function that formats and outputs the log message based on the level
    ///
    /// - parameters:
    ///     * message: The message to log
    ///     * level: The `LogLevel` at which to output the message
    private func log(message: String, level: LogLevel) {
        let formattedMessage = "\(level.rawValue)/\(tag): \(message)"

        // Since logging via `NSLog` has overhead, we only want to output via `NSLog`
        // for critical messages such as errors.  Everything else can be outputted
        // using `print()` so that it does not impact the log on release builds.
        switch level {
        case .error:
            NSLog(formattedMessage)
        default:
            print(formattedMessage)
        }
    }
}
