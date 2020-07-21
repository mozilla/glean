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
import java.lang.NullPointerException
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class StringMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a 'stringMetric' string metric, which will be stored in "store1"
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "string_metric",
            sendInPings = listOf("store1")
        )

        // Record two strings of the same type, with a little delay.
        stringMetric.set("value")

        // Check that data was properly recorded.
        assertTrue(stringMetric.testHasValue())
        assertEquals("value", stringMetric.testGetValue())

        stringMetric.set("overriddenValue")
        // Check that data was properly recorded.
        assertTrue(stringMetric.testHasValue())
        assertEquals("overriddenValue", stringMetric.testGetValue())
    }

    @Test
    fun `disabled strings must not record data`() {
        // Define a 'stringMetric' string metric, which will be stored in "store1". It's disabled
        // so it should not record anything.
        val stringMetric = StringMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "stringMetric",
            sendInPings = listOf("store1")
        )

        // Attempt to store the string.
        stringMetric.set("value")
        // Check that nothing was recorded.
        assertFalse("Strings must not be recorded if they are disabled",
            stringMetric.testHasValue())
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val stringMetric = StringMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "stringMetric",
            sendInPings = listOf("store1")
        )
        stringMetric.testGetValue()
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'stringMetric' string metric, which will be stored in "store1" and "store2"
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "string_metric",
            sendInPings = listOf("store1", "store2")
        )

        // Record two strings of the same type, with a little delay.
        stringMetric.set("value")

        // Check that data was properly recorded in the second ping.
        assertTrue(stringMetric.testHasValue("store2"))
        assertEquals("value", stringMetric.testGetValue("store2"))

        stringMetric.set("overriddenValue")
        // Check that data was properly recorded in the second ping.
        assertTrue(stringMetric.testHasValue("store2"))
        assertEquals("overriddenValue", stringMetric.testGetValue("store2"))
    }

    @Test
    fun `Setting a long string records an error`() {
        // Define a 'stringMetric' string metric, which will be stored in "store1" and "store2"
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "string_metric",
            sendInPings = listOf("store1", "store2")
        )

        stringMetric.set("0123456789".repeat(11))

        assertEquals(1, stringMetric.testGetNumRecordedErrors(ErrorType.InvalidOverflow))
    }
}
