/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import mozilla.components.support.ktx.android.org.json.toList
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.getAndConsumeRustString
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.RustError
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import org.json.JSONArray

class StringListMetricType(
    private var handle: Long,
    private val sendInPings: List<String>
) {
    /**
     * The public constructor used by automatically-generated metrics.
     */
    constructor(
        disabled: Boolean,
        category: String,
        lifetime: Lifetime,
        name: String,
        sendInPings: List<String>
    ) : this(handle = 0, sendInPings = sendInPings) {
        val ffiPingsList = StringArray(sendInPings.toTypedArray(), "utf-8")
        this.handle = LibGleanFFI.INSTANCE.glean_new_string_list_metric(
            category = category,
            name = name,
            send_in_pings = ffiPingsList,
            send_in_pings_len = sendInPings.size,
            lifetime = lifetime.ordinal,
            disabled = disabled.toByte())
    }

    protected fun finalize() {
        if (this.handle != 0L) {
            val error = RustError.ByReference()
            LibGleanFFI.INSTANCE.glean_destroy_string_list_metric(this.handle, error)
        }
        // Do nothing with the error, for now.
        // It is expected to only ever error if this.handle's invalid.
    }

    private fun shouldRecord(): Boolean {
        // Don't record metrics if Glean isn't initialized.
        if (!Glean.isInitialized()) {
            return false
        }

        return LibGleanFFI.INSTANCE.glean_string_list_should_record(Glean.handle, this.handle).toBoolean()
    }

    /**
     * Appends a string value to one or more string list metric stores.  If the string exceeds the
     * maximum string length or if the list exceeds the maximum length it will be truncated.
     *
     * @param value This is a user defined string value. The maximum length of
     *              this string is [MAX_STRING_LENGTH].
     */
    fun add(value: String) {
        if (!shouldRecord()) {
            return
        }

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_string_list_add(
                Glean.handle,
                this@StringListMetricType.handle,
                value)
        }
    }

    /**
     * Sets a string list to one or more metric stores. If any string exceeds the maximum string
     * length or if the list exceeds the maximum length it will be truncated.
     *
     * @param value This is a user defined string list.
     */
    fun set(value: List<String>) {
        if (!shouldRecord()) {
            return
        }

        val ffiValueList = StringArray(value.toTypedArray(), "utf-8")
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_string_list_set(
                Glean.handle,
                this@StringListMetricType.handle,
                ffiValueList,
                value.size)
        }
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
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        val res = LibGleanFFI.INSTANCE.glean_string_list_test_has_value(Glean.handle, this.handle, pingName)
        return res.toBoolean()
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
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        if (!testHasValue(pingName)) {
            throw NullPointerException()
        }

        val jsonRes: JSONArray
        val ptr = LibGleanFFI.INSTANCE.glean_string_list_test_get_value_as_json_string(
            Glean.handle,
            this.handle,
            pingName)!!
        try {
            jsonRes = JSONArray(ptr.getAndConsumeRustString())
        } catch (e: org.json.JSONException) {
            throw NullPointerException()
        }
        return jsonRes.toList<String>()
    }
}
