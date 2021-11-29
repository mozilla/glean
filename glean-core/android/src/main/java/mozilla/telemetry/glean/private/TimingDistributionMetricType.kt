/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.os.SystemClock
import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.internal.TimingDistributionMetric
import mozilla.telemetry.glean.testing.ErrorType

/**
 * This implements the developer facing API for recording timing distribution metrics.
 *
 * Instances of this class type are automatically generated by the parsers at build time,
 * allowing developers to record values that were previously registered in the metrics.yaml file.
 */
class TimingDistributionMetricType(meta: CommonMetricData, timeUnit: TimeUnit) : HistogramBase {
    val inner = TimingDistributionMetric(meta, timeUnit)

    /**
     * Delegate common methods to the underlying type directly.
     */

    fun start() = inner.start()
    fun stopAndAccumulate(timerId: ULong) = inner.stopAndAccumulate(timerId)
    fun cancel(timerId: ULong) = inner.cancel(timerId)

    /**
     * Additional functionality
     */

    override fun accumulateSamples(samples: List<Long>) = inner.accumulateSamples(samples)

    /**
     * Convenience method to simplify measuring a function or block of code.
     *
     * If the measured function throws, the measurement is canceled and the exception rethrown.
     */
    @Suppress("TooGenericExceptionCaught")
    fun <U> measure(funcToMeasure: () -> U): U {
        val timerId = this.start()

        val returnValue = try {
            funcToMeasure()
        } catch (e: Exception) {
            this.cancel(timerId)
            throw e
        }

        this.stopAndAccumulate(timerId)
        return returnValue
    }

    /**
     * Testing-only methods get an annotation
     */

    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    @JvmOverloads
    fun testGetValue(pingName: String? = null) = inner.testGetValue(pingName)

    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    @JvmOverloads
    fun testGetNumRecordedErrors(error: ErrorType, pingName: String? = null) = inner.testGetNumRecordedErrors(error, pingName)

    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    @JvmOverloads
    fun testHasValue(pingName: String? = null): Boolean {
        return this.testGetValue(pingName) != null
    }
}
