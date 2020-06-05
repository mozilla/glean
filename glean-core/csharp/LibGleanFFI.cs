// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Runtime.InteropServices;
using System.Text;

namespace Mozilla.Glean.FFI
{
    static class LibGleanFFI
    {
        private const string SharedGleanLibrary = "glean_ffi.dll";

        // Define the order of fields as laid out in memory.
        // **CAUTION**: This must match _exactly_ the definition on the Rust side.
        //  If this side is changed, the Rust side need to be changed, too.
        [StructLayout(LayoutKind.Sequential)]
        internal class FfiConfiguration
        {
            public string data_dir;
            public string package_name;
            public bool upload_enabled;
            public Int32 max_events;
            public bool delay_ping_lifetime_io;
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
        internal static extern void glean_set_upload_enabled(byte flag);

        [DllImport(SharedGleanLibrary, ExactSpelling = true, CallingConvention = CallingConvention.Cdecl)]
        internal static extern byte glean_is_upload_enabled();

        // TODO: add the rest of the ffi.
    }
}
