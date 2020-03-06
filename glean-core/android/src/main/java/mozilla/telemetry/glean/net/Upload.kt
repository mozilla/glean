/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import android.util.Log
import org.json.JSONObject
import org.json.JSONException
import com.sun.jna.Structure
import com.sun.jna.Pointer
import com.sun.jna.Union
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.rust.getRustString

// Rust represents the upload task as an Enum
// and to go through the FFI that gets transformed into a tagged union.
// Each variant is represented as an 8-bit unsigned integer.
const val WAIT = 0
const val DONE = 2
const val UPLOAD = 1

@Structure.FieldOrder("tag", "uuid", "path", "body", "headers")
internal class UploadBody(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField val tag: Byte? = null,
    @JvmField val uuid: Pointer? = null,
    @JvmField val path: Pointer? = null,
    @JvmField val body: Pointer? = null,
    @JvmField val headers: Pointer? = null
) : Structure() {
    fun toPingRequest(): PingRequest {
        return PingRequest(
            this.uuid!!.getRustString(),
            this.path!!.getRustString(),
            this.body!!.getRustString(),
            this.headers!!.getRustString()
        )
    }
}

@Structure.FieldOrder("tag", "upload")
internal open class FfiPingUploadTask(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField var tag: Byte = DONE.toByte(),
    @JvmField var upload: UploadBody = UploadBody()
) : Union() {
    class ByValue : FfiPingUploadTask(), Structure.ByValue

    fun toPingUploadTask(): PingUploadTask {
        return when (this.tag.toInt()) {
            WAIT -> PingUploadTask.Wait
            UPLOAD -> {
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
    private val uuid: String,
    val path: String,
    val body: String,
    headers: String
) {
    val headers: HeadersList = headersFromJSONString(headers)

    private fun headersFromJSONString(str: String): HeadersList {
        val headers: MutableList<Pair<String, String>> = mutableListOf()
        try {
            val jsonHeaders = JSONObject(str)
            for (key in jsonHeaders.keys()) {
                headers.add(Pair(key, jsonHeaders.get(key).toString()))
            }
        } catch (e: JSONException) {
            // This JSON is created on the Rust side right before sending them over the FFI,
            // it's very unlikely that we get an exception here
            // unless there is some sort of memory corruption.
            Log.e(LOG_TAG, "Error while parsing headers for ping $uuid")
        }

        Glean.configuration.pingTag?.let {
            headers.add(Pair("X-Debug-ID", it))
        }

        return headers
    }

    companion object {
        private const val LOG_TAG: String = "glean/Upload"
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
}
