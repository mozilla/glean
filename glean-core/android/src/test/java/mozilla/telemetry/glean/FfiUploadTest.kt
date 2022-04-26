/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.ObsoleteCoroutinesApi
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.net.FfiPingUploadTask
import mozilla.telemetry.glean.net.HttpResponse
import mozilla.telemetry.glean.net.PingUploadTask
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@ObsoleteCoroutinesApi
@RunWith(AndroidJUnit4::class)
class FfiUploadTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(context)

    @After
    fun resetGlobalState() {
        Glean.setUploadEnabled(true)
    }

    /**
     * This test checks that the rate limiting works as expected:
     *
     * 1. 16 pings are submitted. The rate limit is 15 pings per minute, so we are one over.
     * 2. The first 15 pings are send without delay.
     * 3. On requesting the task for the 16th ping the upload manager asks the ping uploader
     *    to wait a specified amount of time.
     *    This time is less than a minute.
     *
     * Note:
     *   * This test does not wait for the full minute to expire to then get the final upload task.
     *   * We need to test the FFI boundary,
     *     which unfortunately requires to duplicate code that lives in the `PingUploadWorker`.
     */
    @Test
    fun `rate limiting instructs the uploader to wait shortly`() {
        delayMetricsPing(context)
        val server = getMockWebServer()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val customPing = PingType<NoReasonCodes>(
            name = "custom_ping",
            includeClientId = true,
            sendIfEmpty = true,
            reasonCodes = listOf()
        )

        // Submit pings until the rate limit
        for (i in 1..15) {
            customPing.submit()
        }

        // Send exactly one more
        customPing.submit()

        // Expect to upload the first 15 ones.
        for (i in 1..15) {
            val incomingTask = FfiPingUploadTask.ByReference()
            LibGleanFFI.INSTANCE.glean_get_upload_task(incomingTask)
            val action = incomingTask.toPingUploadTask()

            assertTrue("Task $i, expected Upload, was: ${action::class.qualifiedName}", action is PingUploadTask.Upload)

            // Mark as uploaded.
            val result = HttpResponse(200)
            LibGleanFFI.INSTANCE.glean_process_ping_upload_response(incomingTask, result.toFfi())
        }

        // Task 16 is throttled, so the uploader needs to wait

        val incomingTask = FfiPingUploadTask.ByReference()
        LibGleanFFI.INSTANCE.glean_get_upload_task(incomingTask)
        val action = incomingTask.toPingUploadTask()

        assertTrue("Next action is to wait", action is PingUploadTask.Wait)
        val waitTime = (action as PingUploadTask.Wait).time
        assertTrue("Waiting for more than 50s, was: $waitTime", waitTime > 50_000)
        assertTrue("Waiting for less than a minute, was: $waitTime", waitTime <= 60_000)
    }
}
