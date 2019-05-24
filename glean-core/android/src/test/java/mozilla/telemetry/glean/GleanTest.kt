/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.LifecycleRegistry
import androidx.test.core.app.ApplicationProvider
import androidx.work.testing.WorkManagerTestInitHelper
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.ObsoleteCoroutinesApi
import kotlinx.coroutines.runBlocking
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.net.PingUploader
// import mozilla.components.service.glean.firstrun.FileFirstRunDetector
// import mozilla.telemetry.glean.private.DatetimeMetricType
import mozilla.telemetry.glean.private.EventMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.NoExtraKeys
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.private.StringMetricType
// import mozilla.telemetry.glean.private.TimeUnit as GleanTimeUnit
// import mozilla.telemetry.glean.private.UuidMetricType
// import mozilla.components.service.glean.storages.StringsStorageEngine
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.PingUploadWorker
// import mozilla.components.service.glean.storages.StorageEngineManager
import mozilla.telemetry.glean.utils.getLanguageFromLocale
import mozilla.telemetry.glean.utils.getLocaleTag
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
// import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
// import org.junit.Assert.assertNotEquals
import org.junit.Before
import org.junit.Ignore
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.mock
import org.mockito.Mockito.spy
import org.robolectric.RobolectricTestRunner
import java.io.BufferedReader
import java.io.File
import java.io.FileReader
// import java.util.Date
import java.util.Locale
// import java.util.UUID
import java.util.concurrent.TimeUnit

@ObsoleteCoroutinesApi
@ExperimentalCoroutinesApi
@RunWith(RobolectricTestRunner::class)
class GleanTest {

    @Before
    fun setup() {
        WorkManagerTestInitHelper.initializeTestWorkManager(
            ApplicationProvider.getApplicationContext())

        resetGlean()
    }

    @After
    fun resetGlobalState() {
        Glean.setUploadEnabled(true)
    }

    // New from glean-core.
    @Test
    fun `send a ping`() {
        val server = getMockWebServer()
        val context: Context = ApplicationProvider.getApplicationContext()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        Glean.handleBackgroundEvent()
        // Make sure the file is on the disk
        val pingPath = File(context.applicationInfo.dataDir, "glean_data/pings")
        // Only the baseline ping should have been written
        assertEquals(1, pingPath.listFiles()?.size)

        // Now trigger it to upload
        triggerWorkManager()

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals("baseline", docType)
    }

    @Test
    fun `sending an empty ping doesn't queue work`() {
        Glean.sendPings(listOf(Pings.metrics))
        assertFalse(isWorkScheduled(PingUploadWorker.PING_WORKER_TAG))
    }

    // Tests from glean-ac (706af1f).

    @Ignore("This should probably be implemented in Rust. We should also have a test here to check that nothing gets enqueued in the Dispatcher")
    @Test
    fun `disabling upload should disable metrics recording`() {
        /*
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
        assertNull(
                "Metrics should not be recorded if Glean is disabled",
                StringsStorageEngine.getSnapshot(storeName = "store1", clearStore = false)
        )
        */
    }

    @Ignore("The experiments API is not implemented yet. See bug 1552471.")
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

    @Ignore("Disabled because we're not triggering pings when going to background. See bug 1551159.")
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

        resetGlean(getContextWithMockedInfo(), Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        // Fake calling the lifecycle observer.
        val lifecycleRegistry = LifecycleRegistry(mock(LifecycleOwner::class.java))
        val gleanLifecycleObserver = GleanLifecycleObserver()
        lifecycleRegistry.addObserver(gleanLifecycleObserver)

        try {
            // Simulate the first foreground event after the application starts.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_RESUME)
            click.record()

            // Simulate going to background.
            lifecycleRegistry.handleLifecycleEvent(Lifecycle.Event.ON_STOP)

            // Trigger worker task to upload the pings in the background
            triggerWorkManager()

            val requests: MutableMap<String, String> = mutableMapOf()
            for (i in 0..1) {
                val request = server.takeRequest(20L, TimeUnit.SECONDS)
                val docType = request.path.split("/")[3]
                requests.set(docType, request.body.readUtf8())
            }

            val eventsJson = JSONObject(requests["events"])
            checkPingSchema(eventsJson)
            assertEquals("events", eventsJson.getJSONObject("ping_info")["ping_type"])
            assertEquals(1, eventsJson.getJSONArray("events")!!.length())

            val baselineJson = JSONObject(requests["baseline"])
            assertEquals("baseline", baselineJson.getJSONObject("ping_info")["ping_type"])
            checkPingSchema(baselineJson)

            val baselineMetricsObject = baselineJson.getJSONObject("metrics")!!
            val baselineStringMetrics = baselineMetricsObject.getJSONObject("string")!!
            assertEquals(1, baselineStringMetrics.length())
            assertNotNull(baselineStringMetrics.get("glean.baseline.locale"))

            val baselineTimespanMetrics = baselineMetricsObject.getJSONObject("timespan")!!
            assertEquals(1, baselineTimespanMetrics.length())
            assertNotNull(baselineTimespanMetrics.get("glean.baseline.duration"))
        } finally {
            server.shutdown()
            lifecycleRegistry.removeObserver(gleanLifecycleObserver)
        }
    }

