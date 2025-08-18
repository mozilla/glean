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
     * @param request the ping upload request, locked within a uploader capability check
     *
     * @return return the status code of the upload response,
     */
    fun upload(request: CapablePingUploadRequest): UploadResult
}

data class PingUploadRequest(
    val url: String,
    val data: ByteArray,
    val headers: HeadersList,
    val uploaderCapabilities: List<String>,
)

class CapablePingUploadRequest(
    val request: PingUploadRequest,
) {
    fun capable(f: (uploaderCapabilities: List<String>) -> Boolean): PingUploadRequest? {
        if (f(request.uploaderCapabilities)) {
            return request
        }
        return null
    }
}
