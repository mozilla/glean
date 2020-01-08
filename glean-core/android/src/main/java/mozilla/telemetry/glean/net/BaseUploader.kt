/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.BuildConfig
import mozilla.telemetry.glean.config.Configuration
import org.json.JSONException
import org.json.JSONObject
import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Locale
import java.util.TimeZone

/**
 * The logic for uploading pings: this leaves the actual upload implementation
 * to the user-provided delegate.
 */
class BaseUploader(d: PingUploader) : PingUploader by d {
    companion object {
        private const val LOG_TAG: String = "glean/BaseUploader"
        // Since the logcat ring buffer size is configurable, but it's 'max payload' size
        // is not, we must break apart long pings into chunks no larger than the max payload size
        // of 4076b.
        @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
        internal const val MAX_LOG_PAYLOAD_SIZE_BYTES = 4000

        /**
         * Function used to break apart large pings into an array of "chunks" that are compatible with
         * Logcat's max payload size
         */
        @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
        internal fun splitPingForLog(pingData: String, path: String): List<String> {
            // Calculate the total number of chunks
            var chunkCount = pingData.length / MAX_LOG_PAYLOAD_SIZE_BYTES
            if (pingData.length % MAX_LOG_PAYLOAD_SIZE_BYTES > 0) {
                chunkCount += 1
            }

            val chunks = mutableListOf<String>()

            for (curChunk in 0 until chunkCount) {
                // Calculate the start index of the current chunk.  We are only using 4000 here
                // instead of 4076 in order to leave room for a "message part" header.
                val chunkStartIndex = curChunk * MAX_LOG_PAYLOAD_SIZE_BYTES
                // Calculate the end index of the current chunk.
                // **Note: The endIndex is not inclusive of the last element (i.e. "up to but not
                //         including")
                val chunkEndIndex =
                    if (((curChunk + 1) * MAX_LOG_PAYLOAD_SIZE_BYTES) > pingData.length) {
                        // The current chunk is the last one and is not a full payload so grab the
                        // end index value.
                        pingData.length
                    } else {
                        // The current chunk is a full 4000 bytes.
                        (curChunk + 1) * MAX_LOG_PAYLOAD_SIZE_BYTES
                    }

                // Get the current message chunk from the indented JSON. In order to keep the
                // messages linked together, a "message x of n" will be appended to the tag
                val headerMsg = "Glean ping to URL: $path [Part ${curChunk + 1} of $chunkCount]"

                val curChunkContent = pingData.subSequence(
                    startIndex = chunkStartIndex,
                    endIndex = chunkEndIndex
                )

                chunks.add("$headerMsg\n$curChunkContent")
            }

            return chunks
        }
    }

    /**
     * Log the contents of a ping to the console.
     *
     * @param path the URL path to append to the server address
     * @param data the serialized text data to send
     */
    private fun logPing(path: String, data: String) {
        // Parse and reserialize the JSON so it has indentation and is human-readable.
        try {
            val json = JSONObject(data)
            val indented = json.toString(2)

            // If the length of the ping will fit within one logcat payload, then we can
            // short-circuit here and avoid some overhead, otherwise we must split up the
            // message so that we don't truncate it.
            if (indented.length + path.length <= MAX_LOG_PAYLOAD_SIZE_BYTES) {
                Log.d(LOG_TAG, "Glean ping to URL: $path\n$indented")
                return
            }

            val chunks = splitPingForLog(indented, path)

            for (chunk in chunks) {
                Log.d(LOG_TAG, chunk)
            }
        } catch (e: JSONException) {
            Log.d(LOG_TAG, "Exception parsing ping as JSON: $e") // $COVERAGE-IGNORE$
        }
    }

    /**
     * TEST-ONLY. Allows to set specific dates for testing.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun getCalendarInstance(): Calendar { return Calendar.getInstance() }

    /**
     * Generate a date string to be used in the Date header.
     */
    private fun createDateHeaderValue(): String {
        val calendar = getCalendarInstance()
        val dateFormat = SimpleDateFormat("EEE, dd MMM yyyy HH:mm:ss z", Locale.US)
        dateFormat.timeZone = TimeZone.getTimeZone("GMT")
        return dateFormat.format(calendar.time)
    }

    /**
     * Generate a list of headers to send with the request.
     *
     * @param config the Glean configuration object
     * @return a [HeadersList] containing String to String [Pair] with the first
     *         entry being the header name and the second its value.
     */
    private fun getHeadersToSend(config: Configuration): HeadersList {
        val headers = mutableListOf(
            Pair("Content-Type", "application/json; charset=utf-8"),
            Pair("User-Agent", config.userAgent),
            Pair("Date", createDateHeaderValue()),
            // Add headers for supporting the legacy pipeline.
            Pair("X-Client-Type", "Glean"),
            Pair("X-Client-Version", BuildConfig.LIBRARY_VERSION)
        )

        // If there is a pingTag set, then this header needs to be added in order to flag pings
        // for "debug view" use.
        config.pingTag?.let {
            headers.add(Pair("X-Debug-ID", it))
        }

        return headers
    }

    /**
     * This function triggers the actual upload: logs the ping and calls the implementation
     * specific upload function.
     *
     * @param path the URL path to append to the server address
     * @param data the serialized text data to send
     * @param config the Glean configuration object
     *
     * @return true if the ping was correctly dealt with (sent successfully
     *         or faced an unrecoverable error), false if there was a recoverable
     *         error callers can deal with.
     */
    internal fun doUpload(path: String, data: String, config: Configuration): Boolean {
        if (config.logPings) {
            logPing(path, data)
        }

        return upload(
            url = config.serverEndpoint + path,
            data = data,
            headers = getHeadersToSend(config)
        )
    }
}
