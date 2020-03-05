/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.upload

import org.json.JSONObject
import com.sun.jna.Structure
import com.sun.jna.Pointer
import com.sun.jna.Union
import mozilla.telemetry.glean.net.HeadersList
import mozilla.telemetry.glean.rust.getRustString

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

    fun toPingUploadTask(): PingUploadTask {
        return when (this.tag.toInt()) {
            0 -> PingUploadTask.Wait
            1 -> {
                this.readField("upload")
                PingUploadTask.Upload(this.upload.toPingRequest())
            }
            else -> PingUploadTask.Done
        }
    }
}

/**
 * The PingUploadTask makes it easier to consume Upload_Body.
 */
internal class PingRequest(
    val path: String,
    val body: String,
    headers: String
) {
    val headers: HeadersList = headersFromJSONString(headers)

    companion object {
        fun headersFromJSONString(str: String): HeadersList {
            val jsonHeaders = JSONObject(str)
            val headers: MutableList<Pair<String, String>> = mutableListOf()
            for (key in jsonHeaders.keys()) {
                headers.add(Pair(key, jsonHeaders.get(key).toString()))
            }
            return headers
        }
    }
}

/**
 * The PingUploadTask makes it easier to consume FfiPingUploadTask.
 */
internal sealed class PingUploadTask {
    /**
     * A PingRequest popped from the front of the queue.
     */
    class Upload(val request: PingRequest) : PingUploadTask()

    /**
     * A flag signaling that the pending pings directories are not done being processed,
     * thus the requester should wait and come back later.
     */
    object Wait : PingUploadTask()

    /**
     * A flag signaling that the pending pings queue is empty and requester is done.
     */
    object Done : PingUploadTask()

    fun request(): PingRequest? = when (this) {
        is Upload -> this.request
        else -> null
    }
}
