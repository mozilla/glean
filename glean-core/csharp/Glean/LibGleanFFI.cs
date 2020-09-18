// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Mozilla.Glean.FFI
{
    /// <summary>
    /// Result values of attempted ping uploads encoded for FFI use.
    /// They are defined in `glean-core/src/upload/result.rs` and re-defined for use in Kotlin here.
    /// 
    /// NOTE:
    /// THEY MUST BE THE SAME ACROSS BOTH FILES!
    /// </summary>
    internal enum Constants : int
    {
        // A recoverable error.
        UPLOAD_RESULT_RECOVERABLE = 0x1,

        // An unrecoverable error.
        UPLOAD_RESULT_UNRECOVERABLE = 0x2,

        // A HTTP response code.
        UPLOAD_RESULT_HTTP_STATUS = 0x8000
    }

    /// <summary>
    /// Rust represents the upload task as an Enum
    /// and to go through the FFI that gets transformed into a tagged union.
    /// Each variant is represented as an 8-bit unsigned integer.
    ///
    /// This *MUST* have the same order as the variants in `glean-core/ffi/src/upload.rs`.
    /// </summary>
    enum UploadTaskTag : int
    {
        Upload,
        Wait,
        Done
    }

    [StructLayout(LayoutKind.Sequential)]
    internal struct FfiUploadTaskBody
    {
        public byte tag;
        public IntPtr documentId;
        public IntPtr path;
        public int bodyLen;
        public IntPtr body;
        public IntPtr headers;
    }

    /// <summary>
    /// Represent an upload task by simulating the union passed through
    /// the FFI layer.
    /// </summary>
    [StructLayout(LayoutKind.Explicit)]
    internal struct FfiUploadTask
    {
        [FieldOffset(0)]
        public byte tag;
        [FieldOffset(0), MarshalAs(UnmanagedType.Struct)]
        public FfiUploadTaskBody body;
    }

    internal static class LibGleanFFI
    {
        private const string SharedGleanLibrary = "glean_ffi";

        // Define the order of fields as laid out in memory.
        // **CAUTION**: This must match _exactly_ the definition on the Rust side.
        //  If this side is changed, the Rust side need to be changed, too.
        [StructLayout(LayoutKind.Sequential)]
        internal class FfiConfiguration
        {
            public string data_dir;
            public string package_name;
            public string language_binding_name;
            public bool upload_enabled;
            public IntPtr max_events;
            public bool delay_ping_lifetime_io;
        }

        public static string GetFromRustString(IntPtr pointer)
        {
            int len = 0;
            while (Marshal.ReadByte(pointer, len) != 0) { ++len; }
            byte[] buffer = new byte[len];
            Marshal.Copy(pointer, buffer, 0, buffer.Length);
            return Encoding.UTF8.GetString(buffer);
        }

        internal class StringAsReturnValue : SafeHandle
        {
            public StringAsReturnValue() : base(IntPtr.Zero, true) { }

            public override bool IsInvalid
            {
                get { return this.handle == IntPtr.Zero; }
            }

            public string AsString()
            {
                return GetFromRustString(handle);
            }

            protected override bool ReleaseHandle()
            {
                if (!this.IsInvalid)
                {
                    glean_str_free(handle);
                }

                return true;
            }
        }

        // Glean top-level API.

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_initialize(IntPtr cfg);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_clear_application_lifetime_metrics();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_set_dirty_flag(byte flag);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_is_dirty_flag_set();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_test_clear_all_stores();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_is_first_run();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_glean();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_on_ready_to_submit_pings();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_enable_logging();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_set_upload_enabled(bool flag);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_is_upload_enabled();

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_ping_collect(UInt64 ping_type_handle, string reason);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_submit_ping_by_name(string ping_name, string reason);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_set_experiment_active(
            string experiment_id,
            string branch,
            string[] extra_keys,
            string[] extra_values,
            Int32 extra_len
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_set_experiment_inactive(string experiment_id);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_experiment_test_is_active(string experiment_id);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_experiment_test_get_data(string experiment_id);

        // String

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_string_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_string_metric(UInt64 handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_string_set(UInt64 metric_id, string value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_string_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_string_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_string_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // Boolean

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_boolean_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_boolean_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_boolean_set(UInt64 metric_id, byte value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_boolean_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_boolean_test_has_value(UInt64 metric_id, string storage_name);


        // Counter

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_counter_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_counter_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_counter_add(UInt64 metric_id, Int32 amount);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_counter_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern bool glean_counter_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_counter_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            String storage_name
        );

        // Uuid

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_uuid_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_uuid_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_uuid_set(UInt64 metric_id, string value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_uuid_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_uuid_test_has_value(UInt64 metric_id, string storage_name);

        // Timespan

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_timespan_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            Int32 time_unit
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_timespan_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timespan_set_start(UInt64 handle, UInt64 start_time);


        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timespan_set_stop(UInt64 metric_id, UInt64 stop_time);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timespan_cancel(UInt64 metric_id);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timespan_set_raw_nanos(UInt64 metric_id, UInt64 elapsed_nanos);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_timespan_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_timespan_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_timespan_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            String storage_name
        );

        // TimingDistribution

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_timing_distribution_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            Int32 time_unit
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_timing_distribution_metric(UInt64 handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_timing_distribution_set_start(UInt64 metric_id, UInt64 start_time);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timing_distribution_set_stop_and_accumulate(
            UInt64 metric_id,
            UInt64 timer_id,
            UInt64 stop_time
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timing_distribution_cancel(UInt64 metric_id, UInt64 timer_id);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_timing_distribution_accumulate_samples(UInt64 metric_id, Int64[] samples, Int32 len);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_timing_distribution_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_timing_distribution_test_get_value_as_json_string(
            UInt64 metric_id,
            string storage_name
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_timing_distribution_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            string storage_name
        );

        // MemoryDistribution

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_memory_distribution_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            Int32 memory_unit
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_memory_distribution_metric(UInt64 handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_memory_distribution_accumulate(UInt64 metric_id, UInt64 sample);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_memory_distribution_accumulate_samples(UInt64 metric_id, Int64[] samples, Int32 len);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_memory_distribution_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_memory_distribution_test_get_value_as_json_string(
            UInt64 metric_id,
            string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_memory_distribution_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            string storage_name
         );

        // Datetime

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_datetime_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            Int32 time_unit
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_datetime_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_datetime_set(UInt64 metric_id, Int32 year,
            Int32 month, Int32 day, Int32 hour, Int32 minute, Int32 second, long nano, Int32 offset_seconds);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_datetime_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_datetime_test_get_value_as_string(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_datetime_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // Event

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_event_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            string[] allowed_extra_keys,
            Int32 allowed_extra_keys_len
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_event_record(
            UInt64 handle,
            UInt64 timestamp,
            Int32[] extra_keys,
            string[] extra_values,
            Int32 extra_len
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_event_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_event_test_get_value_as_json_string(
            UInt64 handle,
            string storage_Name
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_event_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            string storage_name
        );

        // Labeled Counter

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_labeled_counter_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            string[] labels,
            Int32 label_count
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_labeled_counter_metric_get(UInt64 handle, string label);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_labeled_counter_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // Labeled Boolean

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_labeled_boolean_metric(
            string category,
            string name,
            string [] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            string [] labels,
            Int32 label_count
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_labeled_boolean_metric_get(UInt64 handle, string label);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_labeled_boolean_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // Labeled string

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_labeled_string_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled,
            string[] labels,
            Int32 label_count
          );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_labeled_string_metric_get(UInt64 handle, string label);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_labeled_string_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // JWE

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_jwe_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_jwe_metric(UInt64 handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_jwe_set(
            UInt64 metric_id,
            string header,
            string key,
            string init_vector,
            string cipher_text,
            string auth_tag
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_jwe_set_with_compact_representation(UInt64 metric_id, string value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_jwe_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_jwe_test_get_value_as_json_string(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_jwe_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_jwe_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // StringList

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_string_list_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_string_list_metric(UInt64 handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_string_list_add(UInt64 metric_id, string value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_string_list_set(UInt64 metric_id, string[] value, Int32 values_len);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_string_list_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_string_list_test_get_value_as_json_string(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_string_list_test_get_num_recorded_errors(
             UInt64 metric_id,
             Int32 error_type,
             string storage_name
        );

        // Quantity

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_quantity_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_quantity_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_quantity_set(UInt64 metric_id, Int32 value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_quantity_test_get_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern bool glean_quantity_test_has_value(UInt64 metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_quantity_test_get_num_recorded_errors(
            UInt64 metric_id,
            Int32 error_type,
            String storage_name
        );

        // Custom pings

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern UInt64 glean_new_ping_type(
            string name,
            byte include_client_id,
            byte send_if_empty,
            string[] reason,
            Int32 reason_codes_len
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_ping_type(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_register_ping_type(UInt64 ping_type_handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_test_has_ping_type(string ping_name);

        // Upload API

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_get_upload_task(ref FfiUploadTask result);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_process_ping_upload_response(IntPtr task, int status);

        // Misc

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_str_free(IntPtr ptr);
    }
}
