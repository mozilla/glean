/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore.pings

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.espresso.Espresso.onView
import androidx.test.espresso.action.ViewActions.click
import androidx.test.espresso.action.ViewActions.closeSoftKeyboard
import androidx.test.espresso.matcher.ViewMatchers.withId
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.rule.ActivityTestRule
import org.junit.Assert.assertEquals
import org.mozilla.samples.gleancore.R

import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mozilla.samples.gleancore.MainActivity
import org.mozilla.samples.gleancore.getPingServer
import androidx.test.uiautomator.UiDevice
import androidx.work.WorkInfo
import androidx.work.WorkManager
import androidx.work.testing.WorkManagerTestInitHelper
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import mozilla.telemetry.glean.testing.GleanTestLocalServer
import org.json.JSONObject
import org.junit.Assert.assertNotEquals
import org.junit.Before
import org.mozilla.samples.gleancore.getPingServerPort
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class DeletionRequestPingTest {
    @get:Rule
    val activityRule: ActivityTestRule<MainActivity> = ActivityTestRule(MainActivity::class.java)

    @get:Rule
    val gleanRule = GleanTestLocalServer(getPingServerPort())

    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @Before
    fun clearWorkManager() {
        WorkManagerTestInitHelper.initializeTestWorkManager(context)
    }

    private fun waitForPingContent(
        pingName: String,
        maxAttempts: Int = 3
    ): JSONObject?
    {
        val server = getPingServer()

        var attempts = 0
        do {
            attempts += 1
            val request = server.takeRequest(20L, TimeUnit.SECONDS)
            if (request == null) {
                continue
            }
            val docType = request.path.split("/")[3]
            if (pingName == docType) {
                return JSONObject(request.body.readUtf8())
            }
        } while (attempts < maxAttempts)

        return null
    }

    /**
     * Sadly, the WorkManager still requires us to manually trigger the upload job.
     * This function goes through all the active jobs by Glean (there should only be
     * one!) and triggers them.
     */
    private fun triggerEnqueuedUpload(tag: String) {
        // The tag is really internal to PingUploadWorker, but we can't do much more
        // than copy-paste unless we want to increase our API surface.
        val reasonablyHighCITimeoutMs = 5000L

        runBlocking {
            withTimeout(reasonablyHighCITimeoutMs) {
                do {
                    val workInfoList = WorkManager.getInstance(context).getWorkInfosByTag(tag).get()
                    workInfoList.forEach { workInfo ->
                        if (workInfo.state === WorkInfo.State.ENQUEUED) {
                            // Trigger WorkManager using TestDriver
                            val testDriver = WorkManagerTestInitHelper.getTestDriver(context)
                            testDriver?.setAllConstraintsMet(workInfo.id)
                            return@withTimeout
                        }
                    }
                } while (true)
            }
        }
    }

    @Test
    fun validateDeletionRequestPing() {
        // Wait for the app to be idle/ready.
        InstrumentationRegistry.getInstrumentation().waitForIdleSync()
        val device = UiDevice.getInstance(InstrumentationRegistry.getInstrumentation())
        device.waitForIdle()

        // Disable upload by toggline the switch
        onView(withId(R.id.uploadSwitch))
            .perform(closeSoftKeyboard())
            .perform(click())

        // Wait for the upload job to be present and trigger it.
        Thread.sleep(1000) // FIXME: for some reason, without this, WorkManager won't find the job
        triggerEnqueuedUpload("mozac_service_glean_deletion_ping_upload_worker")

        // Validate the received data.
        val deletionPing = waitForPingContent("deletion_request")!!
        assertEquals("deletion_request", deletionPing.getJSONObject("ping_info")["ping_type"])

        var clientInfo = deletionPing.getJSONObject("client_info")
        val clientId = clientInfo.getString("client_id")
        assertNotEquals(clientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")

        // Try re-enabling and waiting for next baseline ping
        onView(withId(R.id.uploadSwitch)).perform(click())

        // Move it to background.
        device.pressHome()

        // Wait for the upload job to be present and trigger it.
        Thread.sleep(1000) // FIXME: for some reason, without this, WorkManager won't find the job
        triggerEnqueuedUpload("mozac_service_glean_ping_upload_worker")

        // Validate the received data.
        val baselinePing = waitForPingContent("baseline")!!
        assertEquals("baseline", baselinePing.getJSONObject("ping_info")["ping_type"])

        clientInfo = baselinePing.getJSONObject("client_info")

        val newClientId = clientInfo.getString("client_id")
        assertNotEquals(newClientId, clientId)
        assertNotEquals(newClientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")
    }
}
