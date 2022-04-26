/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleEventObserver
import androidx.lifecycle.LifecycleOwner
import mozilla.telemetry.glean.Glean

/**
 * Connects process lifecycle events from Android to Glean's handleEvent
 * functionality (where the actual work of sending pings is done).
 */
internal class GleanLifecycleObserver : LifecycleEventObserver {
    /**
     * Called when lifecycle events are triggered.
     */
    override fun onStateChanged(source: LifecycleOwner, event: Lifecycle.Event) {
        when (event) {
            Lifecycle.Event.ON_STOP -> {
                Glean.handleBackgroundEvent()
            }
            Lifecycle.Event.ON_START -> {
                // We use ON_START here because we don't want to incorrectly count metrics in
                // ON_RESUME as pause/resume can happen when interacting with things like the
                // navigation shade which could lead to incorrectly recording the start of a
                // duration, etc.
                //
                // https://developer.android.com/reference/android/app/Activity.html#onStart()

                Glean.handleForegroundEvent()
            }
            else -> {
                // For other lifecycle events, do nothing
            }
        }
    }
}
