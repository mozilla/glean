/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

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
}
