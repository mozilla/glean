/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.LifecycleRegistry
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.Dispatchers as KotlinDispatchers
import kotlinx.coroutines.ObsoleteCoroutinesApi
import kotlinx.coroutines.runBlocking
import mozilla.telemetry.glean.GleanMetrics.GleanError
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.private.CounterMetricType
import mozilla.telemetry.glean.private.EventMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.NoExtraKeys
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.private.StringMetricType
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.DeletionPingUploadWorker
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.utils.getLanguageFromLocale
import mozilla.telemetry.glean.utils.getLocaleTag
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.mock
import org.mockito.Mockito.spy
import org.robolectric.shadows.ShadowProcess
import java.io.BufferedReader
import java.io.File
import java.io.FileReader
import java.util.Calendar
import java.util.Locale
import java.util.concurrent.TimeUnit

@ObsoleteCoroutinesApi
@RunWith(AndroidJUnit4::class)
class GleanTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(context)

    @After
    fun resetGlobalState() {
        Glean.setUploadEnabled(true)
    }

    @Test
    fun `setting uploadEnabled before initialization should not crash`() {
        // Can't use resetGlean directly
        Glean.testDestroyGleanHandle()

        val config = Configuration()

        Glean.initialize(context, true, config)
    }

    @Test
    fun `getting uploadEnabled before initialization should not crash`() {
        // Can't use resetGlean directly
        Glean.testDestroyGleanHandle()

        val config = Configuration()

        Glean.setUploadEnabled(true)
        assertTrue(Glean.getUploadEnabled())

        Glean.initialize(context, true, config)
        assertTrue(Glean.getUploadEnabled())
    }

    // New from glean-core.
    @Test
    fun `send a ping`() {
        val server = getMockWebServer()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        Glean.handleBackgroundEvent()
        // Make sure the file is on the disk
        val pingPath = File(Glean.getDataDir(), PingUploadWorker.PINGS_DIR)
        // Only the baseline ping should have been written
        assertEquals(1, pingPath.listFiles()?.size)

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("baseline", docType)
    }

    @Test
    fun `sending an empty ping doesn't queue work`() {
        Glean.submitPings(listOf(Pings.metrics))
        assertFalse(getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }

    // Tests from glean-ac (706af1f).

    @Test
    fun `disabling upload should disable metrics recording`() {
        val stringMetric = StringMetricType(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.Application,
                name = "string_metric",
                sendInPings = listOf("store1")
        )
        Glean.setUploadEnabled(false)
        assertEquals(false, Glean.getUploadEnabled())
        stringMetric.set("foo")
        assertFalse(stringMetric.testHasValue())
    }

    @Test
    fun `test experiments recording`() {
        Glean.setExperimentActive(
            "experiment_test", "branch_a"
        )
        Glean.setExperimentActive(
            "experiment_api", "branch_b",
            mapOf("test_key" to "value")
        )
        assertTrue(Glean.testIsExperimentActive("experiment_api"))
        assertTrue(Glean.testIsExperimentActive("experiment_test"))

        Glean.setExperimentInactive("experiment_test")

        assertTrue(Glean.testIsExperimentActive("experiment_api"))
        assertFalse(Glean.testIsExperimentActive("experiment_test"))

        val storedData = Glean.testGetExperimentData("experiment_api")
        assertEquals("branch_b", storedData.branch)
        assertEquals(1, storedData.extra?.size)
        assertEquals("value", storedData.extra?.getValue("test_key"))
    }

    @Test
    fun `test experiments recording before Glean inits`() {
        // This test relies on Glean not being initialized and task queuing to be on.
        Glean.testDestroyGleanHandle()
        Dispatchers.API.setTaskQueueing(true)

        Glean.setExperimentActive(
            "experiment_set_preinit", "branch_a"
        )

        Glean.setExperimentActive(
            "experiment_preinit_disabled", "branch_a"
        )

        Glean.setExperimentInactive("experiment_preinit_disabled")

        // This will init glean and flush the dispatcher's queue.
        resetGlean()

        assertTrue(Glean.testIsExperimentActive("experiment_set_preinit"))
        assertFalse(Glean.testIsExperimentActive("experiment_preinit_disabled"))
    }

    @Test
    fun `test sending of background pings`() {
        val server = getMockWebServer()

        val click = EventMetricType<NoExtraKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("events")
        )

        val context = getContextWithMockedInfo()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        // Fake calling the lifecycle observer.
        val lifecycleOwner = mock(LifecycleOwner::class.java)
        val lifecycleRegistry = LifecycleRegistry(lifecycleOwner)
        val gleanLifecycleObserver = GleanLifecycleObserver(context)
        lifecycleRegistry.addObserver(gleanLifecycleObserver)

        try {
            // Simulate the first foreground event after the application starts.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START)
            click.record()

            // Simulate going to background.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_STOP)

            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            val requests = mutableMapOf<String, String>()
            for (i in 0..1) {
                val request = server.takeRequest(20L, TimeUnit.SECONDS)
                val docType = request.path.split("/")[3]
                requests[docType] = request.body.readUtf8()
            }

            val eventsJson = JSONObject(requests["events"]!!)
            checkPingSchema(eventsJson)
            assertEquals("events", eventsJson.getJSONObject("ping_info")["ping_type"])
            assertEquals(1, eventsJson.getJSONArray("events").length())

            val baselineJson = JSONObject(requests["baseline"]!!)
            assertEquals("baseline", baselineJson.getJSONObject("ping_info")["ping_type"])
            checkPingSchema(baselineJson)

            val baselineMetricsObject = baselineJson.getJSONObject("metrics")
            val baselineStringMetrics = baselineMetricsObject.getJSONObject("string")
            assertEquals(1, baselineStringMetrics.length())
            assertNotNull(baselineStringMetrics.get("glean.baseline.locale"))

            val baselineTimespanMetrics = baselineMetricsObject.getJSONObject("timespan")
            assertEquals(1, baselineTimespanMetrics.length())
            assertNotNull(baselineTimespanMetrics.get("glean.baseline.duration"))
        } finally {
            server.shutdown()
            lifecycleRegistry.removeObserver(gleanLifecycleObserver)
        }
    }

    @Test
    fun `initialize() must not crash the app if Glean's data dir is messed up`() {
        // Remove the Glean's data directory.
        val gleanDir = File(
            context.applicationInfo.dataDir,
            GleanInternalAPI.GLEAN_DATA_DIR
        )
        assertTrue(gleanDir.deleteRecursively())

        // Create a file in its place.
        assertTrue(gleanDir.createNewFile())

        resetGlean()

        assertFalse(Glean.isInitialized())

        // Clean up after this, so that other tests don't fail.
        assertTrue(gleanDir.delete())
    }

    @Test
    fun `queued recorded metrics correctly record during init`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "counter_metric",
            sendInPings = listOf("store1")
        )

        // Enable queuing
        Dispatchers.API.setTaskQueueing(true)

        // This will queue 3 tasks that will add to the metric value once Glean is initialized
        for (i in 0..2) {
            counterMetric.add()
        }

        // Ensure that no value has been stored yet since the tasks have only been queued and not
        // executed yet
        assertFalse("No value must be stored", counterMetric.testHasValue())

        // Calling resetGlean here will cause Glean to be initialized and should cause the queued
        // tasks recording metrics to execute
        resetGlean(clearStores = false)

        // Verify that the callback was executed by testing for the correct value
        assertTrue("Value must exist", counterMetric.testHasValue())
        assertEquals("Value must match", 3, counterMetric.testGetValue())
    }

    @Test
    fun `Initializing twice is a no-op`() {
        val beforeConfig = Glean.configuration

        Glean.initialize(context, true)

        val afterConfig = Glean.configuration

        assertSame(beforeConfig, afterConfig)
    }

    @Test
    fun `Don't handle events when uninitialized`() {
        val gleanSpy = spy<GleanInternalAPI>(GleanInternalAPI::class.java)

        gleanSpy.testDestroyGleanHandle()
        runBlocking {
            gleanSpy.handleBackgroundEvent()
        }
        assertFalse(getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }

    @Test
    fun `Don't schedule pings if metrics disabled`() {
        Glean.setUploadEnabled(false)

        runBlocking {
            Glean.handleBackgroundEvent()
        }
        assertFalse(getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }

    @Test
    fun `Don't schedule pings if there is no ping content`() {
        resetGlean(getContextWithMockedInfo())

        runBlocking {
            Glean.handleBackgroundEvent()
        }

        // We should only have a baseline ping and no events or metrics pings since nothing was
        // recorded
        val files = File(Glean.getDataDir(), PingUploadWorker.PINGS_DIR).listFiles()!!

        // Make sure only the baseline ping is present and no events or metrics pings
        assertEquals(1, files.count())
        val file = files.first()
        BufferedReader(FileReader(file)).use {
            val lines = it.readLines()
            assert(lines[0].contains("baseline"))
        }
    }

    @Test
    fun `The appChannel must be correctly set, if requested`() {
        // No appChannel must be set if nothing was provided through the config
        // options.
        resetGlean(getContextWithMockedInfo(), Configuration())
        assertFalse(GleanInternalMetrics.appChannel.testHasValue())

        // The appChannel must be correctly reported if a channel value
        // was provided.
        val testChannelName = "my-test-channel"
        resetGlean(getContextWithMockedInfo(), Configuration(channel = testChannelName))
        assertTrue(GleanInternalMetrics.appChannel.testHasValue())
        assertEquals(testChannelName, GleanInternalMetrics.appChannel.testGetValue())
    }

    // glean-ac test removed.
    // `client_id and first_run_date metrics should be copied from the old location` was here.
    // 1539480 BACKWARD COMPATIBILITY HACK that is not needed anymore.

    // glean-ac test removed.
    // `client_id and first_run_date metrics should not override new location` was here.
    // 1539480 BACKWARD COMPATIBILITY HACK that is not needed anymore.

    @Test
    fun `getLanguageTag() reports the tag for the default locale`() {
        val defaultLanguageTag = getLocaleTag()

        assertNotNull(defaultLanguageTag)
        assertFalse(defaultLanguageTag.isEmpty())
        assertEquals("en-US", defaultLanguageTag)
    }

    @Test
    fun `getLanguageTag reports the correct tag for a non-default language`() {
        val defaultLocale = Locale.getDefault()

        try {
            Locale.setDefault(Locale("fy", "NL"))

            val languageTag = getLocaleTag()

            assertNotNull(languageTag)
            assertFalse(languageTag.isEmpty())
            assertEquals("fy-NL", languageTag)
        } finally {
            Locale.setDefault(defaultLocale)
        }
    }

    @Test
    fun `getLanguage reports the modern translation for some languages`() {
        assertEquals("he", getLanguageFromLocale(Locale("iw", "IL")))
        assertEquals("id", getLanguageFromLocale(Locale("in", "ID")))
        assertEquals("yi", getLanguageFromLocale(Locale("ji", "ID")))
    }

    @Test
    fun `ping collection must happen after currently scheduled metrics recordings`() {
        // Given the following block of code:
        //
        // Metric.A.set("SomeTestValue")
        // Glean.sendPings(listOf("custom-ping-1"))
        //
        // This test ensures that "custom-ping-1" contains "metric.a" with a value of "SomeTestValue"
        // when the ping is collected.

        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        val pingName = "custom_ping_1"
        val ping = PingType(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false
        )
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "string_metric",
            sendInPings = listOf(pingName)
        )

        // This test relies on testing mode to be disabled, since we need to prove the
        // real-world async behaviour of this. We don't need to care about clearing it,
        // the test-unit hooks will call `resetGlean` anyway.
        Dispatchers.API.setTestingMode(false)

        // This is the important part of the test. Even though both the metrics API and
        // sendPings are async and off the main thread, "SomeTestValue" should be recorded,
        // the order of the calls must be preserved.
        val testValue = "SomeTestValue"
        stringMetric.set(testValue)
        ping.submit()

        // Trigger worker task to upload the pings in the background. We need
        // to wait for the work to be enqueued first, since this test runs
        // asynchronously.
        waitForEnqueuedWorker(context, PingUploadWorker.PING_WORKER_TAG)
        triggerWorkManager(context)

        // Validate the received data.
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals(pingName, docType)

        val pingJson = JSONObject(request.body.readUtf8())
        assertEquals(pingName, pingJson.getJSONObject("ping_info")["ping_type"])
        checkPingSchema(pingJson)

        val pingMetricsObject = pingJson.getJSONObject("metrics")
        val pingStringMetrics = pingMetricsObject.getJSONObject("string")
        assertEquals(1, pingStringMetrics.length())
        assertEquals(testValue, pingStringMetrics.get("telemetry.string_metric"))
    }

    @Test
    fun `Basic metrics should be cleared when disabling uploading`() {
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "string_metric",
            sendInPings = listOf("default")
        )

        stringMetric.set("TEST VALUE")
        assertTrue(stringMetric.testHasValue())

        Glean.setUploadEnabled(false)
        assertFalse(stringMetric.testHasValue())
        stringMetric.set("TEST VALUE")
        assertFalse(stringMetric.testHasValue())

        Glean.setUploadEnabled(true)
        assertFalse(stringMetric.testHasValue())
        stringMetric.set("TEST VALUE")
        assertTrue(stringMetric.testHasValue())
    }

    @Test
    fun `Core metrics should be cleared and restored when disabling and enabling uploading`() {
        assertTrue(GleanInternalMetrics.os.testHasValue())

        Glean.setUploadEnabled(false)
        assertFalse(GleanInternalMetrics.os.testHasValue())

        Glean.setUploadEnabled(true)
        assertTrue(GleanInternalMetrics.os.testHasValue())
    }

    @Test
    fun `Workers should be cancelled when disabling uploading`() {
        // Force the MetricsPingScheduler to schedule the MetricsPingWorker
        Glean.metricsPingScheduler.schedulePingCollection(Calendar.getInstance(), true)
        // Enqueue a worker to send the baseline ping
        Pings.baseline.submit()

        // Verify that the workers are enqueued
        assertTrue("PingUploadWorker is enqueued",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
        assertTrue("MetricsPingWorker is enqueued",
            Glean.metricsPingScheduler.timer != null)

        Glean.setUploadEnabled(true)

        // Verify that the workers are still enqueued to show that setting upload enabled to true
        // doesn't affect any already queued workers, since we ask consumers to set upload enabled
        // before initializing glean.
        assertTrue("PingUploadWorker is enqueued",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
        assertTrue("MetricsPingWorker is enqueued",
            Glean.metricsPingScheduler.timer != null)

        // Toggle upload enabled to false
        Glean.setUploadEnabled(false)

        // Verify workers have been cancelled
        assertFalse("PingUploadWorker is not enqueued",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued)
        assertTrue("MetricsPingWorker is not enqueued",
            Glean.metricsPingScheduler.timer == null)
        // Verify deletion ping upload worker has been scheduled
        assertTrue("DeletionPingUploadWorker is not enqueued",
            getWorkerStatus(context, DeletionPingUploadWorker.PING_WORKER_TAG).isEnqueued)
    }

    @Test
    fun `isMainProcess must only return true if we are in the main process`() {
        val myPid = Int.MAX_VALUE

        assertTrue(Glean.isMainProcess(context))

        ShadowProcess.setPid(myPid)
        Glean.isMainProcess = null

        assertFalse(Glean.isMainProcess(context))
    }

    @Test(expected = IllegalThreadStateException::class)
    fun `Glean initialize must be called on the main thread`() {
        runBlocking(KotlinDispatchers.IO) {
            Glean.initialize(context, true)
        }
    }

    @Test
    fun `overflowing the task queue records telemetry`() {
        val server = getMockWebServer()
        Dispatchers.API.setTestingMode(true)
        Dispatchers.API.setTaskQueueing(true)

        repeat(110) {
            Dispatchers.API.launch {
            }
        }

        assertEquals("Task queue contains the maximum number of tasks",
            100, Dispatchers.API.taskQueue.size)
        assertEquals("overflowCount is correct", 10, Dispatchers.API.overflowCount)

        Glean.testDestroyGleanHandle()
        // Now trigger execution to ensure the tasks fired
        Glean.initialize(context, true, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        assertEquals(110, GleanError.preinitTasksOverflow.testGetValue())

        Pings.metrics.submit()

        // We can't just test the value of the metric here, because initialize causes a submission
        // of the metrics ping, and thus a reset of the ping-lifetime metric
        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val jsonContent = JSONObject(request.body.readUtf8())
        assertEquals(
            110,
            jsonContent
                .getJSONObject("metrics")
                .getJSONObject("counter")
                .getInt("glean.error.preinit_tasks_overflow")
        )

        Dispatchers.API.overflowCount = 0
    }
}
