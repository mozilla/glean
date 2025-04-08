/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.internal.LabeledBoolean
import mozilla.telemetry.glean.internal.LabeledCounter
import mozilla.telemetry.glean.internal.LabeledString
import mozilla.telemetry.glean.testing.ErrorType

/**
 * This implements the developer facing API for labeled metrics.
 *
 * Instances of this class type are automatically generated by the parsers at build time,
 * allowing developers to record values that were previously registered in the metrics.yaml file.
 *
 * Unlike most metric types, LabeledMetricType does not have its own corresponding storage,
 * but records metrics for the underlying metric type T in the storage for that type.  The
 * only difference is that labeled metrics are stored with the special key
 * `$category.$name/$label`. The collect method knows how to pull these special values back
 * out of the individual metric storage and rearrange them correctly in the ping.
 */
@Suppress("LongParameterList")
class LabeledMetricType<T>(
    private val disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    private val labels: Set<String>? = null,
    private val sendInPings: List<String>,
    private val subMetric: T,
) {
    // The inner labeled metric, from which actual metrics are constructed.
    private val inner: Any

    init {
        val meta = CommonLabeledMetricData(
            cmd = CommonMetricData(
                category = category,
                name = name,
                sendInPings = sendInPings,
                disabled = disabled,
                lifetime = lifetime,
            ),
        )

        this.inner = when (subMetric) {
            is CounterMetricType -> LabeledCounter(meta, labels?.toList())
            is BooleanMetricType -> LabeledBoolean(meta, labels?.toList())
            is StringMetricType -> LabeledString(meta, labels?.toList())
            else -> error("Can not create a labeled version of this metric type")
        }
    }

    /**
     * Get the specific metric for a given label.
     *
     * If a set of acceptable labels were specified in the metrics.yaml file,
     * and the given label is not in the set, it will be recorded under the
     * special `__other__`.
     *
     * If a set of acceptable labels was not specified in the metrics.yaml file,
     * only the first 16 unique labels will be used. After that, any additional
     * labels will be recorded under the special `__other__` label.
     *
     * Labels must have a maximum of 71 characters, and may comprise any printable ASCII characters.
     * is used, the metric will be recorded in the special `__other__` label.
     *
     * @param label The label
     * @return The specific metric for that label
     */
    @Suppress("UNCHECKED_CAST")
    operator fun get(label: String): T {
        return when (this.inner) {
            is LabeledCounter -> this.inner.get(label) as T
            is LabeledBoolean -> this.inner.get(label) as T
            is LabeledString -> this.inner.get(label) as T
            else -> error("Can not create a labeled version of this metric type")
        }
    }

    /**
     * Get the specific metric for a given label index.
     *
     * This only works if a set of acceptable labels were specified in the
     * metrics.yaml file. If static labels were not defined in that file or
     * the index of the given label is not in the set, it will be recorded under
     * the special `__other__`.
     *
     * @param labelIndex The label
     * @return The specific metric for that label
     */
    operator fun get(labelIndex: Int): T {
        val actualLabel = if (labels != null && labelIndex < labels.size) {
            labels.elementAt(labelIndex)
        } else {
            "__other__"
        }

        return this[actualLabel]
    }

    /**
     * Returns the number of errors recorded for the given metric.
     *
     * @param error The type of the error recorded.
     * @return the number of errors recorded for the metric.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetNumRecordedErrors(errorType: ErrorType): Int {
        return when (this.inner) {
            is LabeledCounter -> this.inner.testGetNumRecordedErrors(errorType)
            is LabeledBoolean -> this.inner.testGetNumRecordedErrors(errorType)
            is LabeledString -> this.inner.testGetNumRecordedErrors(errorType)
            else -> error("Can not create a labeled version of this metric type")
        }
    }
}
