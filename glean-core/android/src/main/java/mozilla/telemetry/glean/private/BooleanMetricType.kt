/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError

class BooleanMetricType(
        disabled: Boolean,
        category: String,
        lifetime: Lifetime,
        name: String,
        val sendInPings: List<String>)
{
    private var handle: Long

    init {
        println("New Boolean: $category.$name")

        val e = RustError.ByReference()
        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_boolean_metric(
                category = category,
                name = name,
                send_in_pings = ffiPingsList,
                send_in_pings_len = sendInPings.size,
                lifetime = lifetime.ordinal,
                err = e)
    }

    /**
     * Set a boolean value.
     *
     * @param value This is a user defined boolean value.
     */
    fun set(value: Boolean) {
        val e = RustError.ByReference()
        LibGleanFFI.INSTANCE.glean_boolean_set(Glean.handle, this.handle, if (value) { 1 } else { 0 }, e)
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
        // TODO
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
    fun testGetValue(pingName: String = sendInPings.first()): Boolean {
        // TODO
        return false
    }
}
