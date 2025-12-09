/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.getContext
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.getWorkerStatus
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.utils.decompressGZIP
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.io.File
import java.util.concurrent.TimeUnit

/**
 * Testing correct behavior of the deletion ping.
 *
 * We already rely on the Rust side to do the right thing and delete pings at the right time.
 * We still want to test this from the Kotlin side, as this is an important core part of Glean.
 *
 * Even if this seemingly duplicates some of the testing, this should be kept around.
 */
@RunWith(AndroidJUnit4::class)
class DeletionPingTest {
    companion object {
        // These are the same ping directories as defined in `glean-core/src/lib.rs`.
        // We want to test interoperation between the Kotlin and Rust parts.
        private const val DELETION_PING_DIR: String = "deletion_request"
        private const val PENDING_PING_DIR: String = "pending_pings"
    }

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `pending deletion-request pings are sent on startup`() {
        // Create directory for pending deletion-request pings
        val pendingDeletionRequestDir = File(Glean.getDataDir(), DELETION_PING_DIR)
        pendingDeletionRequestDir.mkdirs()

        // Write a deletion-request ping file
        val pingId = "b4e4ede0-8716-4691-a3fa-493c56c5be2d"
        val submitPath = "/submit/org-mozilla-samples-gleancore/deletion-request/1/$pingId"
        val content = "$submitPath\n{}"
        val pingFile = File(pendingDeletionRequestDir, pingId)
        assertTrue(pingFile.createNewFile())
        pingFile.writeText(content)

        val server = getMockWebServer()
        val context = getContext()

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            clearStores = true,
            uploadEnabled = false,
        )
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("deletion-request", docType)
    }

    @Test
    fun `deletion-request pings are only sent when toggled from on to off`() {
        val server = getMockWebServer()
        val context = getContext()

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            clearStores = true,
            uploadEnabled = true,
        )

        // Get directory for pending deletion-request pings
        val pendingDeletionRequestDir = File(Glean.getDataDir(), DELETION_PING_DIR)

        // Disabling upload generates a deletion ping
        Glean.setCollectionEnabled(false)
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("deletion-request", docType)

        // File is deleted afterwards.
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)

        // Re-setting upload to `false` should not generate an additional ping
        // and no worker should be scheduled.
        Glean.setCollectionEnabled(false)

        assertFalse(getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
        // No new file should have been written
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)
    }

    @Test
    fun `non-deletion-pings are not uploaded when upload disabled`() {
        // Create directory for pending pings
        val pendingDeletionRequestDir = File(Glean.getDataDir(), DELETION_PING_DIR)
        pendingDeletionRequestDir.mkdirs()
        val pendingPingDir = File(Glean.getDataDir(), PENDING_PING_DIR)
        pendingPingDir.mkdirs()

        // We manually disable upload and we don't want the ID to be restored, or this will trigger another
        // deletion-request ping.
        val clientIdTxt = File(Glean.getDataDir(), "client_id.txt")
        assertTrue(clientIdTxt.delete())

        // Write a deletion-request ping file
        var deletionPingId = "775b6590-7f21-11ea-92e3-479998edf98c"
        var submitPath = "/submit/org-mozilla-samples-gleancore/deletion-request/1/$deletionPingId"
        var content = "$submitPath\n{}"
        var pingFile = File(pendingDeletionRequestDir, deletionPingId)
        assertTrue(pingFile.createNewFile())
        pingFile.writeText(content)

        // Write a baseline ping file
        var pingId = "899b0ab8-7f20-11ea-ac03-ff76f2a19f1c"
        submitPath = "/submit/org-mozilla-samples-gleancore/baseline/1/$pingId"
        content = "$submitPath\n{}"
        pingFile = File(pendingPingDir, pingId)
        assertTrue(pingFile.createNewFile())
        pingFile.writeText(content)

        val server = getMockWebServer()
        val context = getContext()

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            clearStores = true,
            uploadEnabled = false,
        )
        triggerWorkManager(context)

        var request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        var docType = request.path!!.split("/")[3]
        var docId = request.path!!.split("/")[5]
        assertEquals("Should have received a deletion-request ping", "deletion-request", docType)
        assertEquals("Should be the manually constructed ping", deletionPingId, docId)

        // deletion-request ping is gone
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)

        // Wait a bit to ensure no further ping is received.
        // Unfortunately this requires us to wait for the timeout.
        assertNull("Should not receive any further pings", server.takeRequest(2L, TimeUnit.SECONDS))

        // 'baseline' ping is removed from disk.
        assertEquals(0, pendingPingDir.listFiles()?.size)
    }

    @Test
    fun `deletion-request pings include experimentation id`() {
        val server = getMockWebServer()
        val context = getContext()

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
                experimentationId = "alpha-beta-gamma-delta",
            ),
            clearStores = true,
            uploadEnabled = true,
        )

        // Disabling upload generates a deletion ping
        Glean.setCollectionEnabled(false)
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("deletion-request", docType)

        val body = decompressGZIP(request.body.readByteArray())

        // Parse the body back into JSON
        val deletionPing = JSONObject(body)
        val metrics = deletionPing.getJSONObject("metrics")
        val strings = metrics.getJSONObject("string")
        val experimentationId = strings.getString("glean.client.annotation.experimentation_id")

        assertEquals("Experimentation ids must match", "alpha-beta-gamma-delta", experimentationId)
    }
}
