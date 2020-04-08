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
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class TimespanMetricTypeTest {

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `The API must record to its storage engine`() {
        // Define a timespan metric, which will be stored in "store1"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        // Check that data was properly recorded.
        assertTrue(metric.testHasValue())
        assertTrue(metric.testGetValue() >= 0)
    }

    @Test
    fun `The API should not record if the metric is disabled`() {
        // Define a timespan metric, which will be stored in "store1"
        val metric = TimespanMetricType(
            disabled = true,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        // Let's also call cancel() to make sure it's a no-op.
        metric.cancel()

        // Check that data was not recorded.
        assertFalse("The API should not record a counter if metric is disabled",
            metric.testHasValue())
    }

    @Test
    fun `The API must correctly cancel`() {
        // Define a timespan metric, which will be stored in "store1"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Record a timespan.
        metric.start()
        metric.cancel()
        metric.stop()

        // Check that data was not recorded.
        assertFalse("The API should not record a counter if metric is cancelled",
            metric.testHasValue())
        assertEquals(1, metric.testGetNumRecordedErrors(ErrorType.InvalidState))
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValue() throws NullPointerException if nothing is stored`() {
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )
        metric.testGetValue()
    }

    @Test
    fun `The API saves to secondary pings`() {
        // Define a timespan metric, which will be stored in "store1" and "store2"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1", "store2"),
            timeUnit = TimeUnit.Millisecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store2"))
        assertTrue(metric.testGetValue("store2") >= 0)
    }

    @Test
    fun `Records an error if started twice`() {
        // Define a timespan metric, which will be stored in "store1" and "store2"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1", "store2"),
            timeUnit = TimeUnit.Millisecond
        )

        // Record a timespan.
        metric.start()
        metric.start()
        metric.stop()

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store2"))
        assertTrue(metric.testGetValue("store2") >= 0)
        assertEquals(1, metric.testGetNumRecordedErrors(ErrorType.InvalidState))
    }

    @Test
    fun `Value unchanged if stopped twice`() {
        // Define a timespan metric, which will be stored in "store1" and "store2"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Nanosecond
        )

        // Record a timespan.
        metric.start()
        metric.stop()
        assertTrue(metric.testHasValue())
        val value = metric.testGetValue()

        metric.stop()

        assertEquals(value, metric.testGetValue())
    }

    @Test
    fun `test setRawNanos`() {
        val timespanNanos = 6 * 1000000000L

        val metric = TimespanMetricType(
            false,
            "telemetry",
            Lifetime.Ping,
            "explicit_timespan",
            listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        metric.setRawNanos(timespanNanos)
        assertEquals(6, metric.testGetValue())
    }

    @Test
    fun `test setRawNanos followed by other API`() {
        val timespanNanos = 6 * 1000000000L

        val metric = TimespanMetricType(
            false,
            "telemetry",
            Lifetime.Ping,
            "explicit_timespan_1",
            listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        metric.setRawNanos(timespanNanos)
        assertEquals(6, metric.testGetValue())

        metric.start()
        metric.stop()
        val value = metric.testGetValue()
        assertEquals(6, value)
    }

    @Test
    fun `setRawNanos does not overwrite value`() {
        val timespanNanos = 6 * 1000000000L

        val metric = TimespanMetricType(
            false,
            "telemetry",
            Lifetime.Ping,
            "explicit_timespan_1",
            listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        metric.start()
        metric.stop()
        val value = metric.testGetValue()

        metric.setRawNanos(timespanNanos)

        assertEquals(value, metric.testGetValue())
    }

    @Test
    fun `setRawNanos does nothing when timer is running`() {
        val timespanNanos = 1000000000L

        val metric = TimespanMetricType(
            false,
            "telemetry",
            Lifetime.Ping,
            "explicit_timespan",
            listOf("store1"),
            timeUnit = TimeUnit.Second
        )

        metric.start()
        metric.setRawNanos(timespanNanos)
        metric.stop()

        // If setRawNanos worked, (which it's not supposed to in this case), it would
        // have recorded 1000000000 ns == 1s.  Make sure it's not that.
        assertNotEquals(1, metric.testGetValue())
    }

    @Test
    fun `measure function correctly measures values`() {
        // Define a timespan metric, which will be stored in "store1"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Create a function to measure, which also returns a value to test that we properly pass
        // along the returned value from the measure function
        fun testFunc(value: Boolean): Boolean {
            return value
        }

        // Capture returned value to determine if the function return value matches what is expected
        // and measure the test function, which should record to the metric
        val testValue = metric.measure {
            testFunc(true)
        }

        // Make sure the returned valued matches the expected value of "true"
        assertTrue("Test value must match", testValue)

        // Check that data was properly recorded.
        assertTrue("Metric must have a value", metric.testHasValue())
        assertTrue("Metric value must be greater than zero", metric.testGetValue() >= 0)
    }

    @Test
    fun `measure function bubbles up exceptions and timing is canceled`() {
        // Define a timespan metric, which will be stored in "store1"
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )

        // Create a function that will throw a NPE
        fun testFunc() {
            throw NullPointerException()
        }

        // Attempt to measure the function that will throw an exception.  The `measure` function
        // should allow the exception to bubble up, the timespan measurement is canceled.
        try {
            metric.measure {
                testFunc()
            }
        } catch (e: Exception) {
            // Make sure we caught the right kind of exception: NPE
            assertTrue("Exception type must match", e is NullPointerException)
        } finally {
            assertTrue("Metric must not have a value", !metric.testHasValue())
        }
    }
}
