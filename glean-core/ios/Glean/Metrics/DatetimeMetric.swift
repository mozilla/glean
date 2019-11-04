/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This implements the developer facing API for recording datetime metrics.
///
/// Instances of this class type are automatically generated by the parsers at build time,
/// allowing developers to record values that were previously registered in the metrics.yaml file.
///
/// The datetime API only exposes the `DatetimeMetricType.set(_:)` method, which takes care of validating the input
/// data and making sure that limits are enforced.
public class DatetimeMetricType {
    let handle: UInt64
    let disabled: Bool
    let sendInPings: [String]
    let timeUnit: TimeUnit

    /// The public constructor used by automatically generated metrics.
    public init(
        category: String,
        name: String,
        sendInPings: [String],
        lifetime: Lifetime,
        disabled: Bool,
        timeUnit: TimeUnit = .minute
    ) {
        self.disabled = disabled
        self.sendInPings = sendInPings
        self.timeUnit = timeUnit
        self.handle = withArrayOfCStrings(sendInPings) { pingArray in
            glean_new_datetime_metric(
                category,
                name,
                pingArray,
                Int32(sendInPings.count),
                lifetime.rawValue,
                disabled.toByte(),
                timeUnit.rawValue
            )
        }
    }

    /// Destroy this metric.
    deinit {
        if self.handle != 0 {
            glean_destroy_datetime_metric(self.handle)
        }
    }

    /// Set a datetime value, truncating it to the metric's resolution.
    ///
    /// - parameters:
    ///      * value: The [Date] value to set.  If not provided, will record the current time.
    public func set(_ value: Date = Date()) {
        let calendar = Calendar.current
        let components = calendar.dateComponents(in: TimeZone.current, from: value)
        set(components: components)
    }

    /// Set a datetime value, truncating it to the metric's resolution.
    ///
    /// This is provided as an internal-only function for convenience and so that we can test that timezones
    /// are passed through correctly.  The normal public interface uses `Date` objects which are in the local
    /// timezone.
    ///
    /// - parameters:
    ///     * components: The [DateComponents] value to set.
    func set(components: DateComponents) {
        guard !self.disabled else { return }

        Dispatchers.shared.launchAPI {
            glean_datetime_set(
                Glean.shared.handle,
                self.handle,
                Int32(components.year ?? 0),
                UInt32(components.month ?? 0),
                UInt32(components.day ?? 0),
                UInt32(components.hour ?? 0),
                UInt32(components.minute ?? 0),
                UInt32(components.second ?? 0),
                Int64(components.nanosecond ?? 0),
                Int32(components.timeZone!.secondsFromGMT(for: components.date!))
            )
        }
    }

    /// Tests whether a value is stored for the metric for testing purposes only. This function will
    /// attempt to await the last task (if any) writing to the the metric's storage engine before
    /// returning a value.
    ///
    /// - parameters:
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    /// - returns: true if metric value exists, otherwise false
    func testHasValue(_ pingName: String? = nil) -> Bool {
        Dispatchers.shared.assertInTestingMode()

        let pingName = pingName ?? self.sendInPings[0]
        return glean_datetime_test_has_value(Glean.shared.handle, self.handle, pingName) != 0
    }

    /// Returns the string representation of the stored value for testing purposes only. This function
    /// will attempt to await the last task (if any) writing to the the metric's storage engine before returning
    ///  a value.
    ///
    /// Throws a `Missing value` exception if no value is stored
    ///
    /// - parameters:
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    ///
    /// - returns:  value of the stored metric
    func testGetValueAsString(_ pingName: String? = nil) throws -> String {
        Dispatchers.shared.assertInTestingMode()

        let pingName = pingName ?? self.sendInPings[0]

        if !testHasValue(pingName) {
            throw "Missing value"
        }

        return String(
            freeingRustString: glean_datetime_test_get_value_as_string(
                Glean.shared.handle,
                self.handle,
                pingName
            )
        )
    }

    /// Returns the stored value for testing purposes only. This function will attempt to await the
    /// last task (if any) writing to the the metric's storage engine before returning a value.
    ///
    /// Throws a `Missing value` exception if no value is stored
    ///
    /// - parameters:
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    ///
    /// - returns:  value of the stored metric
    func testGetValue(_ pingName: String? = nil) throws -> Date {
        return Date.fromISO8601String(
            dateString: try testGetValueAsString(pingName),
            precision: timeUnit
        )!
    }

    /// Returns the number of errors recorded for the given metric.
    ///
    /// - parameters:
    ///     * errorType: The type of error recorded.
    ///     * pingName: represents the name of the ping to retrieve the metric for.
    ///                 Defaults to the first value in `sendInPings`.
    ///
    /// - returns: The number of errors recorded for the metric for the given error type.
    func testGetNumRecordedErrors(_ errorType: ErrorType, pingName: String? = nil) -> Int32 {
        Dispatchers.shared.assertInTestingMode()

        let pingName = pingName ?? self.sendInPings[0]

        return glean_datetime_test_get_num_recorded_errors(
            Glean.shared.handle,
            self.handle,
            errorType.rawValue,
            pingName
        )
    }
}