    @Ignore("This is currently failing")
    @Test
    fun `initialize() must not crash the app if Glean's data dir is messed up`() {
        // Remove the Glean's data directory.
        val gleanDir = File(
            ApplicationProvider.getApplicationContext<Context>().applicationInfo.dataDir,
            GleanInternalAPI.GLEAN_DATA_DIR
        )
        assertTrue(gleanDir.deleteRecursively())

        // Create a file in its place.
        assertTrue(gleanDir.createNewFile())

        resetGlean()

        // Clean up after this, so that other tests don't fail.
        assertTrue(gleanDir.delete())
    }

    @Ignore("This should probably be implemented in the Rust side.")
    @Test
    fun `Don't send metrics if not initialized`() {
        /*
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "string_metric",
            sendInPings = listOf("store1")
        )
        Glean.initialized = false
        stringMetric.set("foo")
        assertNull(
            "Metrics should not be recorded if Glean is not initialized",
            StringsStorageEngine.getSnapshot(storeName = "store1", clearStore = false)
        )

        Glean.initialized = true
        */
    }

    @Test
    fun `Initializing twice is a no-op`() {
        val beforeConfig = Glean.configuration

        Glean.initialize(ApplicationProvider.getApplicationContext())

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
        assertFalse(isWorkScheduled(PingUploadWorker.PING_WORKER_TAG))
    }

    @Test
    fun `Don't schedule pings if metrics disabled`() {
        Glean.setUploadEnabled(false)

        runBlocking {
            Glean.handleBackgroundEvent()
        }
        assertFalse(isWorkScheduled(PingUploadWorker.PING_WORKER_TAG))
    }

    @Test
    fun `Don't schedule pings if there is no ping content`() {
        resetGlean(getContextWithMockedInfo())

        runBlocking {
            Glean.handleBackgroundEvent()
        }

        // We should only have a baseline ping and no events or metrics pings since nothing was
        // recorded
        val files = File(Glean.getDataDir(), PingUploader.PINGS_DIR).listFiles()

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

    @Ignore("This should probably be implemented in Rust")
    @Test
    fun `client_id and first_run_date must be generated if not available after the first start`() {
        // 1539480 BACKWARD COMPATIBILITY HACK

        // The resetGlean called right before this function will add client_id
        // and first_run_date to the new location in glean_client_info.  We
        // need to clear those out again so we can test what happens when they
        // are missing.
        /*
        StorageEngineManager(
            applicationContext = ApplicationProvider.getApplicationContext()
        ).clearAllStores()

        assertFalse(GleanInternalMetrics.clientId.testHasValue())
        assertFalse(GleanInternalMetrics.firstRunDate.testHasValue())

        // Set this to be a non-first start with missing clientId/firstRunDate.
        val gleanDataDir =
            File(ApplicationProvider.getApplicationContext<Context>().applicationInfo.dataDir, Glean.GLEAN_DATA_DIR)
        val firstRunDetector = FileFirstRunDetector(gleanDataDir)
        firstRunDetector.createFirstRunFile()

        // This should copy the values to their new locations
        Glean.initialized = false
        Glean.initialize(ApplicationProvider.getApplicationContext())

        assertTrue(GleanInternalMetrics.clientId.testHasValue())
        assertTrue(GleanInternalMetrics.firstRunDate.testHasValue())
        */
    }

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

        val pingName = "custom_ping_1"
        val ping = PingType(
            name = pingName,
            includeClientId = true
        )
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "string_metric",
            sendInPings = listOf(pingName)
        )

        resetGlean(getContextWithMockedInfo(), Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

        // This test relies on testing mode to be disabled, since we need to prove the
        // real-world async behaviour of this. We don't need to care about clearing it,
        // the test-unit hooks will call `resetGlean` anyway.
        Dispatchers.API.setTestingMode(false)

        // This is the important part of the test. Even though both the metrics API and
        // sendPings are async and off the main thread, "SomeTestValue" should be recorded,
        // the order of the calls must be preserved.
        val testValue = "SomeTestValue"
        stringMetric.set(testValue)
        ping.send()

        // Trigger worker task to upload the pings in the background. We need
        // to wait for the work to be enqueued first, since this test runs
        // asynchronously.
        waitForEnqueuedWorker(PingUploadWorker.PING_WORKER_TAG)
        triggerWorkManager()

        // Validate the received data.
        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals(pingName, docType)

        val pingJson = JSONObject(request.body.readUtf8())
        assertEquals(pingName, pingJson.getJSONObject("ping_info")["ping_type"])
        checkPingSchema(pingJson)

        val pingMetricsObject = pingJson.getJSONObject("metrics")!!
        val pingStringMetrics = pingMetricsObject.getJSONObject("string")!!
        assertEquals(1, pingStringMetrics.length())
        assertEquals(testValue, pingStringMetrics.get("telemetry.string_metric"))
    }
}
