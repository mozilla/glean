/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import java.lang.NullPointerException
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanTimerId
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.`when`
import org.mockito.Mockito.spy

@RunWith(AndroidJUnit4::class)
class TimingDistributionMetricTypeTest {

    val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

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
            val id = metric.start()
            `when`(metric.getElapsedTimeNanos()).thenReturn(i)
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
            `when`(metric.getElapsedTimeNanos()).thenReturn(i)
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

    @Test
    fun `The accumulateSamples API correctly stores timing values`() {
        // Define a timing distribution metric
        val metric = TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution_samples",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        // Accumulate a few values
        val testSamples = (1L..3L).toList().toLongArray()
        metric.accumulateSamples(testSamples)

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store1"))
        val snapshot = metric.testGetValue("store1")
        val secondsToNanos = 1000L * 1000L * 1000L
        // Check the sum
        assertEquals(6L * secondsToNanos, snapshot.sum)

        // We should get a sample in 3 buckets.
        // These numbers are a bit magic, but they correspond to
        // `hist.sample_to_bucket_minimum(i * seconds_to_nanos)` for `i = 1..=3`,
        // which lives in the Rust code.
        assertEquals(1L, snapshot.values[984625593])
        assertEquals(1L, snapshot.values[1969251187])
        assertEquals(1L, snapshot.values[2784941737])
    }

    @Test
    fun `Starting a timer before initialization doesn't crash`() {
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        val metric = TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution_samples",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        val timerId = metric.start()
        Glean.initialize(context, true)
        metric.stopAndAccumulate(timerId)

        metric.testGetValue().sum >= 0
    }

    @Test
    fun `Starting and stopping a timer before initialization doesn't crash`() {
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        val metric = TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution_samples",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        val timerId = metric.start()
        metric.stopAndAccumulate(timerId)
        Glean.initialize(context, true)

        metric.testGetValue().sum >= 0
    }

    @Test
    fun `Stopping a non-existent timer records an error`() {
        val metric = TimingDistributionMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Ping,
            name = "timing_distribution_samples",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        metric.stopAndAccumulate(GleanTimerId(-1))
        assertEquals(1, metric.testGetNumRecordedErrors(ErrorType.InvalidState))
    }
}
