/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.rust

import android.util.Log
import com.sun.jna.Library
import com.sun.jna.Native
import com.sun.jna.Pointer
import com.sun.jna.PointerType
import java.lang.reflect.Proxy

internal interface LibGleanFFI : Library {
    companion object {
        private val JNA_LIBRARY_NAME = "glean_ffi"

        internal var INSTANCE: LibGleanFFI = try {
            Native.loadLibrary(JNA_LIBRARY_NAME, LibGleanFFI::class.java) as LibGleanFFI
        } catch (e: UnsatisfiedLinkError) {
            Proxy.newProxyInstance(
                LibGleanFFI::class.java.classLoader,
                arrayOf(LibGleanFFI::class.java))
            { _, _, _ ->
                throw RuntimeException("Glean functionality not available", e)
            } as LibGleanFFI
        }
    }

    // Important: strings returned from rust as *mut char must be Pointers on this end, returning a
    // String will work but either force us to leak them, or cause us to corrupt the heap (when we
    // free them).

    fun glean_boolean_set(metric_id: Long, value: Byte, error: RustError.ByReference)

    fun glean_counter_add(metric_id: Long, amount: Long, error: RustError.ByReference)

    fun glean_initialize(data_dir: String, application_id: String)

    fun glean_is_initialized(): Byte

    fun glean_is_upload_enabled(): Byte

    fun glean_new_boolean_metric(category: String, name: String, err: RustError.ByReference): Long

    fun glean_new_counter_metric(category: String, name: String, err: RustError.ByReference): Long

    fun glean_new_string_metric(category: String, name: String, err: RustError.ByReference): Long

    fun glean_ping_collect(ping_name: String, error: RustError.ByReference): Pointer?

    fun glean_set_upload_enabled(flag: Byte)

    fun glean_string_set(metric_id: Long, value: String, error: RustError.ByReference)

    fun glean_destroy_boolean_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_string_metric(handle: Long, error: RustError.ByReference)

    fun glean_destroy_counter_metric(handle: Long, error: RustError.ByReference)

    fun glean_str_free(ptr: Pointer)

}
internal typealias MetricHandle = Long
