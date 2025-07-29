/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.os.Build
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.checkPingSchema
import mozilla.telemetry.glean.delayMetricsPing
import mozilla.telemetry.glean.getContext
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.getPlainBody
import mozilla.telemetry.glean.getWorkerStatus
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotEquals
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

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = true,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("custom"),
            ),
        )

        counter.add()
        assertEquals(1, counter.testGetValue())

        var callbackWasCalled = false
        customPing.testBeforeNextSubmit { reason ->
            assertNull(reason)
            assertEquals(1, counter.testGetValue())
            callbackWasCalled = true
        }

        customPing.submit()
        assertTrue(callbackWasCalled)

        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("custom", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])

        // This is not "Android" when testing locally,
        // so we just check that it is not Unknown.
        assertNotEquals("Unknown", pingJson.getJSONObject("client_info")["os"])
        assertEquals(Build.VERSION.RELEASE, pingJson.getJSONObject("client_info")["os_version"])
        assertNotNull(pingJson.getJSONObject("client_info")["android_sdk_version"])
        assertNotNull(pingJson.getJSONObject("client_info")["device_model"])
        assertNotNull(pingJson.getJSONObject("client_info")["device_manufacturer"])
        assertNotNull(pingJson.getJSONObject("client_info")["locale"])

        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings with snake_case`() {
        val server = getMockWebServer()

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        val customPing = PingType<NoReasonCodes>(
            name = "custom_ping",
            includeClientId = true,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("custom_ping"),
            ),
        )

        counter.add()
        assertEquals(1, counter.testGetValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("custom_ping", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])
        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings with kebab-case`() {
        val server = getMockWebServer()

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        val customPing = PingType<NoReasonCodes>(
            name = "custom-ping",
            includeClientId = true,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("custom-ping"),
            ),
        )

        counter.add()
        assertEquals(1, counter.testGetValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("custom-ping", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNotNull(pingJson.getJSONObject("client_info")["client_id"])
        checkPingSchema(pingJson)
    }

    @Test
    fun `test sending of custom pings without client_id`() {
        val server = getMockWebServer()

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = false,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("custom"),
            ),
        )

        counter.add()
        assertEquals(1, counter.testGetValue())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("custom", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNull(pingJson.getJSONObject("client_info").opt("client_id"))
        checkPingSchema(pingJson)
    }

    @Test
    fun `test pings that exclude client_id also excludes experimentation id`() {
        val server = getMockWebServer()

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
                experimentationId = "alpha-beta-gamma-delta",
            ),
        )

        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = false,
            sendIfEmpty = false,
            preciseTimestamps = true,
            schedulesPings = emptyList(),
            includeInfoSections = true,
            enabled = true,
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("custom"),
            ),
        )

        counter.add()
        assertEquals(1, counter.testGetValue())

        assertEquals("alpha-beta-gamma-delta", Glean.testGetExperimentationId())

        customPing.submit()
        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("custom", docType)

        val pingJson = JSONObject(request.getPlainBody())
        assertNull(pingJson.getJSONObject("client_info").opt("client_id"))
        assertNull(
            pingJson.optJSONObject("metrics")
                ?.optJSONObject("string")
                ?.opt("glean.client.annotation.experimentation_id"),
        )
        checkPingSchema(pingJson)
    }

    @Test
    fun `Sending a ping with an unknown name is a no-op`() {
        val server = getMockWebServer()

        val counter = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test",
                lifetime = Lifetime.PING,
                name = "counter",
                sendInPings = listOf("unknown"),
            ),
        )

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        // Recording to an unknown ping won't record anything.
        counter.add()
        assertNull(counter.testGetValue())

        // We might have some work queued by init that we'll need to clear.
        triggerWorkManager(context)

        Glean.submitPingByName("unknown")

        assertFalse(
            "We shouldn't have any pings scheduled",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued,
        )
    }
}
