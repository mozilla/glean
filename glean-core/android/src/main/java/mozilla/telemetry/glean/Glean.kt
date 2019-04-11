/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.MetricHandle
import mozilla.telemetry.glean.rust.RustError

open class GleanInternalAPI internal constructor () {
    // `internal` so this can be modified for testing
    internal var initialized = false
    internal var bool_metric : MetricHandle = 0

    /**
     * Initialize glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using glean.
     */
    fun initialize() {
        if (isInitialized()) {
            return
        }
        initialized = true
        LibGleanFFI.INSTANCE.glean_initialize()

        val e = RustError.ByReference()
        bool_metric = LibGleanFFI.INSTANCE.glean_new_boolean_metric("enabled", "glean", e)
    }

    /**
     * Returns true if the Glean library has been initialized.
     */
    internal fun isInitialized(): Boolean {
        return initialized
    }
}

object Glean : GleanInternalAPI() {
}
