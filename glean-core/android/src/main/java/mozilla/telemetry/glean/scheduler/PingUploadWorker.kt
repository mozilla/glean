/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import android.os.SystemClock
import androidx.annotation.VisibleForTesting
import androidx.work.Constraints
import androidx.work.ExistingWorkPolicy
import androidx.work.NetworkType
import androidx.work.OneTimeWorkRequest
import androidx.work.OneTimeWorkRequestBuilder
import androidx.work.WorkManager
import androidx.work.Worker
import androidx.work.WorkerParameters
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.internal.PingUploadTask
import mozilla.telemetry.glean.internal.UploadTaskAction
import mozilla.telemetry.glean.internal.gleanGetUploadTask
import mozilla.telemetry.glean.internal.gleanProcessPingUploadResponse
import mozilla.telemetry.glean.net.CapablePingUploadRequest
import mozilla.telemetry.glean.net.PingUploadRequest
import mozilla.telemetry.glean.utils.testFlushWorkManagerJob

/**
 * Build the constraints around which the worker can be run, such as whether network
 * connectivity is required.
 *
 * @return [Constraints] object containing the required work constraints
 */
@VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
internal fun buildConstraints(): Constraints =
    Constraints
        .Builder()
        .setRequiredNetworkType(NetworkType.CONNECTED)
        .build()

/**
 * Build the [OneTimeWorkRequest] for enqueueing in the [WorkManager].  This also adds a tag
 * by which enqueued requests can be identified.
 *
 * @return [OneTimeWorkRequest] representing the task for the [WorkManager] to enqueue and run
 */
internal inline fun <reified W : Worker> buildWorkRequest(tag: String): OneTimeWorkRequest =
    OneTimeWorkRequestBuilder<W>()
        .addTag(tag)
        .setConstraints(buildConstraints())
        .build()

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
                buildWorkRequest<PingUploadWorker>(PING_WORKER_TAG),
            )

            // Only flush pings immediately if sending to a test endpoint,
            // which means we're probably in instrumented tests.
            if (Glean.isSendingToTestEndpoint) {
                testFlushWorkManagerJob(context, PING_WORKER_TAG)
            }
        }

        /**
         * Perform the upload task synchronously.
         *
         * This will be called from the [doWork] function of the [PingUploadWorker] when Glean is
         * being run from the main process of an application, but for background services it will
         * be called from the Glean.Dispatchers couroutine scope to avoid WorkManager complexity
         * for multi-process applications. See Bug1844533 for more information.
         *
         * @param context the application [Context]
         */
        @OptIn(ExperimentalUnsignedTypes::class)
        internal fun performUpload() {
            do {
                when (val action = gleanGetUploadTask()) {
                    is PingUploadTask.Upload -> {
                        // Upload the ping request.
                        // If the status is `null` there was some kind of unrecoverable error
                        // so we return a known unrecoverable error status code
                        // which will ensure this gets treated as such.
                        val body = action.request.body
                            .toUByteArray()
                            .asByteArray()
                        val request = CapablePingUploadRequest(
                            PingUploadRequest(
                                Glean.configuration.serverEndpoint + action.request.path,
                                body,
                                action.request.headers,
                                action.request.uploaderCapabilities,
                            ),
                        )
                        val result = Glean.httpClient.doUpload(request)

                        // Process the upload response
                        when (gleanProcessPingUploadResponse(action.request.documentId, result)) {
                            UploadTaskAction.NEXT -> continue
                            UploadTaskAction.END -> break
                        }
                    }
                    is PingUploadTask.Wait -> SystemClock.sleep(action.time.toLong())
                    is PingUploadTask.Done -> break
                }
            } while (true)
            // Limits are enforced by glean-core to avoid an inifinite loop here.
            // Whenever a limit is reached, this binding will receive `PingUploadTask.Done` and step out.
        }

        /**
         * Function to cancel any pending ping upload workers
         *
         * @param context the application [Context] to get the [WorkManager] instance for
         */
        internal fun cancel(context: Context) {
            WorkManager.getInstance(context).cancelUniqueWork(PING_WORKER_TAG)
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
        performUpload()
        return Result.success()
    }
}
