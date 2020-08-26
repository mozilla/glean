/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.checkPingSchema
import mozilla.telemetry.glean.getPlainBody
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.getWorkerStatus
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.delayMetricsPing
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class PingTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `test sending of custom pings`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf()
        )

        val counter = CounterMetricType(
            disabled = false,
            category = "test",
            lifetime = Lifetime.Ping,
            name = "counter",
            sendInPings = listOf("custom")
        )

        counter.add()
        assertTrue(counter.testHasValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("custom", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])
        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings with snake_case`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val customPing = PingType<NoReasonCodes>(
            name = "custom_ping",
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf()
        )

        val counter = CounterMetricType(
            disabled = false,
            category = "test",
            lifetime = Lifetime.Ping,
            name = "counter",
            sendInPings = listOf("custom_ping")
        )

        counter.add()
        assertTrue(counter.testHasValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("custom_ping", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])
        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings with kebab-case`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val customPing = PingType<NoReasonCodes>(
            name = "custom-ping",
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf()
        )

        val counter = CounterMetricType(
            disabled = false,
            category = "test",
            lifetime = Lifetime.Ping,
            name = "counter",
            sendInPings = listOf("custom-ping")
        )

        counter.add()
        assertTrue(counter.testHasValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("custom-ping", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])
        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings without client_id`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = false,
            sendIfEmpty = false,
            reasonCodes = listOf()
        )

        val counter = CounterMetricType(
            disabled = false,
            category = "test",
            lifetime = Lifetime.Ping,
            name = "counter",
            sendInPings = listOf("custom")
        )

        counter.add()
        assertTrue(counter.testHasValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("custom", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNull(pingJson.getJSONObject("client_info").opt("client_id"))
        checkPingSchema(pingJson)
    }

    @Test
    fun `Sending a ping with an unknown name is a no-op`() {
        val server = getMockWebServer()

        val counter = CounterMetricType(
            disabled = false,
            category = "test",
            lifetime = Lifetime.Ping,
            name = "counter",
            sendInPings = listOf("unknown")
        )

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        counter.add()
        assertTrue(counter.testHasValue())

        Glean.submitPingByName("unknown")

        assertFalse("We shouldn't have any pings scheduled",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued
        )
    }

    @Test
    fun `Registry should contain built-in pings`() {
        assertTrue(Glean.testHasPingType("metrics"))
        assertTrue(Glean.testHasPingType("events"))
        assertTrue(Glean.testHasPingType("baseline"))
    }
}
