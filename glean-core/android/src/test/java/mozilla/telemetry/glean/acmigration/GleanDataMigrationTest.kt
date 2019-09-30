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
import mozilla.telemetry.glean.utils.getISOTimeString
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.util.*
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class GleanDataMigrationTest {
    // NOTE: do not use the Glean test rule in these tests, as they are testing
    // part of the initialization sequence.

    companion object {
        private const val TEST_CLIENT_ID = "94f94db0-fdf8-4bbc-943f-e43e6de1164f"
        private const val TEST_BASELINE_SEQ = 37
        private val TEST_FIRST_RUN_DATE = generateFirstRunDateWithOffset()

        private fun generateFirstRunDateWithOffset(): String {
            val cal = Calendar.getInstance()
            // Generate a first run day that is 7 days earlier than the
            // test run.
            cal.add(Calendar.DAY_OF_MONTH, -7)
            return getISOTimeString(cal, mozilla.telemetry.glean.private.TimeUnit.Day)
        }
    }

    private fun setFakeSequenceNumber(context: Context, pingName: String, number: Int) {
        val prefs = context.getSharedPreferences(
            GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME,
            Context.MODE_PRIVATE
        )

        prefs?.edit()?.putInt("${pingName}_seq", number)?.apply()
    }

    private fun setInitianlDataToMigrate(context: Context) {
        // Set a fake sequence number for the baseline ping.
        setFakeSequenceNumber(context, "baseline", TEST_BASELINE_SEQ)

        // Set a previously existing client id.
        context
            .getSharedPreferences(
                "${GleanACDataMigrator.GLEAN_AC_PACKAGE_NAME}.storages.UuidsStorageEngine",
                Context.MODE_PRIVATE
            )
            .edit()
            .putString("glean_client_info#client_id", TEST_CLIENT_ID)
            .apply()

        // Set a previously existing first_run_date.
        context
            .getSharedPreferences(
                "${GleanACDataMigrator.GLEAN_AC_PACKAGE_NAME}.storages.DatetimesStorageEngine",
                Context.MODE_PRIVATE
            )
            .edit()
            .putString("glean_client_info#first_run_date", TEST_FIRST_RUN_DATE)
            .apply()
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
        setInitianlDataToMigrate(context)

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
        assertEquals(TEST_BASELINE_SEQ, baselineJson.getJSONObject("ping_info")["seq"])
        assertEquals(TEST_CLIENT_ID, baselineJson.getJSONObject("client_info")["client_id"])
        assertEquals(TEST_FIRST_RUN_DATE, baselineJson.getJSONObject("client_info")["first_run_date"])
    }
}
