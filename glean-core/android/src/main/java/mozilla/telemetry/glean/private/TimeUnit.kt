/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * Enumeration of different resolutions supported by
 * the Timespan and DateTime metric types.
 */
enum class TimeUnit {
    /**
     * Represents nanosecond precision.
     */
    Nanosecond,
    /**
     * Represents microsecond precision.
     */
    Microsecond,
    /**
     * Represents millisecond precision.
     */
    Millisecond,
    /**
     * Represents second precision.
     */
    Second,
    /**
     * Represents minute precision.
     */
    Minute,
    /**
     * Represents hour precision.
     */
    Hour,
    /**
     * Represents day precision.
     */
    Day,
}
