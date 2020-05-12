/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import mozilla.telemetry.glean.rust.Constants

/**
 * Store a list of headers as a String to String [Pair], with the first entry
 * being the header name and the second its value.
 */
typealias HeadersList = List<Pair<String, String>>

/**
 * The result of the ping upload.
 *
 * See below for the different possible cases.
 */
sealed class UploadResult {
    open fun toFfi(): Int {
        return Constants.UPLOAD_RESULT_UNRECOVERABLE
    }
}

/**
 * A HTTP response code.
 *
 * This can still indicate an error, depending on the status code.
 */
data class HttpResponse(val statusCode: Int) : UploadResult() {
    override fun toFfi(): Int {
        return Constants.UPLOAD_RESULT_HTTP_STATUS or statusCode
    }
}

/**
 * An unrecoverable upload failure.
 *
 * A possible cause might be a malformed URL.
 * The ping data is removed afterwards.
 */
object UnrecoverableFailure : UploadResult() {
    override fun toFfi(): Int {
        return Constants.UPLOAD_RESULT_UNRECOVERABLE
    }
}

/**
 * A recoverable failure.
 *
 * During upload something went wrong,
 * e.g. the network connection failed.
 * The upload should be retried at a later time.
 */
object RecoverableFailure : UploadResult() {
    override fun toFfi(): Int {
        return Constants.UPLOAD_RESULT_RECOVERABLE
    }
}

/**
 * The interface defining how to send pings.
 */
interface PingUploader {
    /**
     * Synchronously upload a ping to a server.
     *
     * @param url the URL path to upload the data to
     * @param data the serialized text data to send
     * @param headers a [HeadersList] containing the headers to add.
     *
     * @return return the status code of the upload response,
     *         or null in case upload could not be attempted at all.
     */
    fun upload(url: String, data: ByteArray, headers: HeadersList): UploadResult
}
