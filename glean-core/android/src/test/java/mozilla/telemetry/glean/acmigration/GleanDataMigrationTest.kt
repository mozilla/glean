/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration

import android.content.Context
import android.content.SharedPreferences
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.utils.getISOTimeString
import mozilla.telemetry.glean.utils.toList
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import java.util.Calendar
import java.util.TimeZone
import java.util.concurrent.TimeUnit

@RunWith(AndroidJUnit4::class)
class GleanDataMigrationTest {
    // NOTE: do not use the Glean test rule in these tests, as they are testing
    // part of the initialization sequence.

    companion object {
        private const val TEST_CLIENT_ID = "94f94db0-fdf8-4bbc-943f-e43e6de1164f"
        private const val TEST_BASELINE_SEQ = 37

        private fun generateFirstRunDateWithOffset(tz: TimeZone? = null): String {
            val cal = if (tz != null) {
                Calendar.getInstance(tz)
            } else {
                Calendar.getInstance()
            }

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

    /**
     * A convenience function to make `setInitialDataToMigrate` more readable.
     */
    private fun setSharedPrefsData(
        context: Context,
        storageName: String,
        putFunc: (SharedPreferences.Editor) -> SharedPreferences.Editor
    ) {
        context.getSharedPreferences(
                "${GleanACDataMigrator.GLEAN_AC_PACKAGE_NAME}.storages.$storageName",
                Context.MODE_PRIVATE)
            .edit()
            .let { putFunc(it) }
            .apply()
    }

    private fun setInitialDataToMigrate(context: Context, firstRunDate: String) {
        // Set a fake sequence number for the baseline ping.
        setFakeSequenceNumber(context, "baseline", TEST_BASELINE_SEQ)

        // Set a previously existing client id.
        setSharedPrefsData(context, "UuidsStorageEngine") {
            it.putString("glean_client_info#client_id", TEST_CLIENT_ID)
        }

        // Set a previously existing first_run_date.
        setSharedPrefsData(context, "DatetimesStorageEngine") {
            it.putString("glean_client_info#first_run_date", firstRunDate)
        }

        // Set some metrics in the baseline ping, for convenience.
        // Set a test boolean
        setSharedPrefsData(context, "BooleansStorageEngine") {
            it
                .putBoolean("baseline#test.glean.boolean", true)
                .putBoolean("baseline#test.glean.labeled_boolean_sample/label1", false)
                .putBoolean("baseline#test.glean.labeled_boolean_sample/label2", true)
                .putBoolean("baseline#test.glean.labeled_boolean_sample/label3", false)
        }
        // Set a test counter and some labels for a labeled_counter
        setSharedPrefsData(context, "CountersStorageEngine") {
            it
                .putInt("baseline#test.glean.counter", 10)
                .putInt("baseline#test.glean.labeled_counter_sample/label1", 1)
                .putInt("baseline#test.glean.labeled_counter_sample/label2", 2)
                .putInt("baseline#test.glean.labeled_counter_sample/label3", 3)
        }
        // Set a test string
        setSharedPrefsData(context, "StringsStorageEngine") {
            it
                .putString("baseline#test.glean.string", "Glean")
                .putString("baseline#test.glean.labeled_string_sample/label1", "some")
                .putString("baseline#test.glean.labeled_string_sample/label2", "random")
                .putString("baseline#test.glean.labeled_string_sample/label3", "stuff")
        }
        // Set a test stringlist
        setSharedPrefsData(context, "StringListsStorageEngine") {
            it.putString("baseline#test.glean.stringlist", "[\"a\",\"b\",\"c\"]")
        }
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
        val testFirstRunDate = generateFirstRunDateWithOffset()
        setInitialDataToMigrate(context, testFirstRunDate)

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
        assertEquals(testFirstRunDate, baselineJson.getJSONObject("client_info")["first_run_date"])

        val metrics = baselineJson.getJSONObject("metrics")

        assertEquals(true, metrics.getJSONObject("boolean").getBoolean("test.glean.boolean"))
        assertEquals(10, metrics.getJSONObject("counter").getInt("test.glean.counter"))
        assertEquals("Glean", metrics.getJSONObject("string").getString("test.glean.string"))
        val stringList = metrics.getJSONObject("string_list")
            .getJSONArray("test.glean.stringlist")
            .toList<String>()
        assertEquals("a", stringList[0])
        assertEquals("b", stringList[1])
        assertEquals("c", stringList[2])

        // Verify the labeled_counter data was ported over.
        val labeledCounterData = metrics
            .getJSONObject("labeled_counter")
            .getJSONObject("test.glean.labeled_counter_sample")
        assertNotNull(labeledCounterData)
        assertEquals(1, labeledCounterData.getInt("label1"))
        assertEquals(2, labeledCounterData.getInt("label2"))
        assertEquals(3, labeledCounterData.getInt("label3"))

        // Verify the labeled_boolean data was ported over.
        val labeledBooleanData = metrics
            .getJSONObject("labeled_boolean")
            .getJSONObject("test.glean.labeled_boolean_sample")
        assertNotNull(labeledBooleanData)
        assertEquals(false, labeledBooleanData.getBoolean("label1"))
        assertEquals(true, labeledBooleanData.getBoolean("label2"))
        assertEquals(false, labeledBooleanData.getBoolean("label3"))

        // Verify the labeled_string data was ported over.
        val labeledStringData = metrics
            .getJSONObject("labeled_string")
            .getJSONObject("test.glean.labeled_string_sample")
        assertNotNull(labeledStringData)
        assertEquals("some", labeledStringData.getString("label1"))
        assertEquals("random", labeledStringData.getString("label2"))
        assertEquals("stuff", labeledStringData.getString("label3"))
    }

    @Test
    fun `first_run_date is consistent across timezones when migrated`() {
        // Set a different system default for this test.
        TimeZone.setDefault(TimeZone.getTimeZone("America/New_York"))

        val pingServer = getMockWebServer()

        val context = ApplicationProvider.getApplicationContext<Context>()

        // Make sure we did not migrate so that a new migration process starts.
        val migrator = GleanACDataMigrator(context)
        migrator.testResetMigrationStatus()

        // Set a fake sequence number for the baseline ping.
        val testFirstRunDate = generateFirstRunDateWithOffset(TimeZone.getTimeZone("Europe/Berlin"))
        setInitialDataToMigrate(context, testFirstRunDate)

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
        assertEquals(testFirstRunDate, baselineJson.getJSONObject("client_info")["first_run_date"])
    }
}
