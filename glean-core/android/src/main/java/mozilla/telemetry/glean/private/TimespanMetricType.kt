/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * This implements the developer facing API for recording timespans.
 *
 * Instances of this class type are automatically generated by the parsers at build time,
 * allowing developers to record values that were previously registered in the metrics.yaml file.
 *
 * The timespans API exposes the [start], [stop] and [cancel] methods.
 */
typealias TimespanMetricType = mozilla.telemetry.glean.internal.TimespanMetric

/**
 * Convenience method to simplify measuring a function or block of code
 *
 * If the measured function throws, the measurement is canceled and the exception rethrown.
 */
@Suppress("TooGenericExceptionCaught")
fun <U> TimespanMetricType.measure(funcToMeasure: () -> U): U {
    this.start()

    val returnValue = try {
        funcToMeasure()
    } catch (e: Exception) {
        this.cancel()
        throw e
    }

    this.stop()
    return returnValue
}
