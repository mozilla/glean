package mozilla.telemetry.glean.scheduler

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import java.io.File
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.getWorkerStatus
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class DeletionPingTest {
    companion object {
        // This is the same deletion request ping directory as defined in `glean-core/src/lib.rs`.
        // We want to test interoperation between the Swift and Rust parts.
        private const val DELETION_PING_DIR: String = "deletion_request"
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
        val context = getContextWithMockedInfo()

        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ), clearStores = true, uploadEnabled = false)
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("deletion-request", docType)
    }

    @Test
    fun `deletion-request pings are only sent when toggled from on to off`() {
        val server = getMockWebServer()
        val context = getContextWithMockedInfo()

        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ), clearStores = true, uploadEnabled = true)

        // Get directory for pending deletion-request pings
        val pendingDeletionRequestDir = File(Glean.getDataDir(), DELETION_PING_DIR)

        // Disabling upload generates a deletion ping
        Glean.setUploadEnabled(false)
        triggerWorkManager(context)

        val request = server.takeRequest(2L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("deletion-request", docType)

        // File is deleted afterwards.
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)

        // Re-setting upload to `false` should not generate an additional ping
        // and no worker should be scheduled.
        Glean.setUploadEnabled(false)

        assertFalse(getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
        // No new file should have been written
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)
    }
}
