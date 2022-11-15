/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import kotlinx.coroutines.ExperimentalCoroutinesApi
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner

@ExperimentalCoroutinesApi
@RunWith(RobolectricTestRunner::class)
class CustomDistributionMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a custom distribution metric which will be stored in "store1"
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution",
                sendInPings = listOf("store1"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )

        // Accumulate a few values
        for (i in 1L..3L) {
            metric.accumulateSamples(listOf(i))
        }

        // Check that data was properly recorded.
        val snapshot = metric.testGetValue()!!
        // Check the sum
        assertEquals(6L, snapshot.sum)
        // Check that the 1L fell into the first value bucket
        assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second value bucket
        assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third value bucket
        assertEquals(1L, snapshot.values[3])
    }

    @Test
    fun `disabled custom distributions must not record data`() {
        // Define a custom distribution metric which will be stored in "store1"
        // It's disabled so it should not record anything.
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution",
                sendInPings = listOf("store1"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )

        // Attempt to store to the distribution
        metric.accumulateSamples(listOf(0L))

        // Check that nothing was recorded.
        assertNull(
            "Disabled CustomDistributions should not record data.",
            metric.testGetValue()
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        // Define a custom distribution metric which will be stored in "store1"
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution",
                sendInPings = listOf("store1"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )
        assertNull(metric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a custom distribution metric which will be stored in multiple stores
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution",
                sendInPings = listOf("store1", "store2", "store3"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )

        // Accumulate a few values
        metric.accumulateSamples(listOf(1L, 2L, 3L))

        // Check that data was properly recorded in the second ping.
        val snapshot = metric.testGetValue("store2")!!
        // Check the sum
        assertEquals(6L, snapshot.sum)
        // Check that the 1L fell into the first bucket
        assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket
        assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket
        assertEquals(1L, snapshot.values[3])

        // Check that data was properly recorded in the third ping.
        val snapshot2 = metric.testGetValue("store3")!!
        // Check the sum
        assertEquals(6L, snapshot2.sum)
        // Check that the 1L fell into the first bucket
        assertEquals(1L, snapshot2.values[1])
        // Check that the 2L fell into the second bucket
        assertEquals(1L, snapshot2.values[2])
        // Check that the 3L fell into the third bucket
        assertEquals(1L, snapshot2.values[3])
    }

    @Test
    fun `The accumulateSamples API correctly stores values`() {
        // Define a custom distribution metric which will be stored in multiple stores
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )

        // Accumulate a few values
        val testSamples = (1L..3L).toList()
        metric.accumulateSamples(testSamples)

        // Check that data was properly recorded in the second ping.
        val snapshot = metric.testGetValue("store1")!!
        // Check the sum
        assertEquals(6L, snapshot.sum)
        // Check that the 1L fell into the first bucket
        assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket
        assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket
        assertEquals(1L, snapshot.values[3])
    }

    @Test
    fun `Accumulating negative values records an error`() {
        // Define a custom distribution metric which will be stored in multiple stores
        val metric = CustomDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "custom_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            rangeMin = 0L,
            rangeMax = 60000L,
            bucketCount = 100,
            histogramType = HistogramType.EXPONENTIAL
        )

        // Accumulate a few values
        metric.accumulateSamples(listOf(-1L))

        assertEquals(1, metric.testGetNumRecordedErrors(ErrorType.INVALID_VALUE))
    }
}
