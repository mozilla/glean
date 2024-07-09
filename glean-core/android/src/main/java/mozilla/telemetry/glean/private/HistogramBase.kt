/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * A common interface to be implemented by all the histogram-like metric types
 * supported by the Glean SDK.
 */
interface HistogramBase {
    /**
     * Accumulates the provided samples in the metric.
     *
     * Please note that this assumes that the provided samples are already in the
     * "unit" declared by the instance of the implementing metric type (e.g. if the
     * implementing class is a [TimingDistributionMetricType] and the instance this
     * method was called on is using [TimeUnit.SECOND], then `samples` are assumed
     * to be in that unit).
     *
     * @param samples the [List<Long>] holding the samples to be recorded by the metric.
     */
    fun accumulateSamples(samples: List<Long>)
}

// glean_parser template currently expects `HistogramMetricBase` as the name
// and since this alias was defined in `service-glean` in android-components,
// we need to keep the alias until the parser template is updated also.
// See Bug 1906941 for more information.
typealias HistogramMetricBase = mozilla.telemetry.glean.private.HistogramBase
