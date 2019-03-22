/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

open class GleanInternalAPI internal constructor () {
    private val logger = Logger("glean/Glean")

    // Include our singletons of StorageEngineManager and PingMaker
    // `internal` so this can be modified for testing
    internal var initialized = false

    /**
     * Initialize glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using glean. A message is logged to error and no changes are made
     * to the state if initialize is called a more than once.
     *
     * A LifecycleObserver will be added to send pings when the application goes
     * into the background.
     *
     * @param applicationContext [Context] to access application features, such
     * as shared preferences
     * @param configuration A Glean [Configuration] object with global settings.
     */
    fun initialize() {
        if (isInitialized()) {
            logger.error("Glean should not be initialized multiple times")
            return
        }
        initialized = true
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
