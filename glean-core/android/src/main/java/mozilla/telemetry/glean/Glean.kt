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
    internal var bool_metric : MetricHandle = 0

    /**
     * Initialize glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using glean.
     */
    fun initialize(applicationContext: Context) {
        val data_dir = File(applicationContext.applicationInfo.dataDir, "glean_data")
        Log.i("glean-kotlin", "data dir: $data_dir")

        if (isInitialized()) {
            return
        }

        LibGleanFFI.INSTANCE.glean_initialize(data_dir.getPath())

        val e = RustError.ByReference()
        bool_metric = LibGleanFFI.INSTANCE.glean_new_boolean_metric("enabled", "glean", e)
    }

    /**
     * Returns true if the Glean library has been initialized.
     */
    internal fun isInitialized(): Boolean {
        val initialized = LibGleanFFI.INSTANCE.glean_is_initialized()
        return initialized.toInt() != 0
    }

    fun collect(ping_name: String) {
        val e = RustError.ByReference()
        val s = LibGleanFFI.INSTANCE.glean_ping_collect(ping_name, e)!!
        LibGleanFFI.INSTANCE.glean_str_free(s)

    }
}

object Glean : GleanInternalAPI() {
}
