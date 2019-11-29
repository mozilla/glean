/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import android.content.SharedPreferences
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanMetrics.GleanBaseline
import mozilla.telemetry.glean.GleanMetrics.GleanValidation

/**
 * Connects process lifecycle events from Android to Glean's handleEvent
 * functionality (where the actual work of sending pings is done).
 */
internal class GleanLifecycleObserver(
    private val applicationContext: Context
) : LifecycleEventObserver {
    // Using SharedPreferences to track the status here should be less of an I/O hit
    // compared to plain file usage, given that most of its operations are off the main
    // thread. This also gives us the nice guarantee that once the function is called
    // the OS will keep SharedPreferences working, even though the process is being killed.
    internal val sharedPreferences: SharedPreferences by lazy {
        applicationContext.getSharedPreferences("DirtyBitStatus", Context.MODE_PRIVATE)
    }

    /**
     * Called when lifecycle events are triggered.
     */
    override fun onStateChanged(source: LifecycleOwner, event: Lifecycle.Event) {
        when (event) {
            Lifecycle.Event.ON_STOP -> {
                // We're going to background, so store how much time we spent
                // on foreground.
                GleanBaseline.duration.stop()
                Glean.handleBackgroundEvent()

                // Clear the "dirty bit".
                sharedPreferences.edit().putBoolean("dirty", false).apply()
            }
            Lifecycle.Event.ON_START -> {
                // Updates the baseline.duration metric when entering the foreground.
                // We use ON_START here because we don't want to incorrectly count metrics in
                // ON_RESUME as pause/resume can happen when interacting with things like the
                // navigation shade which could lead to incorrectly recording the start of a
                // duration, etc.
                //
                // https://developer.android.com/reference/android/app/Activity.html#onStart()

                // Note that this is sending the length of the last foreground session
                // because it belongs to the baseline ping and that ping is sent every
                // time the app goes to background.
                GleanBaseline.duration.start()

                // Check if the "dirty bit" was set. If it was, increment the metric.
                // Flip the dirty bit to 'true' after.
                if (sharedPreferences.getBoolean("dirty", false)) {
                    GleanValidation.appForceclosedCount.add(1)
                }

                // Set the "dirty bit".
                sharedPreferences.edit().putBoolean("dirty", true).apply()
            }
            else -> {
                // For other lifecycle events, do nothing
            }
        }
    }
}
