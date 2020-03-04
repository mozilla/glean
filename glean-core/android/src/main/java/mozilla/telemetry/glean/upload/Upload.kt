/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.upload

import com.sun.jna.Structure
import com.sun.jna.Pointer
import com.sun.jna.Union

@Structure.FieldOrder("tag", "uuid", "path", "body", "headers")
internal class Upload_Body(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField val tag: Byte? = null,
    @JvmField val uuid: Pointer? = null,
    @JvmField val path: Pointer? = null,
    @JvmField val body: Pointer? = null,
    @JvmField val headers: Pointer? = null
) : Structure() {
    fun toPingRequest(): PingRequest {
        return PingRequest(
            this.path!!.getRustString(),
            this.body!!.getRustString(),
            this.headers!!.getRustString()
        )
    }
}

@Structure.FieldOrder("tag", "upload")
internal open class FfiPingUploadTask(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField var tag: Byte = 2,
    @JvmField var upload: Upload_Body = Upload_Body()
) : Union() {
    class ByValue : FfiPingUploadTask(), Structure.ByValue
}
