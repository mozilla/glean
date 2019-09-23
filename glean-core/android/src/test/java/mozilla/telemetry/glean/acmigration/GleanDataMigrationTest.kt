/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.triggerWorkManager
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class GleanDataMigrationTest {
    // NOTE: do not use the Glean test rule in these tests, as they are testing
    // part of the initialization sequence.

    private fun setFakeSequenceNumber(context: Context, pingName: String, number: Int) {
        val prefs = context.getSharedPreferences(
            GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME,
            Context.MODE_PRIVATE
        )

        prefs?.edit()?.putInt("${pingName}_seq", number)?.apply()
    }

    @Before
    fun setup() {
        // We're using the WorkManager in a bunch of places, and Glean will crash
        // in tests without this line.
        WorkManagerTestInitHelper.initializeTestWorkManager(
            ApplicationProvider.getApplicationContext())
    }

    @Test
    fun `Glean triggers the migration sequence if needed`() {
        val pingServer = getMockWebServer()

        val context = ApplicationProvider.getApplicationContext<Context>()

        // Make sure we did not migrate so that a new migration process starts.
        val migrator = GleanACDataMigrator(context)
        migrator.testResetMigrationStatus()

        // Set a fake sequence number for the baseline ping.
        setFakeSequenceNumber(context, "baseline", 37)

        // Start Glean and point it to a local ping server.
        Glean.resetGlean(
            context,
            Configuration().copy(
                serverEndpoint = "http://" + pingServer.hostName + ":" + pingServer.port,
                logPings = true
            ),
            true
        )

        // Trigger a baseline and a metrics ping.
        Pings.baseline.send()
        triggerWorkManager()

        // Get the pending pings from the ping server.
        val request = pingServer.takeRequest(20L, TimeUnit.SECONDS)

        // Check that we received the expected sequence number for the baseline ping.
        val baselineJson = JSONObject(request.body.readUtf8())
        assertEquals("baseline", baselineJson.getJSONObject("ping_info")["ping_type"])
        assertEquals(37, baselineJson.getJSONObject("ping_info")["seq"])
    }
}
