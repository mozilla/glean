/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

/**
 * Store a list of headers as a String to String [Pair], with the first entry
 * being the header name and the second its value.
 */
typealias HeadersList = Map<String, String>

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
