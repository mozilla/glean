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
class CounterMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a 'counterMetric' counter metric, which will be stored in "store1"
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1"),
            ),
        )

        assertNull(counterMetric.testGetValue())

        // Add to the counter a couple of times with a little delay.  The first call will check
        // calling add() without parameters to test increment by 1.
        counterMetric.add()

        // Check that the count was incremented and properly recorded.
        assertEquals(1, counterMetric.testGetValue())

        counterMetric.add(10)
        // Check that count was incremented and properly recorded.  This second call will check
        // calling add() with 10 to test increment by other amount
        assertEquals(11, counterMetric.testGetValue())
    }

    @Test
    fun `disabled counters must not record data`() {
        // Define a 'counterMetric' counter metric, which will be stored in "store1".  It's disabled
        // so it should not record anything.
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Attempt to store the counter.
        counterMetric.add()
        // Check that nothing was recorded.
        assertNull(
            "Counters must not be recorded if they are disabled",
            counterMetric.testGetValue(),
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1"),
            ),
        )
        assertNull(counterMetric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a 'counterMetric' counter metric, which will be stored in "store1" and "store2"
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1", "store2"),
            ),
        )

        // Add to the counter a couple of times with a little delay.  The first call will check
        // calling add() without parameters to test increment by 1.
        counterMetric.add()

        // Check that the count was incremented and properly recorded for the second ping.
        assertEquals(1, counterMetric.testGetValue("store2"))

        counterMetric.add(10)
        // Check that count was incremented and properly recorded for the second ping.
        // This second call will check calling add() with 10 to test increment by other amount
        assertEquals(11, counterMetric.testGetValue("store2"))
    }

    @Test
    fun `negative values are not counted`() {
        // Define a 'counterMetric' counter metric, which will be stored in "store1"
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "counter_metric",
                sendInPings = listOf("store1"),
            ),
        )

        // Increment to 1 (initial value)
        counterMetric.add()

        // Check that the count was incremented
        assertEquals(1, counterMetric.testGetValue("store1"))

        counterMetric.add(-10)
        // Check that count was NOT incremented.
        assertEquals(1, counterMetric.testGetValue("store1"))
        assertEquals(1, counterMetric.testGetNumRecordedErrors(ErrorType.INVALID_VALUE))
    }
}
