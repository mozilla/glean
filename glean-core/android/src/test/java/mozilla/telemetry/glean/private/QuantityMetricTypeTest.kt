/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/* This file is based on the tests in the Glean android-components implementation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

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
class QuantityMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a 'quantityMetric' quantity metric, which will be stored in "store1"
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "quantity_metric",
                sendInPings = listOf("store1"),
            ),
        )

        assertNull(quantityMetric.testGetValue())

        // Add to the quantity a couple of times.
        // calling add() without parameters to test increment by 1.
        quantityMetric.set(1L)

        // Check that the count was incremented and properly recorded.
        assertEquals(1L, quantityMetric.testGetValue())

        quantityMetric.set(10L)
        // Check that count was incremented and properly recorded.  This second call will check
        // calling add() with 10 to test increment by other amount
        assertEquals(10L, quantityMetric.testGetValue())
    }

    @Test
    fun `quantities with no lifetime must not record data`() {
        // Define a 'quantityMetric' quantity metric, which will be stored in "store1".
        // It's disabled so it should not record anything.
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "quantity_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Attempt to increment the quantity
        quantityMetric.set(1L)
        // Check that nothing was recorded.
        assertNull(
            "Quantities must not be recorded if they are disabled",
            quantityMetric.testGetValue(),
        )
    }

    @Test
    fun `disabled quantities must not record data`() {
        // Define a 'quantityMetric' quantity metric, which will be stored in "store1".  It's disabled
        // so it should not record anything.
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "quantity_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Attempt to store the quantity.
        quantityMetric.set(1L)
        // Check that nothing was recorded.
        assertNull(
            "Quantities must not be recorded if they are disabled",
            quantityMetric.testGetValue(),
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "quantity_metric",
                sendInPings = listOf("store1"),
            ),
        )
        assertNull(quantityMetric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'quantityMetric' quantity metric, which will be stored in "store1" and "store2"
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "quantity_metric",
                sendInPings = listOf("store1", "store2"),
            ),
        )

        quantityMetric.set(1L)

        assertEquals(1L, quantityMetric.testGetValue("store2"))

        quantityMetric.set(10L)
        assertEquals(10L, quantityMetric.testGetValue("store2"))
    }

    @Test
    fun `negative values are not recorded`() {
        // Define a 'quantityMetric' quantity metric, which will be stored in "store1"
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "quantity_metric",
                sendInPings = listOf("store1"),
            ),
        )

        quantityMetric.set(-10L)
        // Check that quantity was NOT recorded
        assertNull(quantityMetric.testGetValue("store1"))

        // Make sure that the errors have been recorded
        assertEquals(1, quantityMetric.testGetNumRecordedErrors(ErrorType.INVALID_VALUE))
    }
}
