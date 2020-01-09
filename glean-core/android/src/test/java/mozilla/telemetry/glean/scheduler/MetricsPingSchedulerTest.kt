/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.scheduler

import android.content.Context
import android.os.SystemClock
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.components.support.test.any
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.private.StringMetricType
import mozilla.telemetry.glean.private.TimeUnit
import mozilla.telemetry.glean.checkPingSchema
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.utils.getISOTimeString
import mozilla.telemetry.glean.utils.parseISOTimeString
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.anyBoolean
import org.mockito.Mockito.anyString
import org.mockito.Mockito.doReturn
import org.mockito.Mockito.eq
import org.mockito.Mockito.never
import org.mockito.Mockito.spy
import org.mockito.Mockito.times
import org.mockito.Mockito.verify
import org.mockito.Mockito.`when`
import java.util.Calendar
import java.util.concurrent.TimeUnit as AndroidTimeUnit

@RunWith(AndroidJUnit4::class)
class MetricsPingSchedulerTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @Before
    fun setup() {
        WorkManagerTestInitHelper.initializeTestWorkManager(context)

        Glean.enableTestingMode()
    }

    @Test
    fun `milliseconds until the due time must be correctly computed`() {
        val metricsPingScheduler = MetricsPingScheduler(context)

        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 3, 0, 0)

        // We expect the function to return 1 hour, in milliseconds.
        assertEquals(60 * 60 * 1000,
            metricsPingScheduler.getMillisecondsUntilDueTime(
                sendTheNextCalendarDay = false, now = fakeNow, dueHourOfTheDay = 4)
        )

        // If we're exactly at 4am, there must be no delay.
        fakeNow.set(2015, 6, 11, 4, 0, 0)
        assertEquals(0,
            metricsPingScheduler.getMillisecondsUntilDueTime(
                sendTheNextCalendarDay = false, now = fakeNow, dueHourOfTheDay = 4)
        )

        // Set the clock to after 4 of some minutes.
        fakeNow.set(2015, 6, 11, 4, 5, 0)

        // Since `sendTheNextCalendarDay` is false, this will be overdue, returning 0.
        assertEquals(0,
            metricsPingScheduler.getMillisecondsUntilDueTime(
                sendTheNextCalendarDay = false, now = fakeNow, dueHourOfTheDay = 4)
        )

        // With `sendTheNextCalendarDay` true, we expect the function to return 23 hours
        // and 55 minutes, in milliseconds.
        assertEquals(23 * 60 * 60 * 1000 + 55 * 60 * 1000,
            metricsPingScheduler.getMillisecondsUntilDueTime(
                sendTheNextCalendarDay = true, now = fakeNow, dueHourOfTheDay = 4)
        )
    }

    @Test
    fun `getDueTimeForToday must correctly return the due time for the current day`() {
        val mps = MetricsPingScheduler(context)

        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 3, 0, 0)

        val expected = Calendar.getInstance()
        expected.time = fakeNow.time
        expected.set(Calendar.HOUR_OF_DAY, 4)

        assertEquals(expected, mps.getDueTimeForToday(fakeNow, 4))

        // Let's check what happens at "midnight".
        fakeNow.set(2015, 6, 11, 0, 0, 0)
        assertEquals(expected, mps.getDueTimeForToday(fakeNow, 4))
    }

    @Test
    fun `isAfterDueTime must report false before the due time on the same calendar day`() {
        val mps = MetricsPingScheduler(context)

        val fakeNow = Calendar.getInstance()
        fakeNow.clear()

        // Shortly before.
        fakeNow.set(2015, 6, 11, 3, 0, 0)
        assertFalse(mps.isAfterDueTime(fakeNow, 4))

        // The same hour.
        fakeNow.set(2015, 6, 11, 4, 0, 0)
        assertFalse(mps.isAfterDueTime(fakeNow, 4))

        // Midnight.
        fakeNow.set(2015, 6, 11, 0, 0, 0)
        assertFalse(mps.isAfterDueTime(fakeNow, 4))
    }

    @Test
    fun `isAfterDueTime must report true after the due time on the same calendar day`() {
        val mps = MetricsPingScheduler(context)

        val fakeNow = Calendar.getInstance()
        fakeNow.clear()

        // Shortly after.
        fakeNow.set(2015, 6, 11, 4, 1, 0)
        assertTrue(mps.isAfterDueTime(fakeNow, 4))
    }

    @Test
    fun `getLastCollectedDate must report null when no stored date is available`() {
        val mps = MetricsPingScheduler(context)
        mps.sharedPreferences.edit().clear().apply()

        assertNull(
            "null must be reported when no date is stored",
            mps.getLastCollectedDate()
        )
    }

    @Test
    fun `getLastCollectedDate must report null when the stored date is corrupted`() {
        val mps = MetricsPingScheduler(context)
        mps.sharedPreferences
            .edit()
            .putLong(MetricsPingScheduler.LAST_METRICS_PING_SENT_DATETIME, 123L)
            .apply()

        // Wrong key type should trigger returning null.
        assertNull(
            "null must be reported when no date is stored",
            mps.getLastCollectedDate()
        )

        // Wrong date format string should trigger returning null.
        mps.sharedPreferences
            .edit()
            .putString(MetricsPingScheduler.LAST_METRICS_PING_SENT_DATETIME, "not-an-ISO-date")
            .apply()

        assertNull(
            "null must be reported when the date key is of unexpected format",
            mps.getLastCollectedDate()
        )
    }

    @Test
    fun `getLastCollectedDate must report the migrated a-c date, if available`() {
        val testDate = "2018-12-19T12:36:00-06:00"
        val mps = MetricsPingScheduler(
            context,
            testDate
        )

        // Wrong key type should trigger returning null.
        assertEquals(
            parseISOTimeString(testDate),
            mps.getLastCollectedDate()
        )
    }

    @Test
    fun `getLastCollectedDate must report the stored last collected date, if available`() {
        val testDate = "2018-12-19T12:36:00-06:00"
        val mps = MetricsPingScheduler(context)
        mps.updateSentDate(testDate)

        val expectedDate = parseISOTimeString(testDate)!!
        assertEquals(
            "The date the ping was collected must be reported",
            expectedDate,
            mps.getLastCollectedDate()
        )
    }

    @Test
    fun `collectMetricsPing must update the last sent date and reschedule the collection`() {
        val mpsSpy = spy(
            MetricsPingScheduler(context))

        // Ensure we have the right assumptions in place: the methods were not called
        // prior to |collectPingAndReschedule|.
        verify(mpsSpy, times(0)).updateSentDate(anyString())
        verify(mpsSpy, times(0)).schedulePingCollection(
            any(),
            anyBoolean()
        )

        mpsSpy.collectPingAndReschedule(Calendar.getInstance())

        // Verify that we correctly called in the methods.
        verify(mpsSpy, times(1)).updateSentDate(anyString())
        verify(mpsSpy, times(1)).schedulePingCollection(
            any(),
            anyBoolean()
        )
    }

    @Test
    fun `collectMetricsPing must correctly trigger the collection of the metrics ping`() {
        // Setup a test server and make Glean point to it.
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        resetGlean(context, Configuration(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        try {
            // Setup a testing metric and set it to some value.
            val testMetric = StringMetricType(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.Application,
                name = "string_metric",
                sendInPings = listOf("metrics")
            )

            val expectedValue = "test-only metric"
            testMetric.set(expectedValue)
            assertTrue("The initial test data must have been recorded", testMetric.testHasValue())

            // Manually call the function to trigger the collection.
            Glean.metricsPingScheduler.collectPingAndReschedule(Calendar.getInstance())

            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            // Fetch the ping from the server and decode its JSON body.
            val request = server.takeRequest(20L, AndroidTimeUnit.SECONDS)
            val metricsJsonData = request.body.readUtf8()
            val metricsJson = JSONObject(metricsJsonData)

            // Validate the received data.
            checkPingSchema(metricsJson)
            assertEquals("The received ping must be a 'metrics' ping",
                "metrics", metricsJson.getJSONObject("ping_info")["ping_type"])
            assertEquals(
                "The reported metric must contain the expected value",
                expectedValue,
                metricsJson.getJSONObject("metrics")
                    .getJSONObject("string")
                    .getString("telemetry.string_metric")
            )
        } finally {
            server.shutdown()
        }
    }

    @Test
    fun `startupCheck must immediately collect if the ping is overdue for today`() {
        // Set the current system time to a known datetime.
        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 7, 0, 0)

        // Set the last sent date to a previous day, so that today's ping is overdue.
        val mpsSpy =
            spy(MetricsPingScheduler(context))
        val overdueTestDate = "2015-07-05T12:36:00-06:00"
        mpsSpy.updateSentDate(overdueTestDate)

        verify(mpsSpy, never()).collectPingAndReschedule(any(), eq(true))

        // Make sure to return the fake date when requested.
        doReturn(fakeNow).`when`(mpsSpy).getCalendarInstance()

        // Trigger the startup check. We need to wrap this in `blockDispatchersAPI` since
        // the immediate startup collection happens in the Dispatchers.API context. If we
        // don't, test will fail due to async weirdness.
        mpsSpy.schedule()

        // And that we're storing the current date (this only reports the date, not the time).
        fakeNow.set(Calendar.HOUR_OF_DAY, 0)
        assertEquals(fakeNow.time, mpsSpy.getLastCollectedDate())

        // Verify that we're immediately collecting.
        verify(mpsSpy, times(1)).collectPingAndReschedule(fakeNow, true)
    }

    @Test
    fun `startupCheck must schedule collection for the next calendar day if collection already happened`() {
        // Set the current system time to a known datetime.
        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 7, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNow.timeInMillis)

        // Set the last sent date to now.
        val mpsSpy =
            spy(MetricsPingScheduler(context))

        // Inject the application version as already recorded, so we don't hit the case
        // where the ping is sent due to a version change.
        mpsSpy.isDifferentVersion()

        mpsSpy.updateSentDate(getISOTimeString(fakeNow, truncateTo = TimeUnit.Day))

        verify(mpsSpy, never()).schedulePingCollection(any(), anyBoolean())

        // Make sure to return the fake date when requested.
        doReturn(fakeNow).`when`(mpsSpy).getCalendarInstance()

        // Trigger the startup check.
        mpsSpy.schedule()

        // Verify that we're scheduling for the next day and not collecting immediately.
        verify(mpsSpy, times(1)).schedulePingCollection(fakeNow, sendTheNextCalendarDay = true)
        verify(mpsSpy, never()).schedulePingCollection(fakeNow, sendTheNextCalendarDay = false)
        verify(mpsSpy, never()).collectPingAndReschedule(any(), eq(true))
    }

    @Test
    fun `startupCheck must schedule collection for later today if it's before the due time`() {
        // Set the current system time to a known datetime.
        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 2, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNow.timeInMillis)

        // Set the last sent date to yesterday.
        val mpsSpy =
            spy(MetricsPingScheduler(context))

        // Inject the application version as already recorded, so we don't hit the case
        // where the ping is sent due to a version change.
        mpsSpy.isDifferentVersion()

        val fakeYesterday = Calendar.getInstance()
        fakeYesterday.time = fakeNow.time
        fakeYesterday.add(Calendar.DAY_OF_MONTH, -1)
        mpsSpy.updateSentDate(getISOTimeString(fakeYesterday, truncateTo = TimeUnit.Day))

        // Make sure to return the fake date when requested.
        doReturn(fakeNow).`when`(mpsSpy).getCalendarInstance()

        verify(mpsSpy, never()).schedulePingCollection(any(), anyBoolean())

        // Trigger the startup check.
        mpsSpy.schedule()

        // Verify that we're scheduling for today, but not collecting immediately.
        verify(mpsSpy, times(1)).schedulePingCollection(fakeNow, sendTheNextCalendarDay = false)
        verify(mpsSpy, never()).schedulePingCollection(fakeNow, sendTheNextCalendarDay = true)
        verify(mpsSpy, never()).collectPingAndReschedule(any(), eq(true))
    }

    @Test
    fun `startupCheck must correctly handle fresh installs (before due time)`() {
        // Set the current system time to a known datetime: before 4am local.
        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 3, 0, 0)

        // Clear the last sent date.
        val mpsSpy =
            spy(MetricsPingScheduler(context))
        mpsSpy.sharedPreferences.edit().clear().apply()
        // Inject the application version as already recorded, so we don't hit the case
        // where the ping is sent due to a version change.
        mpsSpy.isDifferentVersion()

        verify(mpsSpy, never()).collectPingAndReschedule(any(), anyBoolean())

        // Make sure to return the fake date when requested.
        doReturn(fakeNow).`when`(mpsSpy).getCalendarInstance()

        // Trigger the startup check.
        mpsSpy.schedule()

        // Verify that we're immediately collecting.
        verify(mpsSpy, never()).collectPingAndReschedule(fakeNow, true)
        verify(mpsSpy, times(1)).schedulePingCollection(fakeNow, sendTheNextCalendarDay = false)
    }

    @Test
    fun `startupCheck must correctly handle a version change`() {
        // Clear the last sent date.
        val mpsSpy =
            spy(MetricsPingScheduler(context))
        mpsSpy.sharedPreferences.edit().clear().apply()

        // Insert an old version identifier into shared preferences
        mpsSpy.sharedPreferences.edit()?.putString("last_version_of_app_used", "old version")?.apply()

        // Trigger the startup check.
        mpsSpy.schedule()

        // Verify that we're immediately collecting.
        verify(mpsSpy, times(1)).collectPingAndReschedule(any(), anyBoolean())
    }

    @Test
    fun `startupCheck must correctly handle fresh installs (after due time)`() {
        // Set the current system time to a known datetime: after 4am local.
        val fakeNow = Calendar.getInstance()
        fakeNow.clear()
        fakeNow.set(2015, 6, 11, 6, 0, 0)

        // Clear the last sent date.
        val mpsSpy =
            spy(MetricsPingScheduler(context))
        mpsSpy.sharedPreferences.edit().clear().apply()

        verify(mpsSpy, never()).collectPingAndReschedule(any(), anyBoolean())

        // Make sure to return the fake date when requested.
        doReturn(fakeNow).`when`(mpsSpy).getCalendarInstance()

        // Trigger the startup check.
        mpsSpy.schedule()

        // And that we're storing the current date (this only reports the date, not the time).
        fakeNow.set(Calendar.HOUR_OF_DAY, 0)
        assertEquals(
            "The scheduler must save the date the ping was collected",
            fakeNow.time,
            mpsSpy.getLastCollectedDate()
        )

        // Verify that we're immediately collecting.
        verify(mpsSpy, times(1)).collectPingAndReschedule(fakeNow, true)
        verify(mpsSpy, never()).schedulePingCollection(fakeNow, sendTheNextCalendarDay = false)
    }

    @Test
    fun `schedulePingCollection must correctly append a work request to the WorkManager`() {
        // Replacing the singleton's metricsPingScheduler here since doWork() refers to it when
        // the worker runs, otherwise we can get a lateinit property is not initialized error.
        Glean.metricsPingScheduler = MetricsPingScheduler(context)

        // No work should be enqueued at the beginning of the test.
        assertNull(Glean.metricsPingScheduler.timer)

        // Manually schedule a collection task for today.
        Glean.metricsPingScheduler.schedulePingCollection(Calendar.getInstance(), sendTheNextCalendarDay = false)

        // We expect the worker to be scheduled.
        assertNotNull(Glean.metricsPingScheduler.timer)

        resetGlean(clearStores = true)
    }

    @Test
    fun `cancel() correctly cancels worker`() {
        val context = ApplicationProvider.getApplicationContext<Context>()
        val mps = MetricsPingScheduler(context)

        mps.schedulePingCollection(Calendar.getInstance(), true)

        // Verify that the worker is enqueued
        assertNotNull("MetricsPingWorker is enqueued",
            Glean.metricsPingScheduler.timer)

        // Cancel the worker
        Glean.metricsPingScheduler.cancel()

        // Verify worker has been cancelled
        assertNull("MetricsPingWorker is not enqueued",
            Glean.metricsPingScheduler.timer)
    }

    @Test
    @Suppress("LongMethod")
    fun `Data recorded before Glean inits must not get into overdue pings`() {
        val context = getContextWithMockedInfo()

        // Reset Glean and do not start it right away.
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        // Let's create a fake time the metrics ping was sent: this is required for
        // us to not send a 'metrics' ping the first time we init glean.
        val fakeNowDoNotSend = Calendar.getInstance()
        fakeNowDoNotSend.clear()
        fakeNowDoNotSend.set(2015, 6, 11, 4, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNowDoNotSend.timeInMillis)

        // Create a fake instance of the metrics ping scheduler just to set the last
        // collection time.
        val fakeMpsSetter = spy(MetricsPingScheduler(context))
        fakeMpsSetter.updateSentDate(getISOTimeString(fakeNowDoNotSend, truncateTo = TimeUnit.Day))

        // Create a metric and set its value. We expect this to be sent in the ping that gets
        // generated the SECOND time we start glean.
        val expectedStringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "expected_metric",
            sendInPings = listOf("metrics")
        )
        val expectedValue = "must-exist-in-the-first-ping"

        // Reset Glean and start it for the FIRST time, then record a value.
        resetGlean(context)
        expectedStringMetric.set(expectedValue)

        // Destroy glean: it will retain the previously stored metric.
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        // Create a metric and attempt to record data before Glean is initialized. This
        // will be queued in the dispatcher.
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "canary_metric",
            sendInPings = listOf("metrics")
        )
        val canaryValue = "must-not-be-in-the-first-ping"
        stringMetric.set(canaryValue)

        // Set the current system time to a known datetime: this should make the metrics ping
        // overdue and trigger it at startup.
        val fakeNowTriggerPing = Calendar.getInstance()
        fakeNowTriggerPing.clear()
        fakeNowTriggerPing.set(2015, 6, 12, 7, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNowTriggerPing.timeInMillis)

        // Start the web-server that will receive the metrics ping.
        val server = getMockWebServer()

        try {
            // Initialize Glean the SECOND time: it will send the expected string metric (stored
            // from the previous run) but must not send the canary string, which would be sent
            // next time the 'metrics' ping is collected after this one.
            Glean.initialize(
                context,
                true,
                Configuration(
                    serverEndpoint = "http://" + server.hostName + ":" + server.port, logPings = true
                )
            )

            // Trigger worker task to upload the pings in the background.
            triggerWorkManager(context)

            // Wait for the metrics ping to be received.
            val request = server.takeRequest(20L, AndroidTimeUnit.SECONDS)

            val metricsJsonData = request.body.readUtf8()
            val pingJson = JSONObject(metricsJsonData)

            assertEquals("The received ping must be a 'metrics' ping",
                "metrics", pingJson.getJSONObject("ping_info")["ping_type"])
            assertFalse("The canary metric must not be present in this ping",
                metricsJsonData.contains("must-not-be-in-the-first-ping"))
            assertTrue("The expected metric must be in this ping",
                metricsJsonData.contains(expectedValue))
        } finally {
            server.shutdown()
        }
    }

    @Test
    fun `Glean must preserve lifetime application metrics across runs`() {
        // This test requires to use the glean instance (it's more an integration
        // test than a unit test).
        val context = getContextWithMockedInfo()

        // Reset Glean and do not start it right away.
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        // Let's create a fake time the metrics ping was sent: this is required for
        // us to not send a 'metrics' ping the first time we init glean.
        val fakeNowDoNotSend = Calendar.getInstance()
        fakeNowDoNotSend.clear()
        fakeNowDoNotSend.set(2015, 6, 11, 4, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNowDoNotSend.timeInMillis)

        // Create a fake instance of the metrics ping scheduler just to set the last
        // collection time.
        val fakeMpsSetter = spy(MetricsPingScheduler(context))
        fakeMpsSetter.updateSentDate(getISOTimeString(fakeNowDoNotSend, truncateTo = TimeUnit.Day))

        // Create a metric with lifetime: application and set it. Put
        // it in the metrics ping so that we can easily trigger it for
        // the purpose of this test.
        val testMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "test_applifetime_metric",
            sendInPings = listOf("metrics")
        )
        val expectedString = "I-will-survive!"

        // Reset Glean and start it for the FIRST time, then record a value.
        resetGlean(context)
        testMetric.set(expectedString)

        // Set the current system time to a known datetime: this should make the metrics ping
        // overdue and trigger it at startup.
        val fakeNowTriggerPing = Calendar.getInstance()
        fakeNowTriggerPing.clear()
        fakeNowTriggerPing.set(2015, 6, 12, 7, 0, 0)
        SystemClock.setCurrentTimeMillis(fakeNowTriggerPing.timeInMillis)

        // Start the web-server that will receive the metrics ping.
        val server = getMockWebServer()

        try {
            // Initialize Glean the SECOND time: we won't clear the stored data, we expect
            // the metric to be there and clear after the ping is generated.
            resetGlean(
                context,
                Configuration(
                    serverEndpoint = "http://" + server.hostName + ":" + server.port, logPings = true
                ),
                false
            )

            // Trigger worker task to upload the pings in the background.
            triggerWorkManager(context)

            // Wait for the metrics ping to be received.
            val request = server.takeRequest(20L, AndroidTimeUnit.SECONDS)

            val metricsJsonData = request.body.readUtf8()
            val pingJson = JSONObject(metricsJsonData)

            assertEquals("The received ping must be a 'metrics' ping",
                "metrics", pingJson.getJSONObject("ping_info")["ping_type"])
            assertTrue("The expected metric must be in this ping",
                metricsJsonData.contains(expectedString))
            assertFalse("The metric must be cleared after startup",
                testMetric.testHasValue())
        } finally {
            server.shutdown()
        }
    }

    // @Test
    // fun `Glean should close the measurement window for overdue pings before recording new data`() {
    //     // This test is a bit tricky: we want to make sure that, when our metrics ping is overdue
    //     // and collected at startup (if there's pending data), we don't mistakenly add new collected
    //     // data to it. In order to test for this specific edge case, we resort to the following:
    //     // record some data, then "pretend Glean is disabled" to simulate a crash, start using the
    //     // recording API off the main thread, init Glean in a separate thread and trigger a metrics
    //     // ping at startup. We expect the initially written data to be there ("expected_data!"), but
    //     // not the "should_not_be_recorded", which will be reported in a separate ping.

    //     // Create a string metric with a Ping lifetime.
    //     val stringMetric = StringMetricType(
    //         disabled = false,
    //         category = "telemetry",
    //         lifetime = Lifetime.Ping,
    //         name = "string_metric",
    //         sendInPings = listOf("metrics")
    //     )

    //     // Start Glean in the current thread, clean the local storage.
    //     resetGlean()

    //     // Record the data we expect to be in the final metrics ping.
    //     val expectedValue = "expected_data!"
    //     stringMetric.set(expectedValue)
    //     assertTrue("The initial expected data must be recorded", stringMetric.testHasValue())

    //     // Pretend Glean is disabled. This is used so that any API call will be discarded and
    //     // Glean will init again.
    //     Glean.initialized = false

    //     // Start the web-server that will receive the metrics ping.
    //     val server = getMockWebServer()

    //     // Set the current system time to a known datetime: this should make the metrics ping
    //     // overdue and trigger it at startup.
    //     val fakeNow = Calendar.getInstance()
    //     fakeNow.clear()
    //     fakeNow.set(2015, 6, 11, 7, 0, 0)
    //     SystemClock.setCurrentTimeMillis(fakeNow.timeInMillis)

    //     // Start recording to the metric asynchronously, from a separate thread. If something
    //     // goes wrong with our init, we should see the value set in the loop below in the triggered
    //     // "metrics" ping.
    //     var stopWrites = false
    //     val stringWriter = GlobalScope.async {
    //         do {
    //             stringMetric.set("should_not_be_recorded")
    //         } while (!stopWrites)
    //     }

    //     try {
    //         // Restart Glean in a separate thread to simulate a crash/restart without
    //         // clearing the local storage.
    //         val asyncInit = GlobalScope.async {
    //             Glean.initialize(getContextWithMockedInfo(), Configuration(
    //                 serverEndpoint = "http://" + server.hostName + ":" + server.port,
    //                 logPings = true
    //             ))

    //             // Trigger worker task to upload the pings in the background.
    //             triggerWorkManager()
    //         }

    //         // Wait for the metrics ping to be received.
    //         val request = server.takeRequest(20L, AndroidTimeUnit.SECONDS)

    //         // Stop recording to the test metric and wait for the async stuff
    //         // to complete.
    //         runBlocking {
    //             stopWrites = true
    //             stringWriter.await()
    //             asyncInit.await()
    //         }

    //         // Parse the received ping payload to a JSON object.
    //         val metricsJsonData = request.body.readUtf8()
    //         val metricsJson = JSONObject(metricsJsonData)

    //         // Validate the received data.
    //         checkPingSchema(metricsJson)
    //         assertEquals("The received ping must be a 'metrics' ping",
    //             "metrics", metricsJson.getJSONObject("ping_info")["ping_type"])
    //         assertEquals(
    //             "The reported metric must contain the expected value",
    //             expectedValue,
    //             metricsJson.getJSONObject("metrics")
    //                 .getJSONObject("string")
    //                 .getString("telemetry.string_metric")
    //         )
    //     } finally {
    //         server.shutdown()
    //     }
    // }
}
