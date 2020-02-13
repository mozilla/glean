/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import android.content.Context
import androidx.work.WorkInfo
import androidx.work.WorkManager
import androidx.work.testing.WorkManagerTestInitHelper
import kotlinx.coroutines.isActive
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout

/**
 * TEST ONLY FUNCTION.
 * Sets all the constraints for the provided workmanager work units as met.
 *
 * Waits until the timeout is reached if no work unit is found with the provided
 * tag.
 *
 * @param context the application test context
 * @param workTag the tag of the WorkManager work unit
 * @param timeoutMs the maximum time to wait for the job to be enqueued in the WorkManager
 */
internal fun testFlushWorkManagerJob(context: Context, workTag: String, timeoutMs: Long = 5000L) {
    runBlocking {
        withTimeout(timeoutMs) {
            do {
                val workInfoList = WorkManager.getInstance(context).getWorkInfosByTag(workTag).get()
                workInfoList.forEach { workInfo ->
                    if (workInfo.state === WorkInfo.State.ENQUEUED) {
                        // Trigger WorkManager using TestDriver
                        val testDriver = WorkManagerTestInitHelper.getTestDriver(context)
                        testDriver?.setAllConstraintsMet(workInfo.id)
                        return@withTimeout
                    }
                }
            } while (isActive)
        }
    }
}
