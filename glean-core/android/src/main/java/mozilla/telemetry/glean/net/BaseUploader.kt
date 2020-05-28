/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import mozilla.telemetry.glean.config.Configuration

/**
 * The logic for uploading pings: this leaves the actual upload implementation
 * to the user-provided delegate.
 */
class BaseUploader(d: PingUploader) : PingUploader by d {
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
        return upload(
            url = config.serverEndpoint + path,
            data = data,
            headers = headers
        )
    }
}
