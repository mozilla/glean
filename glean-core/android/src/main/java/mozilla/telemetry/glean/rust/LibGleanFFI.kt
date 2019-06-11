/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@file:Suppress("FunctionNaming", "FunctionParameterNaming", "LongParameterList")

package mozilla.telemetry.glean.rust

import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.StringArray
import java.lang.reflect.Proxy

// Turn a boolean into its Byte (u8) representation
internal fun Boolean.toByte(): Byte = if (this) 1 else 0

// Turn a Byte into a boolean where zero is false and non-zero is true
internal fun Byte.toBoolean(): Boolean = this != 0.toByte()

/**
 * Helper to read a null terminated String out of the Pointer and free it.
 *
 * Important: Do not use this pointer after this! For anything!
 */
internal fun Pointer.getAndConsumeRustString(): String {
    try {
        return this.getRustString()
    } finally {
        LibGleanFFI.INSTANCE.glean_str_free(this)
    }
}

/**
 * Helper to read a null terminated string out of the pointer.
 *
 * Important: doesn't free the pointer, use [getAndConsumeRustString] for that!
 */
internal fun Pointer.getRustString(): String {
    return this.getString(0, "utf8")
}

@Suppress("TooManyFunctions")
internal interface LibGleanFFI : Library {
    companion object {
        private val JNA_LIBRARY_NAME = "glean_ffi"

        internal var INSTANCE: LibGleanFFI = try {
            val lib = Native.loadLibrary(JNA_LIBRARY_NAME, LibGleanFFI::class.java) as LibGleanFFI
            lib.glean_enable_logging()
            lib
        } catch (e: UnsatisfiedLinkError) {
            Proxy.newProxyInstance(
                LibGleanFFI::class.java.classLoader,
                arrayOf(LibGleanFFI::class.java)
            ) { _, _, _ ->
                throw IllegalStateException("Glean functionality not available", e)
            } as LibGleanFFI
        }
    }

    // Important: strings returned from rust as *mut char must be Pointers on this end, returning a
    // String will work but either force us to leak them, or cause us to corrupt the heap (when we
    // free them).

    fun glean_enable_logging()

    fun glean_boolean_set(glean_handle: Long, metric_id: Long, value: Byte)

    fun glean_boolean_should_record(glean_handle: Long, metric_id: Long): Byte

    fun glean_boolean_test_get_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_boolean_test_has_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_counter_add(glean_handle: Long, metric_id: Long, amount: Int)

    fun glean_counter_should_record(glean_handle: Long, metric_id: Long): Byte

    fun glean_counter_test_get_value(glean_handle: Long, metric_id: Long, storage_name: String): Int

    fun glean_counter_test_has_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_datetime_set(
        glean_handle: Long,
        metric_id: Long,
        year: Int,
        month: Int,
        day: Int,
        hour: Int,
        minute: Int,
        second: Int,
        nano: Long,
        offset_seconds: Int
    )

    fun glean_datetime_should_record(glean_handle: Long, metric_id: Long): Byte

    fun glean_datetime_test_has_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_datetime_test_get_value_as_string(glean_handle: Long, metric_id: Long, storage_name: String): Pointer?

    fun glean_new_labeled_counter_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        labels: StringArray?,
        label_count: Int
    ): Long

    fun glean_labeled_counter_metric_get(glean_handle: Long, handle: Long, label: String): Long

    fun glean_new_labeled_boolean_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        labels: StringArray?,
        label_count: Int
    ): Long

    fun glean_labeled_boolean_metric_get(glean_handle: Long, handle: Long, label: String): Long

    fun glean_new_labeled_string_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        labels: StringArray?,
        label_count: Int
    ): Long

    fun glean_labeled_string_metric_get(glean_handle: Long, handle: Long, label: String): Long

    fun glean_initialize(data_dir: String, application_id: String, upload_enabled: Byte): Long

    fun glean_is_upload_enabled(glean_handle: Long): Byte

    fun glean_new_boolean_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_new_counter_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_new_datetime_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        time_unit: Int
    ): Long

    fun glean_new_string_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_new_string_list_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_string_test_get_value(glean_handle: Long, metric_id: Long, storage_name: String): Pointer?

    fun glean_string_test_has_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_ping_collect(glean_handle: Long, ping_type_handle: Long): Pointer?

    fun glean_send_ping(glean_handle: Long, ping_type_handle: Long, log_ping: Byte): Byte

    fun glean_send_ping_by_name(glean_handle: Long, ping_name: String, log_ping: Byte): Byte

    fun glean_set_upload_enabled(glean_handle: Long, flag: Byte)

    fun glean_string_set(glean_handle: Long, metric_id: Long, value: String)

    fun glean_string_should_record(glean_handle: Long, metric_id: Long): Byte

    fun glean_string_list_should_record(glean_handle: Long, metric_id: Long): Byte

    fun glean_string_list_add(glean_handle: Long, metric_id: Long, value: String)

    fun glean_string_list_set(glean_handle: Long, metric_id: Long, values: StringArray, values_len: Int)

    fun glean_string_list_test_has_value(glean_handle: Long, metric_id: Long, storage_name: String): Byte

    fun glean_string_list_test_get_value_as_json_string(glean_handle: Long, metric_id: Long, storage_name: String): Pointer?

    fun glean_destroy_glean(handle: Long, error: RustError.ByReference)

    fun glean_destroy_boolean_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_string_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_string_list_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_counter_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_datetime_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_ping_type(handle: Long, error: RustError.ByReference)

    fun glean_str_free(ptr: Pointer)

    fun glean_register_ping_type(glean_handle: Long, ping_type_id: Long)

    fun glean_new_ping_type(name: String, include_client_id: Byte): Long

    fun glean_test_has_ping_type(glean_handle: Long, name: String): Byte
}

internal typealias MetricHandle = Long
