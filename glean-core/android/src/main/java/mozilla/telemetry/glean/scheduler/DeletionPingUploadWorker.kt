/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import androidx.work.ExistingWorkPolicy
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import mozilla.telemetry.glean.Glean
import java.io.File

private const val LOG_TAG = "glean/DeletionPing"

/**
 * This class is the worker class used by [WorkManager] to handle uploading the ping to the server.
 * @suppress This is internal only, don't show it in the docs.
 */
class DeletionPingUploadWorker(context: Context, params: WorkerParameters) : Worker(context, params) {
    companion object {
        internal const val PING_WORKER_TAG = "mozac_service_glean_deletion_ping_upload_worker"

        // NOTE: The `PINGS_DIR` must be kept in sync with the one in the Rust implementation.
        internal const val DELETION_PING_DIR = "deletion_request"
        // A lock to prevent simultaneous writes in the ping queue directory.
        // In particular, there are issues if the pings are cleared (as part of
        // disabling telemetry), while the ping uploader is trying to upload queued pings.
        // Therefore, this lock is held both when uploading pings and when calling
        // into the Rust code that might clear queued pings (set_upload_enabled).
        internal val pingQueueLock = Any()

        /**
         * Function to aid in properly enqueuing the worker in [WorkManager]
         *
         * @param context the application [Context] to get the [WorkManager] instance for
         */
        internal fun enqueueWorker(context: Context) {
            WorkManager.getInstance(context).enqueueUniqueWork(
                PING_WORKER_TAG,
                ExistingWorkPolicy.KEEP,
                buildWorkRequest<DeletionPingUploadWorker>(PING_WORKER_TAG))
        }

        /**
         * Function to perform the actual ping upload task.  This is created here in the
         * companion object in order to facilitate testing.
         *
         * @return true if process was successful
         */

        /**
         * Function to cancel any pending ping upload workers
         *
         * @param context the application [Context] to get the [WorkManager] instance for
         */
        internal fun cancel(context: Context) {
            WorkManager.getInstance(context).cancelUniqueWork(PING_WORKER_TAG)
        }

        /**
         * Function to deserialize and process all serialized ping files.  This function will ignore
         * files that don't match the UUID regex and just delete them to prevent files from polluting
         * the ping storage directory.
         *
         * @return Boolean representing the success of the upload task. This may be the value bubbled up
         *         from the callback, or if there was an error reading the files.
         */
        internal fun uploadPings(): Boolean {
            val storageDirectory = File(Glean.getDataDir(), DELETION_PING_DIR)

            return processDirectory(pingQueueLock, storageDirectory)
        }
    }

    /**
     * This method is called on a background thread - you are required to **synchronously** do your
     * work and return the [androidx.work.ListenableWorker.Result] from this method.  Once you
     * return from this method, the Worker is considered to have finished what its doing and will be
     * destroyed.
     *
     * A Worker is given a maximum of ten minutes to finish its execution and return a
     * [androidx.work.ListenableWorker.Result].  After this time has expired, the Worker will
     * be signalled to stop.
     *
     * @return The [androidx.work.ListenableWorker.Result] of the computation
     */
    override fun doWork(): Result {
        return when {
            !uploadPings() -> Result.retry()
            else -> Result.success()
        }
    }
}
