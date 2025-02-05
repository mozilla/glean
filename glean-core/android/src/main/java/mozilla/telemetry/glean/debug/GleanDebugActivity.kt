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
            @Suppress("DEPRECATION")
            packageManager.getActivityInfo(targetActivity, PackageManager.GET_META_DATA).exported
        } catch (_: PackageManager.NameNotFoundException) {
            false
        }
    }

    /**
     * On creation of the debug activity, launch the requested command.
     */
    @Suppress("ComplexMethod", "LongMethod", "ReturnCount")
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        if (intent.extras == null) {
            Log.e(LOG_TAG, "No debugging option was provided, doing nothing.")
            finish()
            return
        }

        val supportedCommands = listOf(
            SEND_PING_EXTRA_KEY,
            LOG_PINGS_EXTRA_KEY,
            NEXT_ACTIVITY_TO_RUN,
            TAG_DEBUG_VIEW_EXTRA_KEY,
            SOURCE_TAGS_KEY,
        )

        var nextActivityName: String? = null

        intent.extras?.let {
            it.keySet().forEach { cmd ->
                if (!supportedCommands.contains(cmd)) {
                    Log.e(LOG_TAG, "Unknown command '$cmd'.")
                }
            }

            intent.getStringExtra(TAG_DEBUG_VIEW_EXTRA_KEY)?.let { Glean.setDebugViewTag(it) }
            intent.getBooleanExtra(LOG_PINGS_EXTRA_KEY, false).let { Glean.setLogPings(it) }
            intent.getStringArrayExtra(SOURCE_TAGS_KEY)?.let { tags -> Glean.setSourceTags(tags.toSet()) }
            intent.getStringExtra(NEXT_ACTIVITY_TO_RUN)?.let { nextActivityName = it }
            // Important: this should be applied as the last one, so that
            // any other option will affect the ping submission as well.
            intent.getStringExtra(SEND_PING_EXTRA_KEY)?.let { Glean.submitPingByName(it) }
        }

        // This Activity can be used to tag tests on CI or start products with specific
        // options. We need to make sure to retain and propagate all the options that
        // we were passed to the next intent. Our 3 steps strategy:
        // 1. Copy intent and make extras immutable
        val safeExtras = Bundle(intent.extras).apply { setClassLoader(javaClass.classLoader) }
        val nextIntent = Intent().apply { replaceExtras(safeExtras) }

        // 2. Determine the next component
        val launchIntent = packageManager.getLaunchIntentForPackage(packageName)!!
        val nextComponent = nextActivityName?.let { name ->
            val component = ComponentName(packageName, name)
            if (!isActivityExported(component)) {
                Log.e(LOG_TAG, "Cannot run $packageName/$name: Activity not exported")
                finish()
                return
            }
            component
        } ?: launchIntent.component!!

        Log.i(LOG_TAG, "Running next: ${nextComponent.packageName}/${nextComponent.className}")

        // 3. Configure and launch intent safely
        nextIntent.apply {
            setClassName(packageName, nextComponent.className)
            addFlags(Intent.FLAG_ACTIVITY_NEW_TASK or Intent.FLAG_ACTIVITY_CLEAR_TASK)
        }

        startActivity(nextIntent)
        finish()
    }
}
