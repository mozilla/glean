/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.debug

import android.content.Context
import org.junit.Test
import org.junit.runner.RunWith
import android.content.Intent
import androidx.test.core.app.ApplicationProvider
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.config.Configuration
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import android.content.pm.ActivityInfo
import android.content.pm.ResolveInfo
import androidx.test.core.app.ActivityScenario.launch
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.private.BooleanMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.triggerWorkManager
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.net.HeadersList
import mozilla.telemetry.glean.net.PingUploader
import mozilla.telemetry.glean.net.UploadResult
import mozilla.telemetry.glean.net.HttpResponse
import mozilla.telemetry.glean.private.NoReasonCodes
import mozilla.telemetry.glean.private.PingType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Rule
import org.robolectric.Shadows.shadowOf
import java.util.concurrent.TimeUnit

/**
 * This is a helper class to facilitate testing of ping tagging
 */
private class TestPingTagClient(
    private val responseUrl: String = Configuration.DEFAULT_TELEMETRY_ENDPOINT,
    private val debugHeaderValue: String? = null,
    private val sourceTagsValue: Set<String>? = null
) : PingUploader {
    override fun upload(url: String, data: ByteArray, headers: HeadersList): UploadResult {
        assertTrue("URL must be redirected for tagged pings",
            url.startsWith(responseUrl))
        debugHeaderValue?.let {
            assertEquals("The debug view header must match what the ping tag was set to",
                debugHeaderValue, headers.find { it.first == "X-Debug-ID" }!!.second)
        }
        sourceTagsValue?.let {
            assertEquals("The source tags header must match what the ping tag was set to",
                sourceTagsValue.joinToString(","), headers.find { it.first == "X-Source-Tags" }!!.second)
        }

        return HttpResponse(200)
    }
}

@RunWith(AndroidJUnit4::class)
class GleanDebugActivityTest {

    private val testPackageName = "mozilla.telemetry.glean.test"

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Before
    fun setup() {
        // This makes sure we have a "launch" intent in our package, otherwise
        // it will fail looking for it in `GleanDebugActivityTest`.
        val pm = ApplicationProvider.getApplicationContext<Context>().packageManager
        val launchIntent = Intent(Intent.ACTION_MAIN)
        launchIntent.setPackage(testPackageName)
        launchIntent.addCategory(Intent.CATEGORY_LAUNCHER)

        // Add a test main launcher activity.
        val resolveInfo = ResolveInfo()
        resolveInfo.activityInfo = ActivityInfo()
        resolveInfo.activityInfo.packageName = testPackageName
        resolveInfo.activityInfo.name = "LauncherActivity"
        @Suppress("DEPRECATION")
        shadowOf(pm).addResolveInfoForIntent(launchIntent, resolveInfo)

        // Add a second testing activity.
        val otherActivityInfo = ActivityInfo()
        otherActivityInfo.packageName = testPackageName
        otherActivityInfo.name = "OtherActivity"
        otherActivityInfo.exported = true
        shadowOf(pm).addOrUpdateActivity(otherActivityInfo)

        // Add another hidden testing activity.
        val hiddenActivity = ActivityInfo()
        hiddenActivity.packageName = testPackageName
        hiddenActivity.name = "HiddenActivity"
        hiddenActivity.exported = false
        shadowOf(pm).addOrUpdateActivity(hiddenActivity)
    }

    @Test
    fun `the main activity is correctly started and intent args are propagated`() {
        // Build the intent that will call our debug activity, with no extra.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
            GleanDebugActivity::class.java)
        // Add at least an option, otherwise the activity will be removed.
        intent.putExtra(GleanDebugActivity.LOG_PINGS_EXTRA_KEY, true)
        intent.putExtra("TestOptionFromCLI", "TestValue")
        // Start the activity through our intent.
        val scenario = launch<GleanDebugActivity>(intent)

