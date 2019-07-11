/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.ObsoleteCoroutinesApi
import mozilla.telemetry.glean.resetGlean
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.lang.NullPointerException
import org.mockito.Mockito.spy
import org.mockito.Mockito.`when`

@ObsoleteCoroutinesApi
@ExperimentalCoroutinesApi
@RunWith(RobolectricTestRunner::class)
class TimingDistributionMetricTypeTest {

    @Before
    fun setUp() {
        resetGlean()
    }

    @Test
    fun `The API saves to its storage engine`() {
        // Define a timing distribution metric which will be stored in "store1"
        val metric = spy(TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        ))

        // Accumulate a few values
        for (i in 1L..3L) {
            `when`(metric.getElapsedTimeNanos()).thenReturn(0L)
            var id = metric.start()
            `when`(metric.getElapsedTimeNanos()).thenReturn(i * 1000000L) // ms to ns
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        assertTrue(metric.testHasValue())
        val snapshot = metric.testGetValue()
        // Check the sum
        assertEquals(6L, snapshot.sum)
        // Check that the 1L fell into the first bucket (max 1)
        assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket (max 2)
        assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket (max 3)
        assertEquals(1L, snapshot.values[3])
    }

    @Test
    fun `disabled timing distributions must not record data`() {
        // Define a timing distribution metric which will be stored in "store1"
        // It's lifetime is set to Lifetime.Ping so it should not record anything.
        val metric = TimingDistributionMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Attempt to store the timespan using set
        val id = metric.start()
        metric.stopAndAccumulate(id)

        // Check that nothing was recorded.
        assertFalse("Disabled TimingDistributions should not record data.",
            metric.testHasValue())
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        // Define a timing distribution metric which will be stored in "store1"
        val metric = TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )
        metric.testGetValue()
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a timing distribution metric which will be stored in multiple stores
        val metric = spy(TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution",
            sendInPings = listOf("store1", "store2", "store3"),
            timeUnit = TimeUnit.Millisecond
        ))

        // Accumulate a few values
        for (i in 1L..3L) {
            `when`(metric.getElapsedTimeNanos()).thenReturn(0L)
            val id = metric.start()
            `when`(metric.getElapsedTimeNanos()).thenReturn(i * 1000000L) // ms to ns
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store2"))
        val snapshot = metric.testGetValue("store2")
        // Check the sum
        assertEquals(6L, snapshot.sum)
        // Check that the 1L fell into the first bucket
        assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket
        assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket
        assertEquals(1L, snapshot.values[3])

        // Check that data was properly recorded in the third ping.
        assertTrue(metric.testHasValue("store3"))
        val snapshot2 = metric.testGetValue("store3")
        // Check the sum
        assertEquals(6L, snapshot2.sum)
        // Check that the 1L fell into the first bucket
        assertEquals(1L, snapshot2.values[1])
        // Check that the 2L fell into the second bucket
        assertEquals(1L, snapshot2.values[2])
        // Check that the 3L fell into the third bucket
        assertEquals(1L, snapshot2.values[3])
    }
}
