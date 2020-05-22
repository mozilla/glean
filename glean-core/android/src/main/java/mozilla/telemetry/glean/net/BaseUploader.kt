/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.utils.decompressGZIP
import org.json.JSONException
import org.json.JSONObject

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
            // Build the chunks with the header sequence messages
            val rawChunks = pingData.chunked(MAX_LOG_PAYLOAD_SIZE_BYTES)
            val chunks = mutableListOf<String>()
            for (curChunk in 0 until rawChunks.count()) {
                // In order to keep the messages linked together, a "message x of n" will be
                // appended to the message.
                val headerMsg =
                    "Glean ping to URL: $path [Part ${curChunk + 1} of ${rawChunks.count()}]\n"
                chunks.add(headerMsg + rawChunks.elementAt(curChunk))
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
     * This function triggers the actual upload: logs the ping and calls the implementation
     * specific upload function.
     *
     * @param path the URL path to append to the server address
     * @param data the serialized text data to send
     * @param headers the headers list for this request
     * @param config the Glean configuration object
     *
     * @return return the status code of the upload response or null in case unable to upload.
     */
    internal fun doUpload(path: String, data: ByteArray, headers: HeadersList, config: Configuration): UploadResult {
        val isGzip = !headers.none { (it.first == "Content-Encoding") && (it.second == "gzip") }
        if (config.logPings) {
            logPing(path, if (isGzip) decompressGZIP(data) else data.toString(Charsets.UTF_8))
        }

        return upload(
            url = config.serverEndpoint + path,
            data = data,
            headers = headers
        )
    }
}
