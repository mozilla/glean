/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.jsonArray
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@Serializable
data class BalloonsObject(var items: MutableList<BalloonsObjectItem> = mutableListOf()) : ObjectSerialize {
    fun add(elem: BalloonsObjectItem) = items.add(elem)

    fun addAll(elements: Collection<BalloonsObjectItem>) = items.addAll(elements)

    fun clear() = items.clear()

    fun remove(element: BalloonsObjectItem) = items.remove(element)

    fun removeAll(elements: Collection<BalloonsObjectItem>) = items.removeAll(elements)

    fun removeAt(index: Int) = items.removeAt(index)

    fun set(index: Int, element: BalloonsObjectItem) = items.set(index, element)

    override fun intoSerializedObject(): String {
        return Json.encodeToString(items)
    }
}

@Serializable
data class BalloonsObjectItem(var colour: String? = null, var diameter: Int? = null)

@RunWith(AndroidJUnit4::class)
class ObjectMetricTypeTest {
    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API records to its storage engine`() {
        val metric = ObjectMetricType<BalloonsObject>(
            CommonMetricData(
                category = "test",
                name = "balloon",
                lifetime = Lifetime.PING,
                sendInPings = listOf("store1"),
                disabled = false,
            ),
        )

        var balloons = BalloonsObject()
        balloons.add(BalloonsObjectItem(colour = "red", diameter = 5))
        balloons.add(BalloonsObjectItem(colour = "green"))
        metric.set(balloons)

        val snapshot = metric.testGetValue()!!

        assertEquals(2, snapshot.jsonArray.size)
        val expectedJson = """
        [
            { "colour": "red", "diameter": 5 },
            { "colour": "green" }
        ]
        """
        val expected: JsonElement = Json.decodeFromString(expectedJson)

        assertEquals(expected, snapshot)
    }

    @Test
    fun `disabled objects must not record data`() {
        val metric = ObjectMetricType<BalloonsObject>(
            CommonMetricData(
                category = "test",
                name = "balloon",
                lifetime = Lifetime.PING,
                sendInPings = listOf("store1"),
                disabled = true,
            ),
        )

        var balloons = BalloonsObject()
        balloons.add(BalloonsObjectItem(colour = "yellow", diameter = 10))
        metric.set(balloons)

        // Check that nothing was recorded.
        assertNull(
            "Objects must not be recorded if they are disabled",
            metric.testGetValue(),
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        val metric = ObjectMetricType<BalloonsObject>(
            CommonMetricData(
                category = "test",
                name = "testballoon",
                lifetime = Lifetime.PING,
                sendInPings = listOf("store1"),
                disabled = false,
            ),
        )
        assertNull(metric.testGetValue())
    }

    @Test
    fun `The API records to secondary pings`() {
        val metric = ObjectMetricType<BalloonsObject>(
            CommonMetricData(
                category = "test",
                name = "balloon",
                lifetime = Lifetime.PING,
                sendInPings = listOf("store1", "store2"),
                disabled = false,
            ),
        )

        var balloons = BalloonsObject()
        balloons.add(BalloonsObjectItem(colour = "red", diameter = 5))
        balloons.add(BalloonsObjectItem(colour = "green"))
        metric.set(balloons)

        val expectedJson = """
        [
            { "colour": "red", "diameter": 5 },
            { "colour": "green" }
        ]
        """
        val expected: JsonElement = Json.decodeFromString(expectedJson)

        // store1
        var snapshot = metric.testGetValue("store1")!!
        assertEquals(2, snapshot.jsonArray.size)
        assertEquals(expected, snapshot)

        // store2
        snapshot = metric.testGetValue("store2")!!
        assertEquals(2, snapshot.jsonArray.size)
        assertEquals(expected, snapshot)
    }

    @Test
    fun `objects should not record when upload is disabled`() {
        val metric = ObjectMetricType<BalloonsObject>(
            CommonMetricData(
                category = "test",
                name = "balloon",
                lifetime = Lifetime.PING,
                sendInPings = listOf("store1"),
                disabled = false,
            ),
        )

        Glean.setCollectionEnabled(true)

        var balloons = BalloonsObject()
        balloons.add(BalloonsObjectItem(colour = "yellow", diameter = 10))
        metric.set(balloons)

        var expectedJson = """
        [
            { "colour": "yellow", "diameter": 10 }
        ]
        """
        var expected: JsonElement = Json.decodeFromString(expectedJson)
        assertEquals(expected, metric.testGetValue()!!)

        Glean.setCollectionEnabled(false)
        balloons = BalloonsObject()
        balloons.add(BalloonsObjectItem(colour = "blue", diameter = 1))
        metric.set(balloons)

        // Check that nothing was recorded.
        assertNull(
            "Objects must not be recorded if they are disabled",
            metric.testGetValue(),
        )
    }
}
