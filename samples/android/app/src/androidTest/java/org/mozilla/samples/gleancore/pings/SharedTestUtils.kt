/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore.pings

import mozilla.telemetry.glean.utils.decompressGZIP
import okhttp3.mockwebserver.Dispatcher
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import okhttp3.mockwebserver.RecordedRequest
import org.json.JSONObject
import java.io.BufferedReader
import java.io.ByteArrayInputStream
import java.util.concurrent.TimeUnit
import java.util.zip.GZIPInputStream

/**
 * Create a mock webserver that accepts all requests and replies with "OK".
 * @return a [MockWebServer] instance
 */
internal fun createMockWebServer(): MockWebServer {
    val server = MockWebServer()
    server.setDispatcher(object : Dispatcher() {
        override fun dispatch(request: RecordedRequest): MockResponse {
            return MockResponse().setBody("OK")
        }
    })
    return server
}

/**
 * Decompress the GZIP returned by the glean-core layer.
 *
 * @param data the gzipped [ByteArray] to decompress
 * @return a [String] containing the uncompressed data.
 */
fun decompressGZIP(data: ByteArray): String {
    return GZIPInputStream(ByteArrayInputStream(data)).bufferedReader().use(BufferedReader::readText)
}

/**
 * Convenience method to get the body of a request as a String.
 * The UTF8 representation of the request body will be returned.
 * If the request body is gzipped, it will be decompressed first.
 *
 * @return a [String] containing the body of the request.
 */
fun RecordedRequest.getPlainBody(): String {
    return if (this.getHeader("Content-Encoding") == "gzip") {
        val bodyInBytes = this.body.readByteArray()
        decompressGZIP(bodyInBytes)
    } else {
        this.body.readUtf8()
    }
}

/**
 * Waits for ping with the given name to be received
 * in the test ping server.
 *
 * @param pingName the name of the ping to wait for
 * @param maxAttempts the maximum number of attempts
 */
fun waitForPingContent(
    pingName: String,
    pingReason: String?,
    server: MockWebServer,
    maxAttempts: Int = 3
): JSONObject?
{
    var parsedPayload: JSONObject? = null
    for (attempts in 1..maxAttempts) {
        val request = server.takeRequest(20L, TimeUnit.SECONDS) ?: break
        val docType = request.path.split("/")[3]
        if (pingName == docType) {
            parsedPayload = JSONObject(request.getPlainBody())
            if (pingReason == null) {
                break
            }

            // If we requested a specific ping reason, look for it.
            val reason = parsedPayload.getJSONObject("ping_info").getString("reason")
            if (reason == pingReason) {
                break
            }
        }
    }

    return parsedPayload
}
