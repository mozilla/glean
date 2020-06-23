/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

namespace Mozilla.Glean.Private
{
    /// <summary>
    /// Enumeration of different resolutions supported by
    /// the Timespan and DateTime metric types.
    /// </summary>
    public enum TimeUnit: int
    {
        // Represents nanosecond precision.
        Nanosecond,

        // Represents microsecond precision.
        Microsecond,

        // Represents millisecond precision.
        Millisecond,

        // Represents second precision.
        Second,

        // Represents minute precision.
        Minute,

        // Represents hour precision.
        Hour,

        // Represents day precision.
        Day,
    }
}
