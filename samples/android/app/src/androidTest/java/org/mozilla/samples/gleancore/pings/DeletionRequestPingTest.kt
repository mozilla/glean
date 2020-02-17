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
import org.mozilla.samples.gleancore.R

import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mozilla.samples.gleancore.MainActivity
import androidx.test.uiautomator.UiDevice
import mozilla.telemetry.glean.testing.GleanTestLocalServer
import org.junit.Assert.assertNotEquals
import org.mozilla.samples.gleancore.getPingServerPort

@RunWith(AndroidJUnit4::class)
class DeletionRequestPingTest {
    @get:Rule
    val activityRule: ActivityTestRule<MainActivity> = ActivityTestRule(MainActivity::class.java)

    @get:Rule
    val gleanRule = GleanTestLocalServer(context, getPingServerPort())

    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

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

        // We might receive previous baseline or events ping, let's ignore that
        val deletionPing = waitForPingContent("deletion-request")!!

        // Validate the received data.

        var clientInfo = deletionPing.getJSONObject("client_info")
        val clientId = clientInfo.getString("client_id")
        assertNotEquals(clientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")

        // Try re-enabling and waiting for next baseline ping
        onView(withId(R.id.uploadSwitch)).perform(click())

        // Move it to background.
        device.pressHome()

        // Validate the received data.
        val baselinePing = waitForPingContent("baseline")!!

        clientInfo = baselinePing.getJSONObject("client_info")

        val newClientId = clientInfo.getString("client_id")
        assertNotEquals(newClientId, clientId)
        assertNotEquals(newClientId, "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0")
    }
}
