/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.debug

import android.app.Activity
import android.os.Bundle
import android.util.Log
import mozilla.telemetry.glean.Glean

/**
 * Debugging activity exported by Glean to allow easier debugging.
 * For example, invoking debug mode in the Glean sample application
 * can be done via adb using the following command:
 *
 * adb shell am start -n org.mozilla.samples.glean/mozilla.telemetry.glean.debug.GleanDebugActivity
 *
 * See the adb developer docs for more info:
 * https://developer.android.com/studio/command-line/adb#am
 */
class GleanDebugActivity : Activity() {
    companion object {
        private const val LOG_TAG = "glean/DebugActivity"

        // This is a list of the currently accepted commands
        /**
         * Sends the ping with the given name immediately
         */
        const val SEND_PING_EXTRA_KEY = "sendPing"
        /**
         * If set to `true`, pings are dumped to logcat, defaults to `false`.
         */
        const val LOG_PINGS_EXTRA_KEY = "logPings"
        /**
         * Tags all outgoing pings as debug pings to make them available for real-time validation.
         * The value must match the pattern `[a-zA-Z0-9-]{1,20}`.
         */
        const val TAG_DEBUG_VIEW_EXTRA_KEY = "tagPings"

        // Regular expression filter for debugId
        internal val pingTagPattern = "[a-zA-Z0-9-]{1,20}".toRegex()
    }

    // IMPORTANT: These activities are unsecured, and may be triggered by
    // any other application on the device, including in release builds.
    // Therefore, care should be taken in selecting what features are
    // exposed this way.  For example, it would be dangerous to change the
    // submission URL.

    /**
     * On creation of the debug activity, launch the requested command.
     */
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        if (!Glean.isInitialized()) {
            Log.e(LOG_TAG,
                "Glean is not initialized. " +
                "It may be disabled by the application."
            )
            finish()
            return
        }

        if (intent.extras == null) {
            Log.e(LOG_TAG, "No debugging option was provided, doing nothing.")
            finish()
            return
        }

        // Make sure that at least one of the supported commands was used.
        val supportedCommands =
            listOf(SEND_PING_EXTRA_KEY, LOG_PINGS_EXTRA_KEY, TAG_DEBUG_VIEW_EXTRA_KEY)

        // Enable debugging options and start the application.
        intent.extras?.let {
            it.keySet().forEach { cmd ->
                if (!supportedCommands.contains(cmd)) {
                    Log.e(LOG_TAG, "Unknown command '$cmd'.")
                }
            }

            // Check for ping debug view tag to apply to the X-Debug-ID header when uploading the
            // ping to the endpoint
            var pingTag: String? = intent.getStringExtra(TAG_DEBUG_VIEW_EXTRA_KEY)

            // Validate the ping tag against the regex pattern
            pingTag?.let {
                if (!pingTagPattern.matches(it)) {
                    Log.e(LOG_TAG, "tagPings value $it does not match accepted pattern $pingTagPattern")
                    pingTag = null
                }
            }

            val debugConfig = Glean.configuration.copy(
                logPings = intent.getBooleanExtra(LOG_PINGS_EXTRA_KEY, Glean.configuration.logPings),
                pingTag = pingTag
            )

            // Finally set the default configuration before starting
            // the real product's activity.
            Log.i(LOG_TAG, "Setting debug config $debugConfig")
            Glean.configuration = debugConfig

            intent.getStringExtra(SEND_PING_EXTRA_KEY)?.let {
                Glean.submitPingsByName(listOf(it))
            }
        }

        val intent = packageManager.getLaunchIntentForPackage(packageName)
        startActivity(intent)

        finish()
    }
}
