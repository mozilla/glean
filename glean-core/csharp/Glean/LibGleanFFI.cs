// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Mozilla.Glean.FFI
{
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
            public bool upload_enabled;
            public Int32? max_events;
            public bool delay_ping_lifetime_io;
        }

        /// <summary>
        /// A base handle class meant to be extended by the different metric types to allow
        /// for calling metric specific clearing functions.
        /// </summary>
        internal class BaseGleanHandle : SafeHandle
        {
            public BaseGleanHandle() : base(invalidHandleValue: IntPtr.Zero, ownsHandle: true) { }

            public override bool IsInvalid
            {
                get { return this.handle == IntPtr.Zero; }
            }

            protected override bool ReleaseHandle()
            {
                // Note: this is meant to be implemented by the inheriting class in order to
                // provide a specific cleanup action.
                return false;
            }
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
                int len = 0;
                while (Marshal.ReadByte(handle, len) != 0) { ++len; }
                byte[] buffer = new byte[len];
                Marshal.Copy(handle, buffer, 0, buffer.Length);
                return Encoding.UTF8.GetString(buffer);
            }

            protected override bool ReleaseHandle()
            {
                if (!this.IsInvalid)
                {
                    Console.WriteLine("Freeing string handle");
                    glean_str_free(handle);
                }

                return true;
            }
        }

        // Glean top-level API.

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_initialize(FfiConfiguration cfg);

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

        // TODO: add the rest of the ffi.

        // String

        /// <summary>
        /// A handle for the string metric type, which performs cleanup.
        /// </summary>
        internal sealed class StringMetricTypeHandle : BaseGleanHandle
        {
            protected override bool ReleaseHandle()
            {
                if (!this.IsInvalid)
                {
                    Console.WriteLine("Freeing string metric type handle");
                    glean_destroy_string_metric(handle);
                }

                return true;
            }
        }

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringMetricTypeHandle glean_new_string_metric(
            string category,
            string name,
            string[] send_in_pings,
            Int32 send_in_pings_len,
            Int32 lifetime,
            bool disabled
        );

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_destroy_string_metric(IntPtr handle);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_string_set(StringMetricTypeHandle metric_id, string value);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern StringAsReturnValue glean_string_test_get_value(StringMetricTypeHandle metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_string_test_has_value(StringMetricTypeHandle metric_id, string storage_name);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern Int32 glean_string_test_get_num_recorded_errors(
             StringMetricTypeHandle metric_id,
             Int32 error_type,
             string storage_name
         );

        // Misc

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern void glean_str_free(IntPtr ptr);
    }
}
