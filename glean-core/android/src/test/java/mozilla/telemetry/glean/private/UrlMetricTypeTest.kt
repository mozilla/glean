/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
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
                sendInPings = listOf("store1"),
            ),
        )

        // Record two URLs of the same type, with a little delay.
        urlMetric.set("glean://test")

        // Check that data was properly recorded.
        assertEquals("glean://test", urlMetric.testGetValue())

        urlMetric.set("glean://other")
        // Check that data was properly recorded.
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
                sendInPings = listOf("store1"),
            ),
        )

        // Attempt to store the URL.
        urlMetric.set("glean://notrecorded")
        // Check that nothing was recorded.
        assertNull(
            "Url must not be recorded if they are disabled",
            urlMetric.testGetValue(),
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        val urlMetric = UrlMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "urlMetric",
                sendInPings = listOf("store1"),
            ),
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
                sendInPings = listOf("store1", "store2"),
            ),
        )

        urlMetric.set("glean://value")

        // Check that data was properly recorded in the second ping.
        assertEquals("glean://value", urlMetric.testGetValue("store2"))

        urlMetric.set("glean://overriddenValue")
        // Check that data was properly recorded in the second ping.
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
                sendInPings = listOf("store1", "store2"),
            ),
        )

        // Whenever the URL is longer than our MAX_URL_LENGTH, we truncate the URL to the
        // MAX_URL_LENGTH.
        //
        // This 8-character string was chosen so we could have an even number that is
        // a divisor of our MAX_URL_LENGTH.
        val longPathBase = "abcdefgh"

        val testUrl = "glean://" + longPathBase.repeat(2000)

        // Using 2000 creates a string > 16000 characters, well over MAX_URL_LENGTH.
        urlMetric.set(testUrl)

        // "glean://" is 8 characters
        // "abcdefgh" (longPathBase) is 8 characters
        // `longPathBase` is repeated 1023 times (8184)
        // 8 + 8184 = 8192 (MAX_URL_LENGTH)
        val expected = "glean://" + longPathBase.repeat(1023)

        assertEquals(urlMetric.testGetValue("store1"), expected)
        assertEquals(1, urlMetric.testGetNumRecordedErrors(ErrorType.INVALID_OVERFLOW))
    }
}
