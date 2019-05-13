/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting

class StringListMetricType(
    disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    val sendInPings: List<String>
) {

    /**
     * Appends a string value to one or more string list metric stores.  If the string exceeds the
     * maximum string length or if the list exceeds the maximum length it will be truncated.
     *
     * @param value This is a user defined string value. The maximum length of
     *              this string is [MAX_STRING_LENGTH].
     */
    fun add(value: String) {
        // if (!shouldRecord(logger)) {
        //     return
        // }

        // @Suppress("EXPERIMENTAL_API_USAGE")
        // Dispatchers.API.launch {
        //     // Delegate storing the string to the storage engine.
        //     StringListsStorageEngine.add(
        //         metricData = this@StringListMetricType,
        //         value = value
        //     )
        // }
        // TODO: stub
    }

    /**
     * Sets a string list to one or more metric stores. If any string exceeds the maximum string
     * length or if the list exceeds the maximum length it will be truncated.
     *
     * @param value This is a user defined string list.
     */
    fun set(value: List<String>) {
        // if (!shouldRecord(logger)) {
        //     return
        // }

        // @Suppress("EXPERIMENTAL_API_USAGE")
        // Dispatchers.API.launch {
        //     // Delegate storing the string list to the storage engine.
        //     StringListsStorageEngine.set(
        //         metricData = this@StringListMetricType,
        //         value = value
        //     )
        // }
        // TODO: stub
    }

    /**
     * Tests whether a value is stored for the metric for testing purposes only. This function will
     * attempt to await the last task (if any) writing to the the metric's storage engine before
     * returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return true if metric value exists, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testHasValue(pingName: String = sendInPings.first()): Boolean {
        // @Suppress("EXPERIMENTAL_API_USAGE")
        // Dispatchers.API.assertInTestingMode()

        // return StringListsStorageEngine.getSnapshot(pingName, false)?.get(identifier) != null
        // TODO: stub
        return false
    }

    /**
     * Returns the stored value for testing purposes only. This function will attempt to await the
     * last task (if any) writing to the the metric's storage engine before returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return value of the stored metric
     * @throws [NullPointerException] if no value is stored
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetValue(pingName: String = sendInPings.first()): List<String> {
        // @Suppress("EXPERIMENTAL_API_USAGE")
        // Dispatchers.API.assertInTestingMode()

        // return StringListsStorageEngine.getSnapshot(pingName, false)!![identifier]!!
        return listOf("FOO", "BAR")
    }
}
