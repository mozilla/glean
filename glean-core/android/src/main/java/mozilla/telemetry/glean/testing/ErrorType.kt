/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.testing

/**
 * Different types of errors that can be reported through Glean's error reporting metrics.
 */
enum class ErrorType {
    /**
     * For when the value to be recorded does not match the metric-specific restrictions
     */
    InvalidValue,

    /**
     * For when the label of a labeled metric does not match the restrictions
     */
    InvalidLabel,

    /**
     * For when timings are recorded incorrectly
     */
    InvalidState,

    /**
     * For when the value to be recorded overflows the metric-specific upper range
     */
    InvalidOverflow
}
