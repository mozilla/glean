package mozilla.telemetry.glean.scheduler

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import java.io.File
import mozilla.components.support.test.any
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanInternalAPI
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.getWorkerStatus
import mozilla.telemetry.glean.net.HeadersList
import mozilla.telemetry.glean.net.PingUploader
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.triggerWorkManager
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.anyString
import org.mockito.Mockito.spy
import org.mockito.Mockito.times
import org.mockito.Mockito.verify
import org.robolectric.shadows.ShadowLog

@RunWith(AndroidJUnit4::class)
class DeletionPingUploadWorkerTest {
    val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @After
    @Before
    fun cleanup() {
        ShadowLog.stream = System.out
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)
        WorkManagerTestInitHelper.initializeTestWorkManager(context)
    }

    @Test
    fun `pending deletion-request pings are sent on startup`() {
        val gleanDir = File(
            context.applicationInfo.dataDir,
            GleanInternalAPI.GLEAN_DATA_DIR
        )

        // Create directory for pending deletion-request pings
        val pendingDeletionRequestDir = File(gleanDir, DeletionPingUploadWorker.DELETION_PING_DIR)
        pendingDeletionRequestDir.mkdirs()

        // Write a deletion-request ping file
        val pingId = "b4e4ede0-8716-4691-a3fa-493c56c5be2d"
        val submitPath = "/submit/org-mozilla-samples-gleancore/deletion-request/1/$pingId"
        val content = "$submitPath\n{}"
        val pingFile = File(pendingDeletionRequestDir, pingId)
        assertTrue(pingFile.createNewFile())
        pingFile.writeText(content)

        assertFalse(getWorkerStatus(context, DeletionPingUploadWorker.PING_WORKER_TAG).isEnqueued)

        // Start Glean and let it pick up the file.
        // We can't use `resetGlean` because we need to disable upload.
        Glean.enableTestingMode()
        // Init Glean.
        Glean.testDestroyGleanHandle()
        Glean.initialize(context, false, Configuration())

        assertTrue(getWorkerStatus(context, DeletionPingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }

    /**
     * A stub uploader class that does not upload anything, but we can spy on it.
     */
    private class TestUploader : PingUploader {
        override fun upload(url: String, data: String, headers: HeadersList): Boolean {
            return true
        }
    }

    @Test
    fun `deletion-request pings are only sent when toggled from on to off`() {
        val httpClientSpy = spy(TestUploader())

        // Use the test client in the Glean configuration
        val config = Configuration(httpClient = httpClientSpy)
        resetGlean(context, config)

        // Get directory for pending deletion-request pings
        val pendingDeletionRequestDir = File(Glean.getDataDir(), DeletionPingUploadWorker.DELETION_PING_DIR)

        // Disabling upload generates a deletion ping
        Glean.setUploadEnabled(false)
        assertEquals(1, pendingDeletionRequestDir.listFiles()?.size)

        // Trigger the upload manager and check that the upload was started
        triggerWorkManager(context, DeletionPingUploadWorker.PING_WORKER_TAG)
        verify(httpClientSpy, times(1)).upload(anyString(), anyString(), any())

        // Re-setting upload to `false` should not generate an additional ping
        // and no worker should be scheduled.
        Glean.setUploadEnabled(false)

        assertFalse(getWorkerStatus(context, DeletionPingUploadWorker.PING_WORKER_TAG).isEnqueued)
        // Upload was definitely not triggered again
        verify(httpClientSpy, times(1)).upload(anyString(), anyString(), any())

        // No new file should have been written
        assertEquals(0, pendingDeletionRequestDir.listFiles()?.size)
    }
}
