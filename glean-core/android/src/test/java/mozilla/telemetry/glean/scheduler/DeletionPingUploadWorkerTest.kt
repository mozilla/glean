package mozilla.telemetry.glean.scheduler

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import java.io.File
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanInternalAPI
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.getWorkerStatus
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
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
        Glean.setUploadEnabled(false)
        Glean.initialize(context, Configuration())

        assertTrue(getWorkerStatus(context, DeletionPingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }
}
