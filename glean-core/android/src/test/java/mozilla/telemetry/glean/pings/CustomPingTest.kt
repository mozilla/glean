package mozilla.telemetry.glean.scheduler

import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.utils.tryGetLong
import org.junit.Assert.assertEquals
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
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

    @get:Rule
    val gleanRule = GleanTestRule(context)

    @Test
    fun `sends empty custom ping`() {
        // a smoke test for custom pings

        val server = getMockWebServer()

        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
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
        val server = getMockWebServer()

        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
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

        var pingJson = JSONObject(request.body.readUtf8())
        var pingInfo = pingJson.getJSONObject("ping_info")
        assertEquals(0L, pingInfo.tryGetLong("seq"))

        // Receive the second ping.
        request = server.takeRequest(2L, TimeUnit.SECONDS)
        docType = request.path.split("/")[3]
        assertEquals("custom-ping", docType)

        pingJson = JSONObject(request.body.readUtf8())
        pingInfo = pingJson.getJSONObject("ping_info")
        assertEquals(1L, pingInfo.tryGetLong("seq")!!)
    }
}
