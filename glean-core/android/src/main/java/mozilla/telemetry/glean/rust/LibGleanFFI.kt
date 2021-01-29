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
import mozilla.telemetry.glean.config.FfiConfiguration
import mozilla.telemetry.glean.net.FfiPingUploadTask

// Turn a boolean into its Byte (u8) representation
internal fun Boolean.toByte(): Byte = if (this) 1 else 0

// Turn a Byte into a boolean where zero is false and non-zero is true
internal fun Byte.toBoolean(): Boolean = this != 0.toByte()

/**
 * Result values of attempted ping uploads encoded for FFI use.
 * They are defined in `glean-core/src/upload/result.rs` and re-defined for use in Kotlin here.
 *
 * NOTE:
 * THEY MUST BE THE SAME ACROSS BOTH FILES!
 */
class Constants {
    companion object {
        // A recoverable error.
        val UPLOAD_RESULT_RECOVERABLE: Int = 0x1

        // An unrecoverable error.
        val UPLOAD_RESULT_UNRECOVERABLE: Int = 0x2

        // A HTTP response code.
        val UPLOAD_RESULT_HTTP_STATUS: Int = 0x8000
    }
}

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
            val lib = Native.load(JNA_LIBRARY_NAME, LibGleanFFI::class.java) as LibGleanFFI
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

    // Glean top-level API

    fun glean_initialize(cfg: FfiConfiguration): Byte

    fun glean_flush_rlb_dispatcher()

    fun glean_clear_application_lifetime_metrics()

    fun glean_set_dirty_flag(flag: Byte)

    fun glean_is_dirty_flag_set(): Byte

    fun glean_handle_client_active()

    fun glean_handle_client_inactive()

    fun glean_test_clear_all_stores()

    fun glean_is_first_run(): Byte

    fun glean_destroy_glean()

    fun glean_on_ready_to_submit_pings(): Byte

    fun glean_enable_logging()

    fun glean_set_upload_enabled(flag: Byte)

    fun glean_is_upload_enabled(): Byte

    fun glean_ping_collect(ping_type_handle: Long, reason: String?): Pointer?

    fun glean_submit_ping_by_name(
        ping_name: String,
        reason: String?
    ): Byte

    fun glean_set_experiment_active(
        experiment_id: String,
        branch: String,
        extra_keys: StringArray?,
        extra_values: StringArray?,
        extra_len: Int
    )

    fun glean_set_experiment_inactive(experiment_id: String)

    fun glean_experiment_test_is_active(experiment_id: String): Byte

    fun glean_experiment_test_get_data(experiment_id: String): Pointer?

    // Ping type

    fun glean_new_ping_type(
        name: String,
        include_client_id: Byte,
        send_if_empty: Byte,
        reason_codes: StringArray?,
        reason_codes_len: Int
    ): Long

    fun glean_destroy_ping_type(handle: Long)

    fun glean_register_ping_type(ping_type_id: Long)

    fun glean_test_has_ping_type(name: String): Byte

    // Boolean

    fun glean_new_boolean_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_boolean_metric(handle: Long)

    fun glean_boolean_set(metric_id: Long, value: Byte)

    fun glean_boolean_test_get_value(metric_id: Long, storage_name: String): Byte

    fun glean_boolean_test_has_value(metric_id: Long, storage_name: String): Byte

    // Counter

    fun glean_new_counter_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_counter_metric(handle: Long)

    fun glean_counter_add(metric_id: Long, amount: Int)

    fun glean_counter_test_get_value(metric_id: Long, storage_name: String): Int

    fun glean_counter_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_counter_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Quantity

    fun glean_new_quantity_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_quantity_metric(handle: Long)

    fun glean_quantity_set(metric_id: Long, value: Long)

    fun glean_quantity_test_get_value(metric_id: Long, storage_name: String): Long

    fun glean_quantity_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_quantity_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // String

    fun glean_new_string_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_string_metric(handle: Long)

    fun glean_string_set(metric_id: Long, value: String)

    fun glean_string_test_get_value(metric_id: Long, storage_name: String): Pointer?

    fun glean_string_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_string_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Datetime

    fun glean_new_datetime_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        time_unit: Int
    ): Long

    fun glean_destroy_datetime_metric(handle: Long)

    fun glean_datetime_set(
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

    fun glean_datetime_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_datetime_test_get_value_as_string(metric_id: Long, storage_name: String): Pointer?

    fun glean_datetime_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // String list

    fun glean_new_string_list_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_string_list_metric(handle: Long)

    fun glean_string_list_add(metric_id: Long, value: String)

    fun glean_string_list_set(metric_id: Long, values: StringArray, values_len: Int)

    fun glean_string_list_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_string_list_test_get_value_as_json_string(
        metric_id: Long,
        storage_name: String
    ): Pointer?

    fun glean_string_list_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // UUID

    fun glean_new_uuid_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_uuid_metric(handle: Long)

    fun glean_uuid_set(metric_id: Long, value: String)

    fun glean_uuid_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_uuid_test_get_value(metric_id: Long, storage_name: String): Pointer?

    // Timespan

    fun glean_new_timespan_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        time_unit: Int
    ): Long

    fun glean_destroy_timespan_metric(handle: Long)

    fun glean_timespan_set_start(metric_id: Long, start_time: Long)

    fun glean_timespan_set_stop(metric_id: Long, stop_time: Long)

    fun glean_timespan_cancel(metric_id: Long)

    fun glean_timespan_set_raw_nanos(metric_id: Long, elapsed_nanos: Long)

    fun glean_timespan_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_timespan_test_get_value(metric_id: Long, storage_name: String): Long

    fun glean_timespan_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // TimingDistribution

    fun glean_new_timing_distribution_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        time_unit: Int
    ): Long

    fun glean_destroy_timing_distribution_metric(handle: Long)

    fun glean_timing_distribution_set_start(metric_id: Long, start_time: Long): Long

    fun glean_timing_distribution_set_stop_and_accumulate(
        metric_id: Long,
        timer_id: Long,
        stop_time: Long
    )

    fun glean_timing_distribution_cancel(metric_id: Long, timer_id: Long)

    fun glean_timing_distribution_accumulate_samples(metric_id: Long, samples: LongArray?, len: Int)

    fun glean_timing_distribution_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_timing_distribution_test_get_value_as_json_string(
        metric_id: Long,
        storage_name: String
    ): Pointer?

    fun glean_timing_distribution_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // MemoryDistribution

    fun glean_new_memory_distribution_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        memory_unit: Int
    ): Long

    fun glean_destroy_memory_distribution_metric(handle: Long)

    fun glean_memory_distribution_accumulate(metric_id: Long, sample: Long)

    fun glean_memory_distribution_accumulate_samples(metric_id: Long, samples: LongArray?, len: Int)

    fun glean_memory_distribution_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_memory_distribution_test_get_value_as_json_string(
        metric_id: Long,
        storage_name: String
    ): Pointer?

    fun glean_memory_distribution_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // CustomDistribution

    fun glean_new_custom_distribution_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        range_min: Long,
        range_max: Long,
        bucket_count: Long,
        histogram_type: Int
    ): Long

    fun glean_destroy_custom_distribution_metric(handle: Long)

    fun glean_custom_distribution_accumulate_samples(metric_id: Long, samples: LongArray?, len: Int)

    fun glean_custom_distribution_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_custom_distribution_test_get_value_as_json_string(
        metric_id: Long,
        storage_name: String
    ): Pointer?

    fun glean_custom_distribution_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Event

    fun glean_new_event_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte,
        allowed_extra_keys: StringArray?,
        allowed_extra_keys_len: Int
    ): Long

    fun glean_event_record(
        handle: Long,
        timestamp: Long,
        extra_keys: IntArray?,
        extra_values: StringArray?,
        extra_len: Int
    )

    fun glean_event_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_event_test_get_value_as_json_string(
        handle: Long,
        storage_Name: String
    ): Pointer?

    fun glean_event_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Labeled Counter

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

    fun glean_labeled_counter_metric_get(handle: Long, label: String): Long

    fun glean_labeled_counter_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Labeled Boolean

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

    fun glean_labeled_boolean_metric_get(handle: Long, label: String): Long

    fun glean_labeled_boolean_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // Labeled string

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

    fun glean_labeled_string_metric_get(handle: Long, label: String): Long

    fun glean_labeled_string_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    // JWE

    fun glean_new_jwe_metric(
        category: String,
        name: String,
        send_in_pings: StringArray,
        send_in_pings_len: Int,
        lifetime: Int,
        disabled: Byte
    ): Long

    fun glean_destroy_jwe_metric(handle: Long)

    fun glean_jwe_set_with_compact_representation(metric_id: Long, value: String)

    fun glean_jwe_set(
        metric_id: Long,
        header: String,
        key: String,
        init_vector: String,
        cipher_text: String,
        auth_tag: String
    )

    fun glean_jwe_test_has_value(metric_id: Long, storage_name: String): Byte

    fun glean_jwe_test_get_value(metric_id: Long, storage_name: String): Pointer?

    fun glean_jwe_test_get_value_as_json_string(metric_id: Long, storage_name: String): Pointer?

    fun glean_jwe_test_get_num_recorded_errors(
        metric_id: Long,
        error_type: Int,
        storage_name: String
    ): Int

    fun glean_get_upload_task(task: FfiPingUploadTask.ByReference)

    fun glean_process_ping_upload_response(task: FfiPingUploadTask.ByReference, status: Int)

    fun glean_set_debug_view_tag(value: String): Byte

    fun glean_set_log_pings(value: Byte)

    fun glean_set_source_tags(raw_tags: StringArray, raw_tags_count: Int): Byte

    // Misc

    fun glean_str_free(ptr: Pointer)
}
