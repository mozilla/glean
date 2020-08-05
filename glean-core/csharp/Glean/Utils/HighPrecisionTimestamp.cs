// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using System;
using System.Diagnostics;

namespace Mozilla.Glean.Utils
{
    /// <summary>
    /// Wraps an high precision timer.
    /// </summary>
    internal static class HighPrecisionTimestamp
    {
        // The number of nanoseconds within a second, used to convert ticks
        // to nanoseconds.
        private const long NanosecondsInSeconds = 1000000000;

        // The number of milliseconds within a second, used to convert ticks
        // to milliseconds.
        private const long MillisecondsInSeconds = 1000;

        /// <summary>
        /// *** ONLY FOR TESTS ***
        /// 
        /// When not `null` this sets the value that will be returned
        /// by `GetTimestamp`.
        /// </summary>
        internal static ulong? MockedValue { get; set; }

        private static ulong GetTimeFromTicks(long ticks, long unitsInSeconds)
        {
            // The computation below is a bit tricky: make sure all the divisions happen in double, then
            // cast it back to ulong to avoid overflows.
            return (ulong)(ticks / (1.0 * Stopwatch.Frequency) * unitsInSeconds);
        }

        /// <summary>
        /// Get the current timestamp in the requested time unit.
        /// 
        /// Note that only `TimeUnit.Nanosecond` and `TimeUnit.Millisecond` are
        /// supported. This will throw an exception on other units.
        /// </summary>
        /// <param name="unit">
        /// Either `TimeUnit.Nanosecond` or `TimeUnit.Millisecond` to indicate which
        /// unit the timestamp should be converted to.
        /// </param>
        /// <exception cref="ArgumentOutOfRangeException">
        /// If some value other than `TimeUnit.Nanosecond` or `TimeUnit.Millisecond` was
        /// provided as input.
        /// </exception>
        /// <returns>The timestamp in the desired time unit</returns>
        public static ulong GetTimestamp(TimeUnit unit)
        {
            // This should only be set in tests.
            if (MockedValue != null)
            {
                return MockedValue.Value;
            }

            // The `Stopwatch` class tries to do almost the same thing we're doing! Unfortunately
            // we only want to use its ability to provide us a monotonic timer. At least on recent
            // Windows this is guaranteed to be monotonic due to the usage of the Windows Performance
            // timers under the hood. See
            // https://docs.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps
            // Note that this function will return "ticks", not milliseconds, so we need to convert that.
            long ticks = Stopwatch.GetTimestamp();

            if (unit == TimeUnit.Nanosecond)
            {
                return GetTimeFromTicks(ticks, NanosecondsInSeconds);
            } else if (unit == TimeUnit.Millisecond)
            {
                return GetTimeFromTicks(ticks, MillisecondsInSeconds);
            }

            // We were passed an unsupported unit.
            throw new ArgumentOutOfRangeException("Unexpected time unit for GetTimestamp");
        }
    }
}
