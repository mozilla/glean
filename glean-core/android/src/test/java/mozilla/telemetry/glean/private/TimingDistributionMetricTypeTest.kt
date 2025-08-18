/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanBuildInfo
import mozilla.telemetry.glean.GleanTimerId
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import java.lang.NullPointerException

@RunWith(AndroidJUnit4::class)
class TimingDistributionMetricTypeTest {
    val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API saves to its storage engine`() {
        // Define a timing distribution metric which will be stored in "store1"
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )

        // Accumulate a few values
        for (i in 1L..3L) {
            val id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded.
        val snapshot = metric.testGetValue()!!
        // Check the sum
        assertTrue(snapshot.sum > 0L)
        assertEquals(snapshot.count, 3L)
        // Check that the 1L fell into the first bucket (max 1)
        // assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket (max 2)
        // assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket (max 3)
        // assertEquals(1L, snapshot.values[3])
    }

    @Test
    fun `disabled timing distributions must not record data`() {
        // Define a timing distribution metric which will be stored in "store1"
        // It's lifetime is set to Lifetime.PING SO IT SHOULD NOT RECORD ANYTHING.
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = true,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )

        val id = metric.start()
        metric.stopAndAccumulate(id)

        // Check that nothing was recorded.
        assertNull(
            "Disabled TimingDistributions should not record data.",
            metric.testGetValue(),
        )
    }

    @Test
    fun `testGetValue() returns null if nothing is stored`() {
        // Define a timing distribution metric which will be stored in "store1"
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )
        assertNull(metric.testGetValue())
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a timing distribution metric which will be stored in multiple stores
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution",
                sendInPings = listOf("store1", "store2", "store3"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )

        // Accumulate a few values
        for (i in 1L..3L) {
            val id = metric.start()
            metric.stopAndAccumulate(id)
        }

        // Check that data was properly recorded in the second ping.
        val snapshot = metric.testGetValue("store2")!!
        // Check the sum
        assertTrue(snapshot.sum > 0)
        assertEquals(snapshot.count, 3L)
        // Check that the 1L fell into the first bucket
        // assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket
        // assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket
        // assertEquals(1L, snapshot.values[3])

        // Check that data was properly recorded in the third ping.
        val snapshot2 = metric.testGetValue("store3")!!
        // Check the sum
        assertEquals(snapshot.sum, snapshot2.sum)
        assertEquals(snapshot2.count, 3L)
        // Check that the 1L fell into the first bucket
        // assertEquals(1L, snapshot2.values[1])
        // Check that the 2L fell into the second bucket
        // assertEquals(1L, snapshot2.values[2])
        // Check that the 3L fell into the third bucket
        // assertEquals(1L, snapshot2.values[3])
    }

    @Test
    fun `The accumulateSamples APIs correctly store timing values`() {
        // Define a timing distribution metric
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.SECOND,
        )

        // Accumulate a few values
        val testSamples = (1L..3L).toList()
        metric.accumulateSamples(testSamples)

        // Check that data was properly recorded in the second ping.
        val snapshot = metric.testGetValue("store1")!!
        val secondsToNanos = 1000L * 1000L * 1000L
        // Check the sum
        assertEquals(6L * secondsToNanos, snapshot.sum)

        // Check that we got the right number of samples.
        assertEquals(snapshot.count, 3L)

        // We should get a sample in 3 buckets.
        // These numbers are a bit magic, but they correspond to
        // `hist.sample_to_bucket_minimum(i * seconds_to_nanos)` for `i = 1..=3`,
        // which lives in the Rust code.
        assertEquals(1L, snapshot.values[984625593])
        assertEquals(1L, snapshot.values[1969251187])
        assertEquals(1L, snapshot.values[2784941737])

        // Assure the single sample API properly records.
        metric.accumulateSingleSample(4L)

        // Check that this new data was properly recorded in the second ping.
        val snapshotTwo = metric.testGetValue("store1")!!
        // Check the sum
        assertEquals(10L * secondsToNanos, snapshotTwo.sum)

        // Check that we got the right number of samples.
        assertEquals(snapshotTwo.count, 4L)
    }

    @Test
    fun `Starting a timer before initialization doesn't crash`() {
        Glean.testDestroyGleanHandle()

        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.SECOND,
        )

        val timerId = metric.start()
        Glean.initialize(context, true, buildInfo = GleanBuildInfo.buildInfo)
        metric.stopAndAccumulate(timerId)

        assertTrue(metric.testGetValue()!!.sum >= 0)
    }

    @Test
    fun `Starting and stopping a timer before initialization doesn't crash`() {
        Glean.testDestroyGleanHandle()

        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.SECOND,
        )

        val timerId = metric.start()
        metric.stopAndAccumulate(timerId)
        Glean.initialize(context, true, buildInfo = GleanBuildInfo.buildInfo)

        assertTrue(metric.testGetValue()!!.sum >= 0)
    }

    @Test
    fun `Stopping a non-existent timer records an error`() {
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.SECOND,
        )

        metric.stopAndAccumulate(GleanTimerId(1337UL))
        assertEquals(1, metric.testGetNumRecordedErrors(ErrorType.INVALID_STATE))
    }

    @Test
    fun `measure function correctly measures values`() {
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )

        // Create a test function to "measure". This works by mocking the getElapsedNanos return
        // value setting it to return a known value to make it easier to validate.
        fun testFunc(value: Long): Long = value

        // Accumulate a few values
        for (i in 1L..3L) {
            // Measure the test function, capturing the value to verify we correctly return the
            // value of the underlying function.
            val testValue = metric.measure {
                testFunc(i)
            }

            assertEquals("Returned value must match", i, testValue)
        }

        // Check that data was properly recorded.
        val snapshot = metric.testGetValue()!!
        // Check the sum
        assertTrue(snapshot.sum > 0L)
        assertEquals(snapshot.count, 3L)
        // Check that the 1L fell into the first bucket (max 1)
        // assertEquals(1L, snapshot.values[1])
        // Check that the 2L fell into the second bucket (max 2)
        // assertEquals(1L, snapshot.values[2])
        // Check that the 3L fell into the third bucket (max 3)
        // assertEquals(1L, snapshot.values[3])
    }

    @Test
    fun `measure function does not change behavior with early return`() {
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "inlined",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.NANOSECOND,
        )

        // We define a function that measures the whole function call runtime
        fun testFunc(): Long =
            metric.measure {
                // We want to simulate an early return.
                if (true) {
                    // Blank 'return' is not allowed here, because `measure` is not inlined.
                    // We can return by label though.
                    return@measure 17
                }

                42
            }

        val res = testFunc()
        assertEquals("Test value must match", 17, res)

        val snapshot = metric.testGetValue()!!
        assertTrue("Should have stored some nanoseconds", snapshot.sum > 0L)

        assertEquals(snapshot.count, 1L)
    }

    @Test
    fun `measure function bubbles up exceptions and timing is canceled`() {
        val metric = TimingDistributionMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.PING,
                name = "timing_distribution_samples",
                sendInPings = listOf("store1"),
            ),
            timeUnit = TimeUnit.SECOND,
        )

        // Create a test function that throws a NPE
        fun testFunc(): Unit = throw NullPointerException()

        // Attempt to measure the function that will throw an exception.  The `measure` function
        // should allow the exception to bubble up, the timespan measurement is canceled.
        try {
            metric.measure {
                testFunc()
            }
        } catch (e: Exception) {
            // Ensure that the exception was a NPE
            assertTrue("Exception type must match", e is NullPointerException)
        } finally {
            // Check that no data was recorded.
            assertNull("Metric must not have a value", metric.testGetValue())
        }
    }
}
