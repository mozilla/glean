/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

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
class UrlMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a 'urlMetric' string metric, which will be stored in "store1"
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "url_metric",
                sendInPings = listOf("store1")
            )
        )

        // Record two URLs of the same type, with a little delay.
        urlMetric.set("glean://test")

        // Check that data was properly recorded.
        assertTrue(urlMetric.testHasValue())
        assertEquals("glean://test", urlMetric.testGetValue())

        urlMetric.set("glean://other")
        // Check that data was properly recorded.
        assertTrue(urlMetric.testHasValue())
        assertEquals("glean://other", urlMetric.testGetValue())
    }

    @Test
    fun `disabled urls must not record data`() {
        // Define a 'urlMetric' metric, which will be stored in "store1". It's disabled
        // so it should not record anything.
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "urlMetric",
                sendInPings = listOf("store1")
            )
        )

        // Attempt to store the URL.
        urlMetric.set("glean://notrecorded")
        // Check that nothing was recorded.
        assertFalse(
            "Url must not be recorded if they are disabled",
            urlMetric.testHasValue()
        )
    }

    @Test
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "urlMetric",
                sendInPings = listOf("store1")
            )
        )
        assertNull(urlMetric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'urlMetric' metric, which will be stored in "store1" and "store2"
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "url_metric",
                sendInPings = listOf("store1", "store2")
            )
        )

        urlMetric.set("glean://value")

        // Check that data was properly recorded in the second ping.
        assertTrue(urlMetric.testHasValue("store2"))
        assertEquals("glean://value", urlMetric.testGetValue("store2"))

        urlMetric.set("glean://overriddenValue")
        // Check that data was properly recorded in the second ping.
        assertTrue(urlMetric.testHasValue("store2"))
        assertEquals("glean://overriddenValue", urlMetric.testGetValue("store2"))
    }

    @Test
    fun `Setting a long URL records an error`() {
        // Define a 'urlMetric' URL metric, which will be stored in "store1" and "store2"
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "url_metric",
                sendInPings = listOf("store1", "store2")
            )
        )

        urlMetric.set("glean://" + "testing".repeat(2000))

        assertEquals(1, urlMetric.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW))
    }
}
