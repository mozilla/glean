/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.getPlainBody
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.private.EventMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.NoExtraKeys
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.utils.tryGetLong
import mozilla.telemetry.glean.delayMetricsPing
import okhttp3.mockwebserver.MockWebServer
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.After
import org.junit.Before
import org.json.JSONObject
import java.util.concurrent.TimeUnit

/**
 * Testing behavior of custom pings.
 *
 * We already rely on the Rust side to test custom pings,
 * but this enables us to test the upload mechanism specifically.
 *
 * Even if this seemingly duplicates some of the testing, this should be kept around.
 */
@RunWith(AndroidJUnit4::class)
class CustomPingTest {
    private val context = getContextWithMockedInfo()
    private lateinit var server: MockWebServer

    @get:Rule
    val gleanRule = GleanTestRule(context)

    @Before
    fun setup() {
        server = getMockWebServer()
    }

    @After
    fun teardown() {
        server.shutdown()
    }

    @Test
    fun `sends empty custom ping`() {
        // a smoke test for custom pings

        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), clearStores = true, uploadEnabled = true)

        // Define a new custom ping inline.
        val customPing = PingType<NoReasonCodes>(
            name = "custom-ping",
            includeClientId = true,
            sendIfEmpty = true,
            reasonCodes = emptyList()
        )

        customPing.submit()
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("custom-ping", docType)
    }

    @Test
    fun `multiple pings in one go`() {
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), clearStores = true, uploadEnabled = true)

        // Define a new custom ping inline.
        val customPing = PingType<NoReasonCodes>(
            name = "custom-ping",
            includeClientId = true,
            sendIfEmpty = true,
            reasonCodes = emptyList()
        )

        // Trigger the ping twice.
        customPing.submit()
        customPing.submit()

        // Trigger work manager once.
        // This should launch one worker that handles all pending pings.
        triggerWorkManager(context)

        // Receive the first ping.
        var request = server.takeRequest(2L, TimeUnit.SECONDS)
        var docType = request.path.split("/")[3]
        assertEquals("custom-ping", docType)

        // Not much data in these pings,
        // but order should be preserved, so we can check the sequence number.

        var pingJson = JSONObject(request.getPlainBody())
        var pingInfo = pingJson.getJSONObject("ping_info")
        assertEquals(0L, pingInfo.tryGetLong("seq"))

        // Receive the second ping.
        request = server.takeRequest(2L, TimeUnit.SECONDS)
        docType = request.path.split("/")[3]
        assertEquals("custom-ping", docType)

        pingJson = JSONObject(request.getPlainBody())
        pingInfo = pingJson.getJSONObject("ping_info")
        assertEquals(1L, pingInfo.tryGetLong("seq")!!)
    }

    @Test
    fun `events for custom pings are flushed at startup`() {
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), clearStores = true, uploadEnabled = true)

        val pingName = "custom-events-1"

        // Define a 'click' event
        val click = EventMetricType<NoExtraKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf(pingName),
            allowedExtraKeys = listOf()
        )
        // and record it in the currently initialized Glean instance.
        click.record()

        // We need to simulate that the app is shutdown and all resources are freed.
        Glean.testDestroyGleanHandle()

        // Define a new custom ping inline.
        // This will register the ping as well.
        // Ususally this happens in user code by calling `Glean.registerPings(Pings)`
        @Suppress("UNUSED_VARIABLE")
        val customPing = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = emptyList()
        )

        // This is equivalent to a consumer calling `Glean.initialize` at startup
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), clearStores = true, uploadEnabled = true)

        // Trigger work manager once.
        // This should launch one worker that handles all pending pings.
        triggerWorkManager(context)

        // Receive the custom events ping.
        var request = server.takeRequest(2L, TimeUnit.SECONDS)
        var docType = request.path.split("/")[3]
        assertEquals(pingName, docType)

        val pingJson = JSONObject(request.getPlainBody())
        val events = pingJson.getJSONArray("events")
        assertEquals(1, events.length())

        val event = events.getJSONObject(0)
        val category = event.getString("category")
        val name = event.getString("name")
        assertEquals("ui.click", "$category.$name")
        assertEquals(0, event.getLong("timestamp"))
    }
}
