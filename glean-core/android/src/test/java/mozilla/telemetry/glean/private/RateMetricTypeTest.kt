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
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class RateMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        val rateMetric = RateMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "rate_metric",
                sendInPings = listOf("store1")
            )
        )

        assertNull(rateMetric.testGetValue())

        rateMetric.addToNumerator(2)
        rateMetric.addToDenominator(5)

        // Check that the count was incremented and properly recorded.
        assertEquals(Rate(2, 5), rateMetric.testGetValue())

        rateMetric.addToNumerator(1)
        assertEquals(Rate(3, 5), rateMetric.testGetValue())
    }

    @Test
    fun `disabled rates must not record data`() {
        val rateMetric = RateMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "rate_metric",
                sendInPings = listOf("store1")
            )
        )

        rateMetric.addToNumerator(1)
        rateMetric.addToDenominator(1)
        assertNull(
            "rates must not be recorded if they are disabled",
            rateMetric.testGetValue()
        )
    }

    @Test
    fun `rate with external denominator`() {
        val meta1 = CommonMetricData(
            category = "telemetry",
            name = "rate1",
            sendInPings = listOf("store1"),
            lifetime = Lifetime.APPLICATION,
            disabled = false
        )

        val meta2 = CommonMetricData(
            category = "telemetry",
            name = "rate2",
            sendInPings = listOf("store1"),
            lifetime = Lifetime.APPLICATION,
            disabled = false
        )

        val denom = DenominatorMetricType(
            CommonMetricData(
                category = "telemetry",
                name = "counter",
                sendInPings = listOf("store1"),
                lifetime = Lifetime.APPLICATION,
                disabled = false
            ),
            listOf(meta1, meta2)
        )

        val num1 = NumeratorMetricType(meta1)
        val num2 = NumeratorMetricType(meta2)

        num1.addToNumerator(3)
        num2.addToNumerator(5)

        denom.add(7)

        assertEquals(Rate(3, 7), num1.testGetValue())
        assertEquals(Rate(5, 7), num2.testGetValue())
    }
}