        // Check that our main activity was launched.
        scenario.onActivity { activity ->
            val startedIntent = shadowOf(activity).peekNextStartedActivityForResult().intent
            assertEquals(testPackageName, startedIntent.`package`!!)
            // Make sure that the extra intent option was propagated to this intent.
            assertEquals("TestValue", startedIntent.getStringExtra("TestOptionFromCLI"))
        }
    }

    @Test
    fun `pings are sent using sendPing`() {
        val server = getMockWebServer()

        val context = ApplicationProvider.getApplicationContext<Context>()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        // Put some metric data in the store, otherwise we won't get a ping out
        // Define a 'booleanMetric' boolean metric, which will be stored in "store1"
        val booleanMetric = BooleanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "boolean_metric",
            sendInPings = listOf("metrics")
        )

        booleanMetric.set(true)
        assertTrue(booleanMetric.testHasValue())

        // Set the extra values and start the intent.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
        GleanDebugActivity::class.java)
        intent.putExtra(GleanDebugActivity.SEND_PING_EXTRA_KEY, "metrics")
        launch<GleanDebugActivity>(intent)

        // Since we reset the serverEndpoint back to the default for untagged pings, we need to
        // override it here so that the local server we created to intercept the pings will
        // be the one that the ping is sent to.
        Glean.configuration = Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        )

        triggerWorkManager(context)
        val request = server.takeRequest(10L, TimeUnit.SECONDS)

        assertTrue(
            request.requestUrl.encodedPath().startsWith("/submit/mozilla-telemetry-glean-test/metrics")
        )

        server.shutdown()
    }

    @Test
    fun `debugViewTag filters ID's that don't match the pattern`() {
        val server = getMockWebServer()

        val context = ApplicationProvider.getApplicationContext<Context>()
        resetGlean(context, Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port
        ))

        // Put some metric data in the store, otherwise we won't get a ping out
        // Define a 'booleanMetric' boolean metric, which will be stored in "store1"
        val booleanMetric = BooleanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "boolean_metric",
            sendInPings = listOf("metrics")
        )

        booleanMetric.set(true)
        assertTrue(booleanMetric.testHasValue())

        // Set the extra values and start the intent.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
            GleanDebugActivity::class.java)
        intent.putExtra(GleanDebugActivity.SEND_PING_EXTRA_KEY, "metrics")
        intent.putExtra(GleanDebugActivity.TAG_DEBUG_VIEW_EXTRA_KEY, "inv@lid_id")
        launch<GleanDebugActivity>(intent)

        // Since a bad tag ID results in resetting the endpoint to the default, verify that
        // has happened.
        assertEquals("Server endpoint must be reset if tag didn't pass regex",
            "http://" + server.hostName + ":" + server.port, Glean.configuration.serverEndpoint)

        triggerWorkManager(context)
        val request = server.takeRequest(10L, TimeUnit.SECONDS)

        assertTrue(
            "Request path must be correct",
            request.requestUrl.encodedPath().startsWith("/submit/mozilla-telemetry-glean-test/metrics")
        )

        // resetGlean doesn't actually reset the debug view tag,
        // so we might have a tag from other tests here.
        assertNotEquals("inv@lid_id", request.headers.get("X-Debug-ID"))

        server.shutdown()
    }

    @Test
    fun `pings are correctly tagged using legacy tagPings`() {
        val pingTag = "legacy-debug-ID"

        // Use the test client in the Glean configuration
        val context = ApplicationProvider.getApplicationContext<Context>()
        resetGlean(context, Glean.configuration.copy(
            httpClient = TestPingTagClient(debugHeaderValue = pingTag)
        ))

        // Create a custom ping for testing. Since we're testing headers,
        // it's fine for this to be empty.
        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = false,
            sendIfEmpty = true,
            reasonCodes = listOf()
        )

        // Set the extra values and start the intent.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
            GleanDebugActivity::class.java)
        intent.putExtra(GleanDebugActivity.SEND_PING_EXTRA_KEY, "metrics")
        intent.putExtra(GleanDebugActivity.LEGACY_TAG_PINGS, pingTag)
        launch<GleanDebugActivity>(intent)

        customPing.submit()

        // This will trigger the call to `fetch()` in the TestPingTagClient which is where the
        // test assertions will occur
        triggerWorkManager(context)
    }

    @Test
    fun `pings are correctly tagged using sourceTags`() {
        val testTags = setOf("tag1", "tag2")

        // Use the test client in the Glean configuration
        val context = ApplicationProvider.getApplicationContext<Context>()
        resetGlean(context, Glean.configuration.copy(
                httpClient = TestPingTagClient(sourceTagsValue = testTags)
        ))

        // Create a custom ping for testing. Since we're testing headers,
        // it's fine for this to be empty.
        val customPing = PingType<NoReasonCodes>(
            name = "custom",
            includeClientId = false,
            sendIfEmpty = true,
            reasonCodes = listOf()
        )

        // Set the extra values and start the intent.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
                GleanDebugActivity::class.java)
        intent.putExtra(GleanDebugActivity.SEND_PING_EXTRA_KEY, "metrics")
        intent.putExtra(GleanDebugActivity.SOURCE_TAGS_KEY, testTags.toTypedArray())
        launch<GleanDebugActivity>(intent)

        customPing.submit()

        // This will trigger the call to `fetch()` in the TestPingTagClient which is where the
        // test assertions will occur
        triggerWorkManager(context)
    }

    @Test
    fun `a custom activity is correctly started`() {
        // Build the intent that will call our debug activity, with no extra.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
            GleanDebugActivity::class.java)
        // Add at least an option, otherwise the activity will be removed.
        intent.putExtra(GleanDebugActivity.NEXT_ACTIVITY_TO_RUN, "OtherActivity")
        intent.putExtra("TestOptionFromCLI", "TestValue")
        // Start the activity through our intent.
        val scenario = launch<GleanDebugActivity>(intent)

        // Check that our main activity was launched.
        scenario.onActivity { activity ->
            val startedIntent = shadowOf(activity).peekNextStartedActivityForResult().intent
            assertEquals(testPackageName, startedIntent.`package`!!)
            assertEquals("OtherActivity", startedIntent.component!!.className)
            // Make sure that the extra intent option was propagated to this intent.
            assertEquals("TestValue", startedIntent.getStringExtra("TestOptionFromCLI"))
        }
    }

    @Test
    fun `non-exported activity is not started`() {
        // Build the intent that will call our debug activity, with no extra.
        val intent = Intent(ApplicationProvider.getApplicationContext<Context>(),
            GleanDebugActivity::class.java)
        // Add at least an option, otherwise the activity will be removed.
        intent.putExtra(GleanDebugActivity.NEXT_ACTIVITY_TO_RUN, "HiddenActivity")
        // Start the activity through our intent.
        val scenario = launch<GleanDebugActivity>(intent)
        scenario.onActivity { activity ->
            // We don't expect any activity to be launched.
            assertNull(shadowOf(activity).peekNextStartedActivityForResult())
        }
    }
}
