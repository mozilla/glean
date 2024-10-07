/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.delayMetricsPing
import mozilla.telemetry.glean.getContext
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import okhttp3.mockwebserver.MockWebServer
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
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
class RidealongPingTest {
    private val context = getContext()
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
    fun `sends a ride-along custom ping on baseline schedule`() {
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
                pingSchedule = mapOf("baseline" to listOf("custom-ping")),
            ),
            clearStores = true,
            uploadEnabled = true,
        )

        // Define a new custom ping inline.
        PingType<NoReasonCodes>(
            name = "custom-ping",
            includeClientId = true,
            sendIfEmpty = true,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = emptyList(),
        )

        Glean.handleBackgroundEvent()
        // Trigger it to upload
        triggerWorkManager(context)

        var request = server.takeRequest(2L, TimeUnit.SECONDS)!!
        var docType = request.path!!.split("/")[3]
        assertEquals("baseline", docType)

        request = server.takeRequest(2L, TimeUnit.SECONDS)!!
        docType = request.path!!.split("/")[3]
        assertEquals("custom-ping", docType)
    }
}
