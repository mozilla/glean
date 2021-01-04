/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* This file is based on the tests in the Glean android-components implementation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

package mozilla.telemetry.glean.private

import android.os.SystemClock
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.getPlainBody
import mozilla.telemetry.glean.checkPingSchema
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.delayMetricsPing
import mozilla.telemetry.glean.waitForPingContent
import java.lang.NullPointerException
import java.util.concurrent.TimeUnit
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Assert.fail
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

// Declared here, since Kotlin can not declare nested enum classes.
enum class clickKeys {
    objectId,
    other
}

enum class testNameKeys {
    testName
}

enum class SomeExtraKeys {
    SomeExtra
}

@RunWith(AndroidJUnit4::class)
class EventMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API records to its storage engine`() {

        // Define a 'click' event, which will be stored in "store1"
        val click = EventMetricType<clickKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("store1"),
            allowedExtraKeys = listOf("object_id", "other")
        )

        // Record two events of the same type, with a little delay.
        click.record(extra = mapOf(clickKeys.objectId to "buttonA", clickKeys.other to "foo"))

        val expectedTimeSinceStart: Long = 37
        SystemClock.sleep(expectedTimeSinceStart)

        click.record(extra = mapOf(clickKeys.objectId to "buttonB", clickKeys.other to "bar"))

        // Check that data was properly recorded.
        val snapshot = click.testGetValue()
        assertTrue(click.testHasValue())
        assertEquals(2, snapshot.size)

        val firstEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonA" }
        assertEquals("ui", firstEvent.category)
        assertEquals("click", firstEvent.name)
        assertEquals("foo", firstEvent.extra?.get("other"))

        val secondEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonB" }
        assertEquals("ui", secondEvent.category)
        assertEquals("click", secondEvent.name)
        assertEquals("bar", secondEvent.extra?.get("other"))

        assertTrue("The sequence of the events must be preserved",
            firstEvent.timestamp < secondEvent.timestamp)
    }

    @Test
    fun `The API records to its storage engine when category is empty`() {
        // Define a 'click' event, which will be stored in "store1"
        val click = EventMetricType<clickKeys>(
            disabled = false,
            category = "",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("store1"),
            allowedExtraKeys = listOf("object_id")
        )

        // Record two events of the same type, with a little delay.
        click.record(extra = mapOf(clickKeys.objectId to "buttonA"))

        val expectedTimeSinceStart: Long = 37
        SystemClock.sleep(expectedTimeSinceStart)

        click.record(extra = mapOf(clickKeys.objectId to "buttonB"))

        // Check that data was properly recorded.
        val snapshot = click.testGetValue()
        assertTrue(click.testHasValue())
        assertEquals(2, snapshot.size)

        val firstEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonA" }
        assertEquals("click", firstEvent.name)

        val secondEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonB" }
        assertEquals("click", secondEvent.name)

        assertTrue("The sequence of the events must be preserved",
            firstEvent.timestamp < secondEvent.timestamp)
    }

    @Test
    fun `disabled events must not record data`() {
        // Define a 'click' event, which will be stored in "store1". It's disabled
        // so it should not record anything.
        val click = EventMetricType<NoExtraKeys>(
            disabled = true,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("store1")
        )

        // Attempt to store the event.
        click.record()

        // Check that nothing was recorded.
        assertFalse("Events must not be recorded if they are disabled",
            click.testHasValue())
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val testEvent = EventMetricType<NoExtraKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "testEvent",
            sendInPings = listOf("store1")
        )
        testEvent.testGetValue()
    }

    @Test
    fun `The API records to secondary pings`() {
        // Define a 'click' event, which will be stored in "store1" and "store2"
        val click = EventMetricType<clickKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("store1", "store2"),
            allowedExtraKeys = listOf("object_id")
        )

        // Record two events of the same type, with a little delay.
        click.record(extra = mapOf(clickKeys.objectId to "buttonA"))

        val expectedTimeSinceStart: Long = 37
        SystemClock.sleep(expectedTimeSinceStart)

        click.record(extra = mapOf(clickKeys.objectId to "buttonB"))

        // Check that data was properly recorded in the second ping.
        val snapshot = click.testGetValue("store2")
        assertTrue(click.testHasValue("store2"))
        assertEquals(2, snapshot.size)

        val firstEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonA" }
        assertEquals("ui", firstEvent.category)
        assertEquals("click", firstEvent.name)

        val secondEvent = snapshot.single { e -> e.extra?.get("object_id") == "buttonB" }
        assertEquals("ui", secondEvent.category)
        assertEquals("click", secondEvent.name)

        assertTrue("The sequence of the events must be preserved",
            firstEvent.timestamp < secondEvent.timestamp)
    }

    @Test
    fun `events should not record when upload is disabled`() {
        val eventMetric = EventMetricType<testNameKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "event_metric",
            sendInPings = listOf("store1"),
            allowedExtraKeys = listOf("test_name")
        )
        Glean.setUploadEnabled(true)
        eventMetric.record(mapOf(testNameKeys.testName to "event1"))
        val snapshot1 = eventMetric.testGetValue()
        assertEquals(1, snapshot1.size)
        Glean.setUploadEnabled(false)
        eventMetric.record(mapOf(testNameKeys.testName to "event2"))
        @Suppress("EmptyCatchBlock")
        try {
            eventMetric.testGetValue()
            fail("Expected events to be empty")
        } catch (e: NullPointerException) {
        }
        Glean.setUploadEnabled(true)
        eventMetric.record(mapOf(testNameKeys.testName to "event3"))
        val snapshot3 = eventMetric.testGetValue()
        assertEquals(1, snapshot3.size)
    }

    // Moved from EventsStorageEngineTest.kt in glean-ac
    @Test
    fun `flush queued events on startup`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = true
        )

        val event = EventMetricType<SomeExtraKeys>(
            disabled = false,
            category = "telemetry",
            name = "test_event",
            lifetime = Lifetime.Ping,
            sendInPings = listOf("events"),
            allowedExtraKeys = listOf("someExtra")
        )

        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "bar"))
        assertEquals(1, event.testGetValue().size)

        // Start a new Glean instance to trigger the sending of "stale" events
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = false
        )

        triggerWorkManager(context)

        val request = server.takeRequest(1L, TimeUnit.SECONDS)
        assertEquals("POST", request.method)
        val applicationId = "mozilla-telemetry-glean-test"
        assert(
            request.path.startsWith("/submit/$applicationId/events/")
        )
        val pingJsonData = request.getPlainBody()
        val pingJson = JSONObject(pingJsonData)
        checkPingSchema(pingJson)
        assertNotNull(pingJson.opt("events"))
        assertEquals(
            1,
            pingJson.getJSONArray("events").length()
        )
        assertEquals(
            "startup",
            pingJson.getJSONObject("ping_info").getString("reason")
        )
    }

    // Moved from EventsStorageEngineTest.kt in glean-ac
    @kotlinx.coroutines.ObsoleteCoroutinesApi
    @Suppress("LongMethod")
    @Test
    fun `flush queued events on startup and correctly handle pre-init events`() {
        val server = getMockWebServer()

        val context = getContextWithMockedInfo()
        delayMetricsPing(context)
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = true
        )

        val event = EventMetricType<SomeExtraKeys>(
            disabled = false,
            category = "telemetry",
            name = "test_event",
            lifetime = Lifetime.Ping,
            sendInPings = listOf("events"),
            allowedExtraKeys = listOf("someExtra")
        )

        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "run1"))
        assertEquals(1, event.testGetValue().size)

        Dispatchers.API.setTaskQueueing(true)
        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "pre-init"))

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = false
        )

        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "post-init"))

        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        var pingJson = waitForPingContent("events", null, server)!!
        checkPingSchema(pingJson)
        assertNotNull(pingJson.opt("events"))

        // This event comes from disk from the prior "run"
        assertEquals(
            "startup",
            pingJson.getJSONObject("ping_info").getString("reason")
        )
        assertEquals(
            1,
            pingJson.getJSONArray("events").length()
        )
        assertEquals(
            "run1",
            pingJson.getJSONArray("events").getJSONObject(0).getJSONObject("extra").getString("someExtra")
        )

        Glean.submitPingByName("events", "background")

        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        pingJson = waitForPingContent("events", null, server)!!
        checkPingSchema(pingJson)
        assertNotNull(pingJson.opt("events"))

        // This event comes from the pre-initialization event
        assertEquals(
            "background",
            pingJson.getJSONObject("ping_info").getString("reason")
        )
        assertEquals(
            2,
            pingJson.getJSONArray("events").length()
        )
        assertEquals(
            "pre-init",
            pingJson.getJSONArray("events").getJSONObject(0).getJSONObject("extra").getString("someExtra")
        )
        assertEquals(
            "post-init",
            pingJson.getJSONArray("events").getJSONObject(1).getJSONObject("extra").getString("someExtra")
        )
    }

    @Test
    fun `Long extra values record an error`() {
        // Define a 'click' event, which will be stored in "store1"
        val click = EventMetricType<clickKeys>(
            disabled = false,
            category = "ui",
            lifetime = Lifetime.Ping,
            name = "click",
            sendInPings = listOf("store1"),
            allowedExtraKeys = listOf("object_id", "other")
        )

        val longString = "0123456789".repeat(11)

        click.record(extra = mapOf(clickKeys.objectId to longString))

        assertEquals(1, click.testGetNumRecordedErrors(ErrorType.InvalidOverflow))
    }

    @Test
    fun `overdue events are submitted in registered custom pings`() {
        val server = getMockWebServer()
        val context = getContextWithMockedInfo()
        delayMetricsPing(context)

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = true
        )

        val pingName = "another-ping"
        val event = EventMetricType<SomeExtraKeys>(
            disabled = false,
            category = "telemetry",
            name = "test_event",
            lifetime = Lifetime.Ping,
            sendInPings = listOf(pingName),
            allowedExtraKeys = listOf("someExtra")
        )

        // Let's record a single event. This will be queued up but not be sent.
        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "alternative"))
        assertEquals(1, event.testGetValue().size)

        // Let's act as if the app was stopped
        Glean.testDestroyGleanHandle()

        // Now create and register a ping before Glean.initialize
        @Suppress("UNUSED_VARIABLE")
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf())

        // Reset Glean
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = false
        )

        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        val request = server.takeRequest(20L, TimeUnit.SECONDS)
        val docType = request.path.split("/")[3]
        assertEquals(pingName, docType)

        val pingJsonData = request.getPlainBody()
        val pingJson = JSONObject(pingJsonData)
        checkPingSchema(pingJson)
        assertNotNull(pingJson.opt("events"))

        // This event comes from disk from the prior "run"
        assertEquals(
            1,
            pingJson.getJSONArray("events").length()
        )
        assertEquals(
            "alternative",
            pingJson.getJSONArray("events").getJSONObject(0).getJSONObject("extra").getString("someExtra")
        )
    }

    @Test
    fun `overdue events are discarded if ping is not registered`() {
        // This is similar to the above test,
        // except that we register the custom ping AFTER initialize.
        // Overdue events are thus discarded because the ping is unknown at initialization time.

        val server = getMockWebServer()
        val context = getContextWithMockedInfo()
        delayMetricsPing(context)

        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = true
        )

        val pingName = "another-ping-2"
        val event = EventMetricType<SomeExtraKeys>(
            disabled = false,
            category = "telemetry",
            name = "test_event",
            lifetime = Lifetime.Ping,
            sendInPings = listOf(pingName),
            allowedExtraKeys = listOf("someExtra")
        )

        // Let's record a single event. This will be queued up but not be sent.
        event.record(extra = mapOf(SomeExtraKeys.SomeExtra to "alternative"))
        assertEquals(1, event.testGetValue().size)

        // Let's act as if the app was stopped
        Glean.testDestroyGleanHandle()

        // Reset Glean
        resetGlean(
            context,
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port
            ),
            clearStores = false
        )

        // Create and register a ping AFTER Glean.initialize
        @Suppress("UNUSED_VARIABLE")
        val ping = PingType<NoReasonCodes>(
            name = pingName,
            includeClientId = true,
            sendIfEmpty = false,
            reasonCodes = listOf())

        // Trigger worker task to upload the pings in the background
        triggerWorkManager(context)

        // We can't properly test the absence of data,
        // but we can try to receive one and that causes an exception if there is none.
        assertNull(waitForPingContent(pingName, null, server))

        // Now try to manually submit the ping.
        // No events should be left, thus we don't receive it.
        ping.submit()
        assertNull(waitForPingContent(pingName, null, server))
    }
}
