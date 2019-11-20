/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import android.os.Looper

/**
 * Utilities related to threads.
 */
object ThreadUtils {
    private val uiThread = Looper.getMainLooper().thread

    /**
     * Assert that this code is run on the main (UI) thread.
     */
    fun assertOnUiThread() {
        val currentThread = Thread.currentThread()
        val currentThreadId = currentThread.id
        val expectedThreadId = uiThread.id

        if (currentThreadId == expectedThreadId) {
            return
        }

        throw IllegalThreadStateException("Expected UI thread, but running on " + currentThread.name)
    }
}
