/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.debug

import android.app.Activity
import android.content.ComponentName
import android.content.Intent
import android.content.pm.PackageManager
import android.os.Bundle
import android.util.Log
import mozilla.telemetry.glean.Glean

/**
 * Debugging activity exported by Glean to allow easier debugging.
 * For example, invoking debug mode in the Glean sample application
 * can be done via adb using the following command:
 *
 * adb shell am start -n org.mozilla.samples.gleancore/mozilla.telemetry.glean.debug.GleanDebugActivity
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
        const val TAG_DEBUG_VIEW_EXTRA_KEY = "debugViewTag"
        const val LEGACY_TAG_PINGS = "tagPings"

        /**
         * Tags all outgoing pings as debug pings to make them available for real-time validation.
         * The value must match the pattern `[a-zA-Z0-9-]{1,20}`.
         */
        const val SOURCE_TAGS_KEY = "sourceTags"

        /**
         * Executes the activity with the provided name instead of the main one
         * after finishing with the `GleanDebugActivity`.
         */
        const val NEXT_ACTIVITY_TO_RUN = "startNext"
    }

    // IMPORTANT: These activities are unsecured, and may be triggered by
    // any other application on the device, including in release builds.
    // Therefore, care should be taken in selecting what features are
    // exposed this way.  For example, it would be dangerous to change the
    // submission URL.

    private fun isActivityExported(targetActivity: ComponentName): Boolean {
        return try {
            packageManager.getActivityInfo(targetActivity, PackageManager.GET_META_DATA).exported
        } catch (e: PackageManager.NameNotFoundException) {
            false
        }
    }

    /**
     * On creation of the debug activity, launch the requested command.
     */
    @Suppress("ComplexMethod", "LongMethod", "ReturnCount")
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
        val supportedCommands = listOf(
            SEND_PING_EXTRA_KEY, LOG_PINGS_EXTRA_KEY, NEXT_ACTIVITY_TO_RUN,
            TAG_DEBUG_VIEW_EXTRA_KEY, SOURCE_TAGS_KEY, LEGACY_TAG_PINGS
        )

        var nextActivityName: String? = null

        // Enable debugging options and start the application.
        intent.extras?.let {
            it.keySet().forEach { cmd ->
                if (!supportedCommands.contains(cmd)) {
                    Log.e(LOG_TAG, "Unknown command '$cmd'.")
                }
            }

            // Check for ping debug view tag to apply to the X-Debug-ID header when uploading the
            // ping to the endpoint
            val debugViewTag: String? = intent.getStringExtra(TAG_DEBUG_VIEW_EXTRA_KEY)

            // Set the debug view tag, if the tag is invalid it won't be set
            debugViewTag?.let {
                Glean.setDebugViewTag(debugViewTag)
            } ?: run {
                // If the 'debugViewTag' was not used, try to look for the legacy
                // way of setting debug view tags. We should leave this block of
                // code around at most until December 2020.
                intent.getStringExtra(LEGACY_TAG_PINGS)?.let { legacyTag ->
                    Glean.setDebugViewTag(legacyTag)
                }
            }

            val logPings: Boolean? = intent.getBooleanExtra(LOG_PINGS_EXTRA_KEY, false)
            logPings?.let {
                Glean.setLogPings(logPings)
            }

            intent.getStringArrayExtra(SOURCE_TAGS_KEY)?.let { tags ->
                Glean.setSourceTags(tags.toSet())
            }

            intent.getStringExtra(NEXT_ACTIVITY_TO_RUN)?.let { nextActivity ->
                nextActivityName = nextActivity
            }

            // Important: this should be applied as the last one, so that
            // any other option will affect the ping submission as well.
            intent.getStringExtra(SEND_PING_EXTRA_KEY)?.let { name ->
                Glean.submitPingByName(name)
            }
        }

        // This Activity can be used to tag tests on CI or start products with specific
        // options. We need to make sure to retain and propagate all the options that
        // we were passed to the next intent. Our 3 steps strategy:
        // 1. use the `Intent` copy constructor to copy all the intent options from the
        //   intent which started this activity;
        val nextIntent = Intent(intent)

        // 2. get the main launch intent for the product using the Glean SDK or lookup
        //    the one passed as a command line argument to this activity.
        val launchIntent = packageManager.getLaunchIntentForPackage(packageName)!!
        nextIntent.`package` = launchIntent.`package`

        val nextComponent = nextActivityName?.let { name ->
            // Try to lookup the Activity that was asked by the caller.
            val component = ComponentName(packageName, name)
            if (!isActivityExported(component)) {
                Log.e(LOG_TAG, "Cannot run $packageName/$name: Activity not exported")
                finish()
                return
            }
            component
        } ?: launchIntent.component

        Log.i(LOG_TAG, "Running next: ${nextComponent!!.packageName}/${nextComponent.className}")

        // 3. change the starting "component" to the one from the previous step.
        nextIntent.component = nextComponent
        startActivity(nextIntent)

        finish()
    }
}
