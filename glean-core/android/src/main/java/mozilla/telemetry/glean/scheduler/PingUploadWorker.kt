/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import android.util.Log
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
import java.io.BufferedReader
import java.io.File
import java.io.FileNotFoundException
import java.io.FileReader
import java.io.IOException

/**
 * This class is the worker class used by [WorkManager] to handle uploading the ping to the server.
 * @suppress This is internal only, don't show it in the docs.
 */
class PingUploadWorker(context: Context, params: WorkerParameters) : Worker(context, params) {
    companion object {
        internal const val PING_WORKER_TAG = "mozac_service_glean_ping_upload_worker"

        // Since ping file names are UUIDs, this matches UUIDs for filtering purposes
        private const val FILE_PATTERN = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}"
        private const val LOG_TAG = "glean/PingUploadWorker"
        // NOTE: The `PINGS_DIR` must be kept in sync with the one in the Rust implementation.
        internal const val PINGS_DIR = "pending_pings"
        // A lock to prevent simultaneous writes in the ping queue directory.
        // In particular, there are issues if the pings are cleared (as part of
        // disabling telemetry), while the ping uploader is trying to upload queued pings.
        // Therefore, this lock is held both when uploading pings and when calling
        // into the Rust code that might clear queued pings (set_upload_enabled).
        internal val pingQueueLock = Any()

        /**
         * Build the constraints around which the worker can be run, such as whether network
         * connectivity is required.
         *
         * @return [Constraints] object containing the required work constraints
         */
        @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
        fun buildConstraints(): Constraints = Constraints.Builder()
            .setRequiredNetworkType(NetworkType.CONNECTED)
            .build()

        /**
         * Build the [OneTimeWorkRequest] for enqueueing in the [WorkManager].  This also adds a tag
         * by which enqueued requests can be identified.
         *
         * @return [OneTimeWorkRequest] representing the task for the [WorkManager] to enqueue and run
         */
        internal fun buildWorkRequest(): OneTimeWorkRequest = OneTimeWorkRequestBuilder<PingUploadWorker>()
            .addTag(PING_WORKER_TAG)
            .setConstraints(buildConstraints())
            .build()

        /**
         * Function to aid in properly enqueuing the worker in [WorkManager]
         */
        internal fun enqueueWorker() {
            WorkManager.getInstance().enqueueUniqueWork(
                PING_WORKER_TAG,
                ExistingWorkPolicy.KEEP,
                buildWorkRequest())
        }

        /**
         * Function to perform the actual ping upload task.  This is created here in the
         * companion object in order to facilitate testing.
         *
         * @return true if process was successful
         */
        internal fun uploadPings(): Boolean = process()

        /**
         * Function to cancel any pending ping upload workers
         */
        internal fun cancel() {
            WorkManager.getInstance().cancelUniqueWork(PING_WORKER_TAG)
        }

        /**
         * Function to deserialize and process all serialized ping files.  This function will ignore
         * files that don't match the UUID regex and just delete them to prevent files from polluting
         * the ping storage directory.
         *
         * @return Boolean representing the success of the upload task. This may be the value bubbled up
         *         from the callback, or if there was an error reading the files.
         */
        fun process(): Boolean {
            // This function is from PingsStorageEngine in glean-ac

            var success = true
            // TODO: 1551694 Get this directory from the rust side
            val storageDirectory = File(Glean.getDataDir(), PINGS_DIR)

            Log.d(LOG_TAG, "Processing persisted pings at ${storageDirectory.absolutePath}")

            synchronized(pingQueueLock) {
                storageDirectory.listFiles()?.forEach { file ->
                    if (file.name.matches(Regex(FILE_PATTERN))) {
                        Log.d(LOG_TAG, "Processing ping: ${file.name}")
                        if (!processFile(file)) {
                            Log.e(LOG_TAG, "Error processing ping file: ${file.name}")
                            success = false
                        }
                    } else {
                        // Delete files that don't match the UUID FILE_PATTERN regex
                        Log.d(LOG_TAG, "Pattern mismatch. Deleting ${file.name}")
                        file.delete()
                    }
                }
            }

            return success
        }

        /**
         * This function encapsulates processing of a single ping file.
         *
         * @param file The [File] to process
         *
         */
        @Suppress("ReturnCount")
        private fun processFile(
            file: File
        ): Boolean {
            // This function is from PingsStorageEngine in glean-ac

            var processed = false
            BufferedReader(FileReader(file)).use {
                try {
                    val path = it.readLine()
                    val serializedPing = it.readLine()

                    processed = serializedPing == null ||
                        Glean.httpClient.doUpload(path, serializedPing, Glean.configuration)
                } catch (e: FileNotFoundException) {
                    // This shouldn't happen after we queried the directory.
                    Log.e(LOG_TAG, "Could not find ping file ${file.name}")
                    return false
                } catch (e: IOException) {
                    // Something is not right.
                    Log.e(LOG_TAG, "IO Exception when reading file ${file.name}")
                    return false
                }
            }

            return if (processed) {
                val fileWasDeleted = file.delete()
                Log.d(LOG_TAG, "${file.name} was deleted: $fileWasDeleted")
                true
            } else {
                // The callback couldn't process this file.
                false
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
            !Glean.getUploadEnabled() -> Result.failure()
            !uploadPings() -> Result.retry()
            else -> Result.success()
        }
    }
}
