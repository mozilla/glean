/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

/**
 * The logic for uploading pings: this leaves the actual upload implementation
 * to the user-provided delegate.
 */
class BaseUploader(
    d: PingUploader,
) : PingUploader by d {
    /**
     * This function triggers the actual upload: logs the ping and calls the implementation
     * specific upload function.
     *
     * @param request the ping upload request, locked within a capability check
     *
     * @return return the status code of the upload response
     */
    internal fun doUpload(request: CapablePingUploadRequest): UploadResult = upload(request)
}
