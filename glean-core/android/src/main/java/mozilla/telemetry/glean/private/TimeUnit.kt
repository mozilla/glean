/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * Enumeration of different resolutions supported by
 * the Timespan and TimingDistribution metric types.
 */
enum class TimeUnit {
    /**
     * Represents a nanosecond precision.
     */
    Nanosecond,
    /**
     * Represents a microsecond precision.
     */
    Microsecond,
    /**
     * Represents a millisecond precision.
     */
    Millisecond,
    /**
     * Represents a second precision.
     */
    Second,
    /**
     * Represents a minute precision.
     */
    Minute,
    /**
     * Represents a hour precision.
     */
    Hour,
    /**
     * Represents a day precision.
     */
    Day,
}
