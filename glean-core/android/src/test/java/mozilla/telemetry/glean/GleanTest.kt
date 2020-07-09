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
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.ObsoleteCoroutinesApi
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import mozilla.telemetry.glean.GleanMetrics.GleanError
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.private.CounterMetricType
import mozilla.telemetry.glean.private.EventMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.NoExtraKeys
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.private.StringMetricType
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
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
import org.robolectric.shadows.ShadowLog
import java.io.File
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

    // New from glean-core.
    @Test
    fun `send a ping`() {
        val server = getMockWebServer()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        Glean.handleBackgroundEvent()
        // Trigger it to upload
        triggerWorkManager(context)

        // Make sure the number of request received thus far is only 1,
        // we are only expecting one baseline ping.
        assertEquals(server.requestCount, 1)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("baseline", docType)
    }

    @Test
    fun `X-Debug-ID header is correctly added when debug view tag is set`() {
        val server = getMockWebServer()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        Glean.setDebugViewTag("this-ping-is-tagged")
        Glean.handleBackgroundEvent()

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        assertEquals(request.getHeader("X-Debug-ID"), "this-ping-is-tagged")
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
    @Suppress("ComplexMethod", "LongMethod")
    fun `test sending of foreground and background pings`() {
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
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        // Fake calling the lifecycle observer.
        val lifecycleOwner = mock(LifecycleOwner::class.java)
        val lifecycleRegistry = LifecycleRegistry(lifecycleOwner)
        val gleanLifecycleObserver = GleanLifecycleObserver()
        lifecycleRegistry.addObserver(gleanLifecycleObserver)

        try {
            // Simulate the first foreground event after the application starts.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START)
            click.record()

            // Simulate going to background.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_STOP)

            // Simulate going to foreground.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START)

            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            for (i in 0..3) {
                val request = server.takeRequest(20L, TimeUnit.SECONDS)
                val docType = request.path.split("/")[3]
                val json = JSONObject(request.getPlainBody())
                checkPingSchema(json)
                if (docType == "events") {
                    assertEquals(1, json.getJSONArray("events").length())
                } else if (docType == "baseline") {
                    val seq = json.getJSONObject("ping_info").getInt("seq")

                    // There are three baseline pings:
                    //   - seq: 0, reason: foreground, duration: null
                    //   - seq: 1, reason: background, duration: non-null
                    //   - seq: 2, reason: foreground, duration: null
                    if (seq == 0 || seq == 2) {
                        // We may get error metrics in foreground pings,
                        // so 'metrics' may exist.
                        if (json.has("metrics")) {
                            val baselineMetricsObject = json.getJSONObject("metrics")
                            // Since we are only expecting error metrics,
                            // let's check that this is all we got.
                            assertEquals(1, baselineMetricsObject.length())
                            val baselineLabeledCounters = baselineMetricsObject.getJSONObject("labeled_counter")
                            baselineLabeledCounters.keys().forEach {
                                assert(it.startsWith("glean.error"))
                            }
                        }
                        assertEquals("foreground", json.getJSONObject("ping_info").getString("reason"))
                    } else if (seq == 1) {
                        val baselineMetricsObject = json.getJSONObject("metrics")
                        assertEquals("background", json.getJSONObject("ping_info").getString("reason"))
                        val baselineTimespanMetrics = baselineMetricsObject.getJSONObject("timespan")
                        assertEquals(1, baselineTimespanMetrics.length())
                        assertNotNull(baselineTimespanMetrics.get("glean.baseline.duration"))
                    }
                } else {
                    assertTrue("Unknown docType $docType", false)
                }
            }
        } finally {
            server.shutdown()
            lifecycleRegistry.removeObserver(gleanLifecycleObserver)
        }
    }

    @Test
    fun `test sending of startup baseline ping`() {
        // Set the dirty flag.
        LibGleanFFI.INSTANCE.glean_set_dirty_flag(true.toByte())

        // Restart glean and don't clear the stores.
        val server = getMockWebServer()
        val context = getContextWithMockedInfo()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), false)

        try {
            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            val request = server.takeRequest(20L, TimeUnit.SECONDS)
            val docType = request.path.split("/")[3]
            assertEquals("The received ping must be a 'baseline' ping", "baseline", docType)

            val baselineJson = JSONObject(request.getPlainBody())
            assertEquals("dirty_startup", baselineJson.getJSONObject("ping_info")["reason"])
            checkPingSchema(baselineJson)

            // We may get error metrics in dirty startup pings,
            // so 'metrics' may exist.
            if (baselineJson.has("metrics")) {
                val baselineMetricsObject = baselineJson.getJSONObject("metrics")
                // Since we are only expecting error metrics,
                // let's check that this is all we got.
                assertEquals(1, baselineMetricsObject.length())
                val baselineLabeledCounters = baselineMetricsObject.getJSONObject("labeled_counter")
                baselineLabeledCounters.keys().forEach {
                    assert(it.startsWith("glean.error"))
                }
            }
        } finally {
            server.shutdown()
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
            assertFalse(LibGleanFFI.INSTANCE.glean_submit_ping_by_name("events", null).toBoolean())
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
        // Glean.submitPings(listOf("custom-ping-1"))
        //
        // This test ensures that "custom-ping-1" contains "metric.a" with a value of "SomeTestValue"
        // when the ping is collected.

        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        val pingName = "custom_ping_1"
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf()
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

        val pingJson = JSONObject(request.getPlainBody())
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
        Glean.metricsPingScheduler.schedulePingCollection(
            Calendar.getInstance(),
            true,
            Pings.metricsReasonCodes.overdue
        )
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
        assertTrue("MetricsPingWorker is not enqueued",
            Glean.metricsPingScheduler.timer == null)
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
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        assertEquals(110, GleanError.preinitTasksOverflow.testGetValue())

        Pings.metrics.submit()

        // We can't just test the value of the metric here, because initialize causes a submission
        // of the metrics ping, and thus a reset of the ping-lifetime metric
        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val jsonContent = JSONObject(request.getPlainBody())
        assertEquals(
            110,
            jsonContent
                .getJSONObject("metrics")
                .getJSONObject("counter")
                .getInt("glean.error.preinit_tasks_overflow")
        )

        Dispatchers.API.overflowCount = 0
    }

    @Test
    fun `sending deletion ping if disabled outside of run`() {
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            uploadEnabled = true
        )

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            uploadEnabled = false,
            clearStores = false
        )

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("deletion-request", docType)
    }

    @Test
    fun `no sending of deletion ping if unchanged outside of run`() {
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            uploadEnabled = false
        )

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            uploadEnabled = false,
            clearStores = false
        )

        assertEquals(0, server.requestCount)
    }

    @Test
    fun `test sending of startup baseline ping with application lifetime metric`() {
        // Set the dirty flag.
        LibGleanFFI.INSTANCE.glean_set_dirty_flag(true.toByte())

        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "app_lifetime",
            sendInPings = listOf("baseline")
        )
        stringMetric.set("HELLOOOOO!")

        // Restart glean and don't clear the stores.
        val server = getMockWebServer()
        val context = getContextWithMockedInfo()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ), false)

        try {
            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            val request = server.takeRequest(20L, TimeUnit.SECONDS)
            val docType = request.path.split("/")[3]
            assertEquals("The received ping must be a 'baseline' ping", "baseline", docType)

            val baselineJson = JSONObject(request.getPlainBody())
            assertEquals("dirty_startup", baselineJson.getJSONObject("ping_info")["reason"])
            checkPingSchema(baselineJson)

            val appLifetimeMetricsObject = baselineJson.getJSONObject("metrics")
            val appLifetimeStringMetrics = appLifetimeMetricsObject.getJSONObject("string")
            assertEquals("HELLOOOOO!", appLifetimeStringMetrics.get("telemetry.app_lifetime"))
        } finally {
            server.shutdown()
        }
    }

    @Test
    fun `Initializing while registering pings isn't a race condition`() {
        // See bug 1635865

        Glean.testDestroyGleanHandle()

        // We need a signal for when "initialization is done", and one doesn't
        // really exist. For that, this sets a StringMetric on the pre-init task queue and
        // then waits for the task queue to be empty.

        Dispatchers.API.setTaskQueueing(true)
        Dispatchers.API.setTestingMode(false)

        val stringMetric = StringMetricType(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.Application,
                name = "string_metric",
                sendInPings = listOf("store1")
        )
        stringMetric.set("foo")

        // Add a bunch of ping types to the pingTypeQueue. We need many in here so
        // that registering pings during initialization is slow enough that we can
        // get other pings to be registered at the same time from another thread.

        // However, we don't want to add them to the queue and leave them there for
        // other tests (that makes the whole testing suite slower), so we first take
        // a copy of the current state, and restore it at the end of this test.

        val pingTypeQueueInitialState = HashSet(Glean.pingTypeQueue)

        for (i in 1..1000) {
            val ping = PingType<NoReasonCodes>(
                    name = "race-condition-ping$i",
                    includeClientId = true,
                    sendIfEmpty = false,
                    reasonCodes = listOf()
            )
            Glean.registerPingType(ping)
        }

        // Initialize Glean.  This will do most of the Glean initialization in the main
        // Glean coroutine in Dispatchers.API.
        val config = Configuration()
        Glean.initialize(context, true, config)

        // From another coroutine, just register pings as fast as we can to simulate the
        // ping registration race condition. Do this until any queued tasks in Glean are
        // complete (which signals the end of initialization). After that, restore the
        // pingTypeQueue state.
        runBlocking {
            GlobalScope.launch {
                val ping = PingType<NoReasonCodes>(
                        name = "race-condition-ping",
                        includeClientId = true,
                        sendIfEmpty = false,
                        reasonCodes = listOf()
                )

                // This timeout will fail and thereby fail the unit test if the
                // Glean.initialize coroutine crashes.
                withTimeout(500) {
                    while (Dispatchers.API.taskQueue.size > 0) {
                        Glean.registerPingType(ping)
                    }
                }
            }.join()
        }

        // Restore the initial state of the pingTypeQueue
        Glean.pingTypeQueue.clear()
        for (it in pingTypeQueueInitialState) {
            Glean.pingTypeQueue.add(it)
        }
    }

    @Test
    fun `test dirty flag is reset to false`() {
        // Set the dirty flag.
        LibGleanFFI.INSTANCE.glean_set_dirty_flag(true.toByte())

        resetGlean(context, Glean.configuration, false)

        assertFalse(LibGleanFFI.INSTANCE.glean_is_dirty_flag_set().toBoolean())
    }

    @Test
    fun `setting debugViewTag before initialization should not crash`() {
        // Can't use resetGlean directly
        Glean.testDestroyGleanHandle()

        val context: Context = ApplicationProvider.getApplicationContext()
        val server = getMockWebServer()
        val config = Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
        )

        Glean.setDebugViewTag("valid-tag")
        Glean.initialize(context, true, config)

        // Send a ping
        Glean.handleBackgroundEvent()
        // Trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        assertEquals(request.getHeader("X-Debug-ID"), "valid-tag")
    }

    @Test
    fun `flipping upload enabled respects order of events`() {
        // NOTES(janerik):
        // I'm reasonably sure this test is excercising the right code paths
        // and from the log output it does the right thing:
        //
        // * It fully initializes with the assumption uploadEnabled=true
        // * It then disables upload
        // * Then it submits the custom ping, which rightfully is ignored because uploadEnabled=false.
        //
        // The test passes.
        // But it also does that for the old code and I think it's because of some weird WorkManager behaviour,
        // where it doesn't actually start the work (= the upload).

        // Redirecting log output, usually done by resetGlean, which we don't use here.
        ShadowLog.stream = System.out
        // This test relies on Glean not being initialized, we do that ourselves.
        Glean.testDestroyGleanHandle()

        // This test relies on testing mode to be disabled, since we need to prove the
        // real-world async behaviour of this.
        // We don't need to care about clearing it,
        // the test-unit hooks will call `resetGlean` anyway.
        Dispatchers.API.setTaskQueueing(true)
        Dispatchers.API.setTestingMode(false)

        // We create a ping and a metric before we initialize Glean
        val pingName = "sample_ping_1"
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf()
        )
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "string_metric",
            sendInPings = listOf(pingName)
        )

        val server = getMockWebServer()
        val context = getContextWithMockedInfo()
        val config = Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        )
        Glean.initialize(context, true, config)

        // Glean might still be initializing. Disable upload.
        Glean.setUploadEnabled(false)

        // Set data and try to submit a custom ping.
        val testValue = "SomeTestValue"
        stringMetric.set(testValue)
        ping.submit()

        // Trigger worker task to upload any submitted ping.
        // We need to wait for the work to be enqueued first,
        // since this test runs asynchronously.
        waitForEnqueuedWorker(context, PingUploadWorker.PING_WORKER_TAG)
        triggerWorkManager(context)

        // Validate the received data.
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("deletion-request", docType)
    }
}
