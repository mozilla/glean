/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore.pings

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.platform.app.InstrumentationRegistry
import androidx.test.ext.junit.rules.ActivityScenarioRule

import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mozilla.samples.gleancore.MainActivity
import androidx.test.uiautomator.UiDevice
import mozilla.telemetry.glean.testing.GleanTestLocalServer
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Assert.assertNotNull

@RunWith(AndroidJUnit4::class)
class BaselinePingTest {
    private val server = createMockWebServer()

    @get:Rule
    val activityRule: ActivityScenarioRule<MainActivity> = ActivityScenarioRule(MainActivity::class.java)

    @get:Rule
    val gleanRule = GleanTestLocalServer(context, server.port)

    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @Test
    fun validateBaselinePing() {
        // Wait for the app to be idle/ready.
        InstrumentationRegistry.getInstrumentation().waitForIdleSync()
        val device = UiDevice.getInstance(InstrumentationRegistry.getInstrumentation())
        device.waitForIdle()

        var baselinePing = waitForPingContent("baseline", "active", server)
        assertNotNull(baselinePing)

        // Wait for 1 second: this should guarantee we have some valid duration in the
        // ping.
        Thread.sleep(1000)

        // Move it to background.
        device.pressHome()

        // Validate the received data.
        baselinePing = waitForPingContent("baseline", "inactive", server)!!

        val metrics = baselinePing.getJSONObject("metrics")

        // Make sure we have a 'duration' field with a reasonable value: it should be >= 1, since
        // we slept for 1000ms.
        val timespans = metrics.getJSONObject("timespan")
        assertTrue(timespans.getJSONObject("glean.baseline.duration").getLong("value") >= 1L)

        // Make sure there's no errors.
        val errors = metrics.optJSONObject("labeled_counter")?.keys()
        errors?.forEach {
            assertFalse(it.startsWith("glean.error."))
        }
    }
}
