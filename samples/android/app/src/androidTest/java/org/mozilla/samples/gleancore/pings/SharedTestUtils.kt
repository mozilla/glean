/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore.pings

import org.json.JSONObject
import org.mozilla.samples.gleancore.getPingServer
import java.util.concurrent.TimeUnit

/**
 * Waits for ping with the given name to be received
 * in the test ping server.
 *
 * @param pingName the name of the ping to wait for
 * @param maxAttempts the maximum number of attempts
 */
fun waitForPingContent(
    pingName: String,
    maxAttempts: Int = 3
): JSONObject?
{
    val server = getPingServer()

    var attempts = 0
    do {
        attempts += 1
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        if (pingName == docType) {
            return JSONObject(request.body.readUtf8())
        }
    } while (attempts < maxAttempts)

    return null
}
