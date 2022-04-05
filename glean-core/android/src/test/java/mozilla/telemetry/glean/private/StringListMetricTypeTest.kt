/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* This file is based on the tests in the Glean android-components implentation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class StringListMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine by first adding then setting`() {
        // Define a 'stringListMetric' string list metric, which will be stored in "store1"
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1")
            )
        )

        // Record two lists using add and set
        stringListMetric.add("value1")
        stringListMetric.add("value2")
        stringListMetric.add("value3")

        // Check that data was properly recorded.
        val snapshot = stringListMetric.testGetValue()!!
        assertEquals(3, snapshot.size)
        assertTrue(stringListMetric.testHasValue())
        assertEquals("value1", snapshot[0])
        assertEquals("value2", snapshot[1])
        assertEquals("value3", snapshot[2])

        // Use set() to see that the first list is replaced by the new list
        stringListMetric.set(listOf("other1", "other2", "other3"))
        // Check that data was properly recorded.
        val snapshot2 = stringListMetric.testGetValue()!!
        assertEquals(3, snapshot2.size)
        assertTrue(stringListMetric.testHasValue())
        assertEquals("other1", snapshot2[0])
        assertEquals("other2", snapshot2[1])
        assertEquals("other3", snapshot2[2])
    }

    @Test
    fun `The API saves to its storage engine by first setting then adding`() {
        // Define a 'stringListMetric' string list metric, which will be stored in "store1"
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1")
            )
        )

        // Record two lists using set and add
        stringListMetric.set(listOf("value1", "value2", "value3"))

        // Check that data was properly recorded.
        val snapshot = stringListMetric.testGetValue()!!
        assertEquals(3, snapshot.size)
        assertTrue(stringListMetric.testHasValue())
        assertEquals("value1", snapshot[0])
        assertEquals("value2", snapshot[1])
        assertEquals("value3", snapshot[2])

        // Use add() to see that the list is appended to
        stringListMetric.add("added1")
        // Check that data was properly recorded.
        val snapshot2 = stringListMetric.testGetValue()!!
        assertEquals(4, snapshot2.size)
        assertTrue(stringListMetric.testHasValue())
        assertEquals("value1", snapshot2[0])
        assertEquals("value2", snapshot2[1])
        assertEquals("value3", snapshot2[2])
        assertEquals("added1", snapshot2[3])
    }

    @Test
    fun `disabled lists must not record data`() {
        // Define a string list metric which will be stored in "store1".
        // It's disabled so it should not record anything.
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1")
            )
        )

        // Attempt to store the string list using set.
        stringListMetric.set(listOf("value1", "value2", "value3"))
        // Check that nothing was recorded.
        assertFalse(
            "StringLists must not be recorded if they are disabled",
            stringListMetric.testHasValue()
        )

        // Attempt to store the string list using add.
        stringListMetric.add("value4")
        // Check that nothing was recorded.
        assertFalse(
            "StringLists must not be recorded if they are disabled",
            stringListMetric.testHasValue()
        )
    }

    @Test
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1")
            )
        )
        assertNull(stringListMetric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'stringListMetric' string list metric, which will be stored in "store1" and "store2"
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1", "store2")
            )
        )

        // Record two lists using add and set
        stringListMetric.add("value1")
        stringListMetric.add("value2")
        stringListMetric.add("value3")

        // Check that data was properly recorded in the second ping.
        assertTrue(stringListMetric.testHasValue("store2"))
        val snapshot = stringListMetric.testGetValue("store2")!!
        assertEquals(3, snapshot.size)
        assertEquals("value1", snapshot[0])
        assertEquals("value2", snapshot[1])
        assertEquals("value3", snapshot[2])

        // Use set() to see that the first list is replaced by the new list.
        stringListMetric.set(listOf("other1", "other2", "other3"))
        // Check that data was properly recorded in the second ping.
        assertTrue(stringListMetric.testHasValue("store2"))
        val snapshot2 = stringListMetric.testGetValue("store2")!!
        assertEquals(3, snapshot2.size)
        assertEquals("other1", snapshot2[0])
        assertEquals("other2", snapshot2[1])
        assertEquals("other3", snapshot2[2])
    }

    @Test
    fun `Long string lists are truncated`() {
        // Define a 'stringListMetric' string list metric, which will be stored in "store1"
        val stringListMetric = StringListMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "string_list_metric",
                sendInPings = listOf("store1")
            )
        )

        for (x in 0..20) {
            stringListMetric.add("value$x")
        }

        val snapshot = stringListMetric.testGetValue("store1")!!
        assertEquals(20, snapshot.size)

        assertEquals(1, stringListMetric.testGetNumRecordedErrors(ErrorType.INVALID_VALUE))
    }
}
