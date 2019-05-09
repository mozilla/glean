/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.util.Log
import android.content.Context
import java.io.File
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.MetricHandle
import mozilla.telemetry.glean.rust.RustError

open class GleanInternalAPI internal constructor () {
    // `internal` so this can be modified for testing
    internal var bool_metric: MetricHandle = 0L
    internal var handle: MetricHandle = 0L

    /**
     * Initialize glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using glean.
     */
    fun initialize(applicationContext: Context) {
        val dataDir = File(applicationContext.applicationInfo.dataDir, "glean_data")
        Log.i("glean-kotlin", "data dir: $dataDir")

        if (isInitialized()) {
            return
        }

        handle = LibGleanFFI.INSTANCE.glean_initialize(dataDir.path, applicationContext.packageName)

        val e = RustError.ByReference()
        bool_metric = LibGleanFFI.INSTANCE.glean_new_boolean_metric("glean", "enabled", e)
    }

    /**
     * Returns true if the Glean library has been initialized.
     */
    internal fun isInitialized(): Boolean {
        if (handle == 0L) {
            return false
        }

        val initialized = LibGleanFFI.INSTANCE.glean_is_initialized(handle)
        return initialized.toInt() != 0
    }

    fun collect(ping_name: String) {
        val e = RustError.ByReference()
        val s = LibGleanFFI.INSTANCE.glean_ping_collect(handle, ping_name, e)!!
        LibGleanFFI.INSTANCE.glean_str_free(s)
    }
}

object Glean : GleanInternalAPI()
