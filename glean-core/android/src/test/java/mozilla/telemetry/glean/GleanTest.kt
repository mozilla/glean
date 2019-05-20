/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.io.File
import java.util.concurrent.TimeUnit

@RunWith(RobolectricTestRunner::class)
class GleanTest {
    @Before
    fun setUp() {
        resetGlean()
    }

    @Test
    fun `send a ping`() {
        val server = getMockWebServer()
        val context: Context = ApplicationProvider.getApplicationContext()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        Glean.handleBackgroundEvent()
        // Make sure the file is on the disk
        val pingPath = File(context.applicationInfo.dataDir, "glean_data/pings")
        // Only the baseline ping should have been written
        assertEquals(1, pingPath.listFiles()?.size)

        // Now trigger it to upload
        triggerWorkManager()

        val requests: MutableMap<String, String> = mutableMapOf()
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("baseline", docType)
    }

    @Test
    fun `sending an empty ping doesn't queue work`() {
        Glean.sendPings(listOf(Pings.metrics))
        assertFalse(isWorkScheduled(PingUploadWorker.PING_WORKER_TAG))
    }
}
