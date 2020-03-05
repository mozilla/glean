/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import androidx.annotation.VisibleForTesting
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequest
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.utils.testFlushWorkManagerJob
import mozilla.telemetry.glean.upload.PingUploadTask

/**
 * Build the constraints around which the worker can be run, such as whether network
 * connectivity is required.
 *
 * @return [Constraints] object containing the required work constraints
 */
@VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
internal fun buildConstraints(): Constraints = Constraints.Builder()
    .setRequiredNetworkType(NetworkType.CONNECTED)
    .build()

/**
 * Build the [OneTimeWorkRequest] for enqueueing in the [WorkManager].  This also adds a tag
 * by which enqueued requests can be identified.
 *
 * @return [OneTimeWorkRequest] representing the task for the [WorkManager] to enqueue and run
 */
internal inline fun <reified W : Worker> buildWorkRequest(tag: String): OneTimeWorkRequest {
    return OneTimeWorkRequestBuilder<W>()
        .addTag(tag)
        .setConstraints(buildConstraints())
        .build()
}

/**
 * This class is the worker class used by [WorkManager] to handle uploading the ping to the server.
 * @suppress This is internal only, don't show it in the docs.
 */
class PingUploadWorker(context: Context, params: WorkerParameters) : Worker(context, params) {
    companion object {
        internal const val PING_WORKER_TAG = "mozac_service_glean_ping_upload_worker"

        /**
         * Function to aid in properly enqueuing the worker in [WorkManager]
         *
         * @param context the application [Context] to get the [WorkManager] instance for
         */
        internal fun enqueueWorker(context: Context) {
            WorkManager.getInstance(context).enqueueUniqueWork(
                PING_WORKER_TAG,
                ExistingWorkPolicy.KEEP,
                buildWorkRequest<PingUploadWorker>(PING_WORKER_TAG)
            )

            // Only flush pings immediately if sending to a test endpoint,
            // which means we're probably in instrumented tests.
            if (Glean.isSendingToTestEndpoint) {
                testFlushWorkManagerJob(context, PING_WORKER_TAG)
            }
        }

        /**
         * Function to cancel any pending ping upload workers
         *
         * @param context the application [Context] to get the [WorkManager] instance for
         */
        internal fun cancel(context: Context) {
            WorkManager.getInstance(context).cancelUniqueWork(PING_WORKER_TAG)
        }

        /**
         * Function to perform the actual ping upload task.
         *
         * @return true if process was successful
         */
        internal fun uploadPings(): Boolean {
            var ffiTask = LibGleanFFI.INSTANCE.glean_get_upload_task()
            var task = ffiTask.toPingUploadTask()
            while (task is PingUploadTask.Upload) {
                // Get the request
                val request = task.request()!!
                // Upload the request
                // If the status is `null` there was some kind of unrecoverable error
                // so we return 400 which will ensure this gets treated as such.
                @Suppress("MagicNumber")
                val status = Glean.httpClient.doUpload(
                        request.path,
                        request.body,
                        request.headers,
                        Glean.configuration
                ) ?: 400
                // Process the upload response
                LibGleanFFI.INSTANCE.glean_process_ping_upload_response(ffiTask, status)
                // Get the next task
                ffiTask = LibGleanFFI.INSTANCE.glean_get_upload_task()
                task = ffiTask.toPingUploadTask()
            }

            return when (task) {
                is PingUploadTask.Wait -> false
                is PingUploadTask.Done -> true
                else -> throw IllegalStateException("Unknown ping uploading task!")
            }
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
