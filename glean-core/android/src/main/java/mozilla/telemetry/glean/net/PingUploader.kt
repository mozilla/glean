/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.config.Configuration
import java.io.BufferedReader
import java.io.File
import java.io.FileNotFoundException
import java.io.FileReader
import java.io.IOException
import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Locale
import java.util.TimeZone

/**
 * The interface defining how to send pings.
 */
internal interface PingUploader {
    companion object {
        // Since ping file names are UUIDs, this matches UUIDs for filtering purposes
        private const val FILE_PATTERN = "[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}"
        private const val LOG_TAG = "glean/PingUploader"
    }

    fun upload(path: String, data: String, config: Configuration): Boolean

    fun createDateHeaderValue(): String {
        val calendar = Calendar.getInstance()
        val dateFormat = SimpleDateFormat("EEE, dd MMM yyyy HH:mm:ss z", Locale.US)
        dateFormat.timeZone = TimeZone.getTimeZone("GMT")
        return dateFormat.format(calendar.time)
    }

    /**
     * Function to deserialize and process all serialized ping files.  This function will ignore
     * files that don't match the UUID regex and just delete them to prevent files from polluting
     * the ping storage directory.
     *
     * @param processingCallback Callback function to do the actual process action on the ping.
     *                           Typically this will be the [HttpPingUploader.upload] function.
     * @return Boolean representing the success of the upload task. This may be the value bubbled up
     *         from the callback, or if there was an error reading the files.
     */
    fun process(processingCallback: (String, String, Configuration) -> Boolean): Boolean {
        // This function is from PingsStorageEngine in glean-ac

        var success = true
        // TODO: 1551694 Get this directory from the rust side
        val storageDirectory = File(Glean.getDataDir(), "pings")

        Log.d(LOG_TAG, "Processing persisted pings at ${storageDirectory.absolutePath}")

        storageDirectory.listFiles()?.forEach { file ->
            if (file.name.matches(Regex(FILE_PATTERN))) {
                Log.d(LOG_TAG, "Processing ping: ${file.name}")
                if (!processFile(file, processingCallback)) {
                    Log.e(LOG_TAG, "Error processing ping file: ${file.name}")
                    success = false
                }
            } else {
                // Delete files that don't match the UUID FILE_PATTERN regex
                Log.d(LOG_TAG, "Pattern mismatch. Deleting ${file.name}")
                file.delete()
            }
        }

        return success
    }

    /**
     * This function encapsulates processing of a single ping file.
     *
     * @param file The [File] to process
     * @param processingCallback the callback that actually processes the file
     *
     */
    private fun processFile(
        file: File,
        processingCallback: (String, String, Configuration) -> Boolean
    ): Boolean {
        // This function is from PingsStorageEngine in glean-ac

        var processed = false
        BufferedReader(FileReader(file)).use {
            try {
                val path = it.readLine()
                val serializedPing = it.readLine()

                processed = serializedPing == null ||
                    processingCallback(path, serializedPing, Glean.configuration)
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
