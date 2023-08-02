/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore.pings

import android.content.Context
import android.content.Intent
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.test.rule.ServiceTestRule
import mozilla.telemetry.glean.testing.GleanTestLocalServer
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Ignore
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mozilla.samples.gleancore.SampleBackgroundProcess

// Note that this is ignored because for some reason the mock server
// doesn't seem to be able to catch the background ping when it is
// sent, despite the logs clearly stating that the ping is being sent
// and from testing with the Glean Debug View also clearly showing
// that the ping is being sent. See Bug 1846518 filed as a follow-up
// to this to try and fix this.
@RunWith(AndroidJUnit4::class)
@Ignore
class BackgroundPingTest {
    private val server = createMockWebServer()

    @get:Rule
    val serviceRule: ServiceTestRule = ServiceTestRule()

    @get:Rule
    val gleanRule = GleanTestLocalServer(context, server.port)

    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @Test
    fun validateBackgroundPing() {
        val serviceIntent = Intent(
            context,
            SampleBackgroundProcess::class.java,
        )
        serviceRule.startService(serviceIntent)

        val backgroundPing = waitForPingContent("background", "started", server)
        assertNotNull(backgroundPing)

        val metrics = backgroundPing?.getJSONObject("metrics")

        val counters = metrics?.getJSONObject("counter")
        assertTrue(counters?.getJSONObject("custom.bg_counter")?.getLong("value") == 1L)

        // Make sure there's no errors.
        val errors = metrics?.optJSONObject("labeled_counter")?.keys()
        errors?.forEach {
            assertFalse(it.startsWith("glean.error."))
        }
    }
}
