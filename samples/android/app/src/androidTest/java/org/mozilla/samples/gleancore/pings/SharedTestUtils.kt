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
import java.io.ByteArrayInputStream
import java.util.concurrent.TimeUnit

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
 * Waits for ping with the given name to be received
 * in the test ping server.
 *
 * @param pingName the name of the ping to wait for
 * @param maxAttempts the maximum number of attempts
 */
fun waitForPingContent(
    pingName: String,
    server: MockWebServer,
    maxAttempts: Int = 3
): JSONObject?
{
    var attempts = 0
    do {
        attempts += 1
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        if (pingName == docType) {
            return if (request.getHeader("Content-Encoding") == "gzip") {
                val bodyInBytes = ByteArrayInputStream(request.body.readByteArray()).readBytes()
                JSONObject(decompressGZIP(bodyInBytes))
            } else {
                JSONObject(request.body.readUtf8())
            }
        }
    } while (attempts < maxAttempts)

    return null
}
