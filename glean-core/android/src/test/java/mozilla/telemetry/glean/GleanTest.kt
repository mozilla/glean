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
import kotlinx.coroutines.runBlocking
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.internal.gleanSubmitPingByNameSync
import mozilla.telemetry.glean.private.CommonMetricData
import mozilla.telemetry.glean.private.CounterMetricType
import mozilla.telemetry.glean.private.EventMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.NoExtras
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.private.StringMetricType
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.utils.getLanguageFromLocale
import mozilla.telemetry.glean.utils.getLocaleTag
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Ignore
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.mock
import org.robolectric.shadows.ShadowLog
import org.robolectric.shadows.ShadowProcess
import java.io.File
import java.text.SimpleDateFormat
import java.util.Calendar
import java.util.Date
import java.util.Locale
import java.util.TimeZone
import java.util.concurrent.TimeUnit
import kotlinx.coroutines.Dispatchers as KotlinDispatchers

@RunWith(AndroidJUnit4::class)
class GleanTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(context)

    @After
    fun resetGlobalState() {
        Glean.setCollectionEnabled(true)
    }

    @Test
    fun `send a ping`() {
        delayMetricsPing(context)
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        Glean.handleBackgroundEvent()
        // Trigger it to upload
        triggerWorkManager(context)

        // We got exactly 1 ping here:
        // The baseline ping triggered by the background event above.
        assertEquals(1, server.requestCount)

        var request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        var docType = request.path!!.split("/")[3]
        assertEquals("baseline", docType)
    }

    @Test
    fun `X-Debug-ID header is correctly added when debug view tag is set`() {
        delayMetricsPing(context)
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        Glean.setDebugViewTag("this-ping-is-tagged")
        Glean.handleBackgroundEvent()

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        assertEquals(request!!.getHeader("X-Debug-ID"), "this-ping-is-tagged")
    }

    @Test
    fun `disabling upload should disable metrics recording`() {
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_metric",
                sendInPings = listOf("store1"),
            ),
        )
        Glean.setCollectionEnabled(false)

        stringMetric.set("foo")
        assertNull(stringMetric.testGetValue())
    }

    @Test
    fun `test experiments recording`() {
        Glean.setExperimentActive(
            "experiment_test",
            "branch_a",
        )
        Glean.setExperimentActive(
            "experiment_api",
            "branch_b",
            mapOf("test_key" to "value"),
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

        Glean.setExperimentActive(
            "experiment_set_preinit",
            "branch_a",
        )

        Glean.setExperimentActive(
            "experiment_preinit_disabled",
            "branch_a",
        )

        Glean.setExperimentInactive("experiment_preinit_disabled")

        // This will init glean and flush the dispatcher's queue.
        resetGlean()

        assertTrue(Glean.testIsExperimentActive("experiment_set_preinit"))
        assertFalse(Glean.testIsExperimentActive("experiment_preinit_disabled"))
    }

    @Test
    fun `test experimentation id recording`() {
        resetGlean(config = Configuration(experimentationId = "alpha-beta-gamma-delta"))
        assertEquals("alpha-beta-gamma-delta", Glean.testGetExperimentationId())
    }

    // Suppressing our own deprecation before we move over to the new event recording API.
    @Test
    @Suppress("ComplexMethod", "LongMethod", "NestedBlockDepth", "DEPRECATION")
    fun `test sending of foreground and background pings`() {
        val server = getMockWebServer()

        val click = EventMetricType<NoExtras>(
            CommonMetricData(
                disabled = false,
                category = "ui",
                lifetime = Lifetime.PING,
                name = "click",
                sendInPings = listOf("events"),
            ),
            allowedExtraKeys = emptyList(),
        )

        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        // Fake calling the lifecycle observer.
        val lifecycleOwner = mock(LifecycleOwner::class.java)
        val lifecycleRegistry = LifecycleRegistry(lifecycleOwner)
        val gleanLifecycleObserver = GleanLifecycleObserver()
        lifecycleRegistry.addObserver(gleanLifecycleObserver)

        try {
            // Simulate the first foreground event after the application starts.
            // Triggers a `baseline` ping (1)
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START)

            // Record an event in the foreground
            click.record()

            // Simulate going to background.
            // Triggers a `baseline` ping (2) and an `events` ping (3)
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_STOP)

            // Simulate going to foreground.
            // Triggers a `baseline` ping (4)
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_START)

            // Submit a metrics ping so we can check the foreground_count
            // Triggers a `metrics` ping (5)
            Glean.submitPingByName("metrics")

            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            // We got 5 pings total:
            // * 3 `baseline` pings
            // * 1 `events` ping
            // * 1 `metrics` ping
            assertEquals(5, server.requestCount)

            for (ignored in 1..5) {
                val request = server.takeRequest(5L, TimeUnit.SECONDS)!!
                val docType = request.path!!.split("/")[3]

                val json = JSONObject(request.getPlainBody())
                checkPingSchema(json)
                if (docType == "events") {
                    assertEquals("inactive", json.getJSONObject("ping_info").getString("reason"))
                    assertEquals(1, json.getJSONArray("events").length())
                } else if (docType == "baseline") {
                    val seq = json.getJSONObject("ping_info").getInt("seq")

                    // There are three baseline pings:
                    //   - seq: 0, reason: active, duration: null
                    //   - seq: 1, reason: inactive, duration: non-null
                    //   - seq: 2, reason: active, duration: null
                    if (seq == 0) {
                        assertEquals("active", json.getJSONObject("ping_info").getString("reason"))
                    }
                    if (seq == 1) {
                        val baselineMetricsObject = json.getJSONObject("metrics")
                        assertEquals("inactive", json.getJSONObject("ping_info").getString("reason"))
                        val baselineTimespanMetrics = baselineMetricsObject.getJSONObject("timespan")
                        assertEquals(1, baselineTimespanMetrics.length())
                        assertNotNull(baselineTimespanMetrics.get("glean.baseline.duration"))
                    }
                    if (seq == 2) {
                        assertEquals("active", json.getJSONObject("ping_info").getString("reason"))
                    }
                } else if (docType == "metrics") {
                    val seq = json.getJSONObject("ping_info").getInt("seq")
                    if (seq == 1) {
                        assertEquals(
                            2,
                            json.getJSONObject("metrics")
                                .getJSONObject("counter")
                                .getLong("glean.validation.foreground_count"),
                        )
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
        // TODO: Should be in Rust now.
        // Set the dirty flag.
        Glean.handleForegroundEvent()

        // Restart glean and don't clear the stores.
        val server = getMockWebServer()
        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            false,
        )

        try {
            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            var request = server.takeRequest(20L, TimeUnit.SECONDS)!!
            var docType = request.path!!.split("/")[3]
            assertEquals("The received ping must be a 'baseline' ping", "baseline", docType)

            var baselineJson = JSONObject(request.getPlainBody())
            assertEquals("dirty_startup", baselineJson.getJSONObject("ping_info")["reason"])
            checkPingSchema(baselineJson)

            request = server.takeRequest(20L, TimeUnit.SECONDS)!!
            docType = request.path!!.split("/")[3]
            assertEquals("The received ping must be a 'baseline' ping", "baseline", docType)
            baselineJson = JSONObject(request.getPlainBody())
            assertEquals("active", baselineJson.getJSONObject("ping_info")["reason"])
        } finally {
            server.shutdown()
        }
    }

    @Test
    @Ignore("this causes some subsequent tests to intermittently fail")
    fun `initialize() must not crash the app if Glean's data dir is messed up`() {
        // Remove the Glean's data directory.
        val gleanDir = File(
            context.applicationInfo.dataDir,
            GleanInternalAPI.GLEAN_DATA_DIR,
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
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Destroy Glean, so that the dispatcher will queue tasks until flushed.
        val gleanDataDir = File(context.applicationInfo.dataDir, GleanInternalAPI.GLEAN_DATA_DIR)
        Glean.testDestroyGleanHandle(clearStores = true, gleanDataDir.path)

        // This will queue 3 tasks that will add to the metric value once Glean is initialized
        for (ignored in 0..2) {
            counterMetric.add()
        }

        // We can't ensure that no value has been stored yet.
        // The tasks have been queued, but we have no Glean object yet to query the database.

        // Calling resetGlean here will cause Glean to be initialized and should cause the queued
        // tasks recording metrics to execute
        resetGlean(clearStores = false)

        // Verify that the callback was executed by testing for the correct value
        assertEquals("Value must match", 3, counterMetric.testGetValue())
    }

    @Test
    fun `Initializing twice is a no-op`() {
        val beforeConfig = Glean.configuration

        Glean.initialize(
            context,
            true,
            buildInfo = GleanBuildInfo.buildInfo,
        )

        val afterConfig = Glean.configuration

        assertSame(beforeConfig, afterConfig)
    }

    @Test
    fun `synchronous submit without Glean is a no-op`() {
        Glean.testDestroyGleanHandle()
        assertFalse(gleanSubmitPingByNameSync("events"))
    }

    @Test
    fun `The appChannel must be correctly set, if requested`() {
        // No appChannel must be set if nothing was provided through the config
        // options.
        resetGlean(getContext(), Configuration())
        assertNull(GleanInternalMetrics.appChannel.testGetValue())

        // The appChannel must be correctly reported if a channel value
        // was provided.
        val testChannelName = "my-test-channel"
        resetGlean(getContext(), Configuration(channel = testChannelName))
        assertEquals(testChannelName, GleanInternalMetrics.appChannel.testGetValue())
    }

    @Test
    fun `getLanguageTag reports the tag for the default locale`() {
        val defaultLanguageTag = getLocaleTag()

        assertNotNull(defaultLanguageTag)
        assertFalse(defaultLanguageTag.isEmpty())
        assertEquals("en-US", defaultLanguageTag)
    }

    @Test
    fun `getLanguageTag reports the correct tag for a non-default language`() {
        val defaultLocale = Locale.getDefault()

        try {
            Locale.setDefault(Locale.Builder().setLanguage("fy").setRegion("NL").build())

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
        assertEquals("he", getLanguageFromLocale(Locale.Builder().setLanguage("iw").setRegion("IL").build()))
        assertEquals("id", getLanguageFromLocale(Locale.Builder().setLanguage("in").setRegion("ID").build()))
        assertEquals("yi", getLanguageFromLocale(Locale.Builder().setLanguage("ji").setRegion("ID").build()))
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

        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        val pingName = "custom_ping_1"
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "string_metric",
                sendInPings = listOf(pingName),
            ),
        )

        // This test relies on testing mode to be disabled, since we need to prove the
        // real-world async behaviour of this. We don't need to care about clearing it,
        // the test-unit hooks will call `resetGlean` anyway.
        // Dispatchers.API.setTestingMode(false)
        Glean.setTestingMode(false)

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
        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
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
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "string_metric",
                sendInPings = listOf("store1"),
            ),
        )

        stringMetric.set("TEST VALUE")
        assertEquals("TEST VALUE", stringMetric.testGetValue()!!)

        Glean.setCollectionEnabled(false)
        assertNull(stringMetric.testGetValue())
        stringMetric.set("TEST VALUE")
        assertNull(stringMetric.testGetValue())

        Glean.setCollectionEnabled(true)
        assertNull(stringMetric.testGetValue())
        stringMetric.set("TEST VALUE")
        assertEquals("TEST VALUE", stringMetric.testGetValue()!!)
    }

    @Test
    fun `Core metrics are not cleared when disabling and enabling uploading`() {
        assertNotNull(GleanInternalMetrics.os.testGetValue())

        Glean.setCollectionEnabled(false)
        assertNotNull(GleanInternalMetrics.os.testGetValue())

        Glean.setCollectionEnabled(true)
        assertNotNull(GleanInternalMetrics.os.testGetValue())
    }

    @Test
    fun `Workers should be cancelled when disabling uploading`() {
        // Force the MetricsPingScheduler to schedule the MetricsPingWorker
        Glean.metricsPingScheduler!!.schedulePingCollection(
            Calendar.getInstance(),
            true,
            Pings.metricsReasonCodes.overdue,
        )
        // Enqueue a worker to send the baseline ping
        Pings.baseline.submit(Pings.baselineReasonCodes.active)

        // Verify that the workers are enqueued
        assertTrue(
            "PingUploadWorker is enqueued",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued,
        )
        assertTrue(
            "MetricsPingWorker is enqueued",
            Glean.metricsPingScheduler!!.timer != null,
        )

        Glean.setCollectionEnabled(true)

        // Verify that the workers are still enqueued to show that setting upload enabled to true
        // doesn't affect any already queued workers, since we ask consumers to set upload enabled
        // before initializing glean.
        assertTrue(
            "PingUploadWorker is enqueued",
            getWorkerStatus(context, PingUploadWorker.PING_WORKER_TAG).isEnqueued,
        )
        assertTrue(
            "MetricsPingWorker is enqueued",
            Glean.metricsPingScheduler!!.timer != null,
        )

        // Toggle upload enabled to false
        Glean.setCollectionEnabled(false)

        // Verify workers have been cancelled
        assertTrue(
            "MetricsPingWorker is not enqueued",
            Glean.metricsPingScheduler!!.timer == null,
        )
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
            Glean.initialize(
                context,
                true,
                buildInfo = GleanBuildInfo.buildInfo,
            )
        }
    }

    @Test
    @Ignore("oops, long running test see Bug 1911350 for more info")
    fun `overflowing the task queue records telemetry`() {
        delayMetricsPing(context)
        val server = getMockWebServer()

        // No Glean active, tasks will be queued.
        val gleanDataDir = File(context.applicationInfo.dataDir, GleanInternalAPI.GLEAN_DATA_DIR)
        Glean.testDestroyGleanHandle(clearStores = true, gleanDataDir.path)

        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "repeatedly",
                sendInPings = listOf("metrics"),
            ),
        )

        repeat(1000010) {
            counterMetric.add()
        }

        // Now trigger execution to ensure the tasks fired
        resetGlean(
            context,
            config = Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            clearStores = false,
            uploadEnabled = true,
        )

        Pings.metrics.submit()

        // We can't just test the value of the metric here, because initialize causes a submission
        // of the metrics ping, and thus a reset of the ping-lifetime metric
        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val jsonContent = JSONObject(request.getPlainBody())
        val counters =
            jsonContent
                .getJSONObject("metrics")
                .getJSONObject("counter")
        assertTrue(
            "Ping payload: $jsonContent",
            10 <= counters.getInt("glean.error.preinit_tasks_overflow"),
        )
        assertEquals(
            "Ping payload: $jsonContent",
            1000000,
            counters.getInt("telemetry.repeatedly"),
        )
    }

    @Test
    fun `sending deletion ping if disabled outside of run`() {
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            uploadEnabled = true,
        )

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            uploadEnabled = false,
            clearStores = false,
        )

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("deletion-request", docType)
    }

    @Test
    fun `no sending of deletion ping if unchanged outside of run`() {
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            uploadEnabled = false,
        )

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            uploadEnabled = false,
            clearStores = false,
        )

        assertEquals(0, server.requestCount)
    }

    @Test
    fun `test sending of startup baseline ping with application lifetime metric`() {
        // Set the dirty flag.
        Glean.setDirtyFlag(true)

        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "app_lifetime",
                sendInPings = listOf("baseline"),
            ),
        )
        stringMetric.set("HELLOOOOO!")

        // Restart glean and don't clear the stores.
        val server = getMockWebServer()
        val context = getContext()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
            clearStores = false,
        )

        try {
            // Trigger worker task to upload the pings in the background
            triggerWorkManager(context)

            val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
            val docType = request.path!!.split("/")[3]
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
    fun `setting debugViewTag before initialization should not crash`() {
        // Can't use resetGlean directly
        Glean.testDestroyGleanHandle()

        val context: Context = ApplicationProvider.getApplicationContext()
        val server = getMockWebServer()
        val config = Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
        )

        Glean.setDebugViewTag("valid-tag")
        resetGlean(context, config, uploadEnabled = true)

        // Send a ping
        Glean.handleBackgroundEvent()
        // Trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        assertEquals(request.getHeader("X-Debug-ID"), "valid-tag")
    }

    @Ignore("Bug 1685273: Frequent intermittent failures")
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

        val context = getContext()
        delayMetricsPing(context)

        // This test relies on testing mode to be disabled, since we need to prove the
        // real-world async behaviour of this.
        // We don't need to care about clearing it,
        // the test-unit hooks will call `resetGlean` anyway.
        // Dispatchers.API.setTaskQueueing(true)
        // Dispatchers.API.setTestingMode(false)

        // We create a ping and a metric before we initialize Glean
        val pingName = "sample_ping_1"
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            preciseTimestamps = true,
            includeInfoSections = true,
            enabled = true,
            schedulesPings = emptyList(),
            reasonCodes = listOf(),
            followsCollectionEnabled = true,
            uploaderCapabilities = emptyList(),
        )
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "string_metric",
                sendInPings = listOf(pingName),
            ),
        )

        val server = getMockWebServer()
        val config = Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
        )
        Glean.initialize(context, true, config, GleanBuildInfo.buildInfo)

        // Glean might still be initializing. Disable upload.
        Glean.setCollectionEnabled(false)

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
        val request = server.takeRequest(20L, TimeUnit.SECONDS)!!
        val docType = request.path!!.split("/")[3]
        assertEquals("deletion-request", docType)
    }

    @Test
    fun `test passing in explicit BuildInfo`() {
        Glean.testDestroyGleanHandle()

        val buildDate = Calendar.getInstance(TimeZone.getTimeZone("GMT+0"))
            .also { cal -> cal.set(2020, 10, 6, 11, 30, 50) }
        Glean.initialize(
            context,
            true,
            buildInfo = BuildInfo(versionName = "foo", versionCode = "c0ffee", buildDate = buildDate),
        )

        assertEquals("c0ffee", GleanInternalMetrics.appBuild.testGetValue())
        assertEquals("foo", GleanInternalMetrics.appDisplayVersion.testGetValue())
        assertEquals("2020-11-06T11:30:50+00:00", GleanInternalMetrics.buildDate.testGetValueAsString())
    }

    @Test
    fun `Date header is set on actual HTTP POST`() {
        delayMetricsPing(context)
        val server = getMockWebServer()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
            ),
        )

        Glean.handleBackgroundEvent()

        // Before the `Date` was set on submit,
        // so waiting here would ensure the actual sending is later.
        // Now we set the date on actual sending,
        // so we can observe it close to receive time further below.
        Thread.sleep(2000)

        // Now trigger it to upload
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val expected = Date()

        val dateHeader = request!!.getHeader("Date")!!
        val dateFormat = SimpleDateFormat("EEE, dd MMM yyyy HH:mm:ss z", Locale.US)
        dateFormat.timeZone = TimeZone.getTimeZone("GMT")
        val dateParsed = dateFormat.parse(dateHeader)!!

        val diff = expected.getTime() - dateParsed.getTime()

        // Send time is within 2 seconds of receive time
        // Due to timings the uploader takes a bit of time to launch.
        // Receiving the request takes a bit more.
        // This can easily sum up to just above one second.
        // We sleep for 2 seconds above, so if we check for less than 2 seconds here
        // we're still below that sleep time.
        assertTrue("Difference should be more than 0 seconds, was $diff", diff > 0)
        assertTrue("Difference should be less than 2 seconds, was $diff", diff < 2000)
    }

    @Test
    fun `Initialization succeeds with valid DB path`() {
        // Initialize with a custom data path and ensure `isCustomDataPath` is true.
        Glean.testDestroyGleanHandle()
        val cfg = Configuration(
            dataPath = File(context.applicationInfo.dataDir, "glean_test").absolutePath,
        )
        Glean.initialize(context, true, cfg, buildInfo = GleanBuildInfo.buildInfo)
        assertTrue(Glean.isCustomDataPath)

        // Initialize without a custom data path and ensure `isCustomDataPath` is false.
        Glean.testDestroyGleanHandle()
        Glean.initialize(context, true, buildInfo = GleanBuildInfo.buildInfo)
        assertFalse(Glean.isCustomDataPath)
    }

    @Test
    fun `Initialization fails with invalid DB path`() {
        Glean.testDestroyGleanHandle()

        // The path provided here is invalid because it is an empty string.
        val cfg = Configuration(dataPath = "")
        Glean.initialize(context, true, cfg, buildInfo = GleanBuildInfo.buildInfo)

        // Since the path is invalid, Glean should not properly initialize.
        assertFalse(Glean.initialized)
    }

    @Test
    fun `remote metric configurations are correctly applied`() {
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Set a metric configuration which will enable the telemetry.string_metric
        val metricConfig = """
            {
              "metrics_enabled": {
                "telemetry.string_metric": true
              }
            }
        """.trimIndent()
        Glean.applyServerKnobsConfig(metricConfig)

        // This should result in the metric being set to "foo"
        stringMetric.set("foo")
        assertNotNull(stringMetric.testGetValue())
        assertEquals("foo", stringMetric.testGetValue())
    }
}
