/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* This file is based on the tests in the Glean android-components implentation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

package mozilla.telemetry.glean.private

import android.os.SystemClock
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.checkPingSchema
import mozilla.telemetry.glean.getContextWithMockedInfo
import mozilla.telemetry.glean.getMockWebServer
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.triggerWorkManager
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.fail
import org.junit.Test
import org.junit.Rule
import org.junit.runner.RunWith
import java.lang.NullPointerException
import java.util.concurrent.TimeUnit

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
        assertEquals(true, Glean.getUploadEnabled())
        Glean.setUploadEnabled(true)
        eventMetric.record(mapOf(testNameKeys.testName to "event1"))
        val snapshot1 = eventMetric.testGetValue()
        assertEquals(1, snapshot1.size)
        Glean.setUploadEnabled(false)
        assertEquals(false, Glean.getUploadEnabled())
        eventMetric.record(mapOf(testNameKeys.testName to "event2"))
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

        resetGlean(getContextWithMockedInfo(), Glean.configuration.copy(
            serverEndpoint = "http://" + server.hostName + ":" + server.port,
            logPings = true
        ))

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
            getContextWithMockedInfo(),
            Glean.configuration.copy(
                serverEndpoint = "http://" + server.hostName + ":" + server.port,
                logPings = true
            ),
            clearStores = false
        )

        // Trigger worker task to upload the pings in the background
        PingUploadWorker.enqueueWorker()
        triggerWorkManager()

        val request = server.takeRequest(1L, TimeUnit.SECONDS)
        assertEquals("POST", request.method)
        val applicationId = "mozilla-telemetry-glean-test"
        assert(
            request.path.startsWith("/submit/$applicationId/events/")
        )
        val pingJsonData = request.body.readUtf8()
        val pingJson = JSONObject(pingJsonData)
        checkPingSchema(pingJson)
        assertNotNull(pingJson.opt("events"))
        assertEquals(
            1,
            pingJson.getJSONArray("events").length()
        )
    }
}
