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
import mozilla.telemetry.glean.rust.getRustString
import mozilla.telemetry.glean.rust.RustBuffer

// Rust represents the upload task as an Enum
// and to go through the FFI that gets transformed into a tagged union.
// Each variant is represented as an 8-bit unsigned integer.
//
// This *MUST* have the same order as the variants in `glean-core/ffi/src/upload.rs`.
enum class UploadTaskTag {
    Upload,
    Wait,
    Done
}

@Structure.FieldOrder("tag", "documentId", "path", "body", "headers")
internal class UploadBody(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField val tag: Byte = UploadTaskTag.Upload.ordinal.toByte(),
    @JvmField val documentId: Pointer? = null,
    @JvmField val path: Pointer? = null,
    @JvmField var body: RustBuffer = RustBuffer(),
    @JvmField val headers: Pointer? = null
) : Structure() {
    fun toPingRequest(): PingRequest {
        return PingRequest(
            this.documentId!!.getRustString(),
            this.path!!.getRustString(),
            this.body.getByteArray(),
            this.headers!!.getRustString()
        )
    }
}

@Structure.FieldOrder("tag", "time")
internal class WaitBody(
    @JvmField var tag: Byte = UploadTaskTag.Wait.ordinal.toByte(),
    @JvmField var time: Long = 60_000
) : Structure()

internal open class FfiPingUploadTask(
    // NOTE: We need to provide defaults here, so that JNA can create this object.
    @JvmField var tag: Byte = UploadTaskTag.Done.ordinal.toByte(),
    @JvmField var upload: UploadBody = UploadBody(),
    @JvmField var wait: WaitBody = WaitBody()
) : Union() {
    class ByReference : FfiPingUploadTask(), Structure.ByReference

    init {
        // Initialize to be the `tag`-only variant
        setType("tag")
    }

    fun toPingUploadTask(): PingUploadTask {
        return when (this.tag.toInt()) {
            UploadTaskTag.Wait.ordinal -> {
                this.readField("wait")
                PingUploadTask.Wait(this.wait.time)
            }
            UploadTaskTag.Upload.ordinal -> {
                this.readField("upload")
                PingUploadTask.Upload(this.upload.toPingRequest())
            }
            else -> PingUploadTask.Done
        }
    }
}

/**
 * Represents a request to upload a ping.
 */
internal class PingRequest(
    private val documentId: String,
    val path: String,
    val body: ByteArray,
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
            Log.e(LOG_TAG, "Error while parsing headers for ping $documentId")
        }

        return headers
    }

    companion object {
        private const val LOG_TAG: String = "glean/Upload"
    }
}

/**
 * When asking for the next ping request to upload,
 * the requester may receive one out of three possible tasks.
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
    data class Wait(val time: Long) : PingUploadTask()

    /**
     * A flag signaling that requester doesn't need to request any more upload tasks at this moment.
     *
     * There are two possibilities for this scenario:
     * * Pending pings queue is empty, no more pings to request;
     * * Requester has reported more max recoverable upload failures on the same uploading_window[1]
     *   and should stop requesting at this moment.
     *
     * [1]: An "uploading window" starts when a requester gets a new `PingUploadTask::Upload(PingRequest)`
     *      response and finishes when they finally get a `PingUploadTask::Done` or `PingUploadTask::Wait` response.
     */
    object Done : PingUploadTask()
}
