/* This file is based on the tests in the Glean android-components implentation.
 *
 * Care should be taken to not reorder elements in this file so it will be easier
 * to track changes in Glean android-components.
 */

package mozilla.telemetry.glean.private

// import mozilla.telemetry.glean.error.ErrorRecording.ErrorType
// import mozilla.telemetry.glean.error.ErrorRecording.testGetNumRecordedErrors
import mozilla.telemetry.glean.resetGlean
// import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.lang.NullPointerException

@RunWith(RobolectricTestRunner::class)
class TimespanMetricTypeTest {

    @Before
    fun setUp() {
        resetGlean()
    }

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
        assertTrue(metric.testGetValueAsUnit() >= 0)
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
        // TODO(bug 1556963)
        // assertEquals(1, testGetNumRecordedErrors(metric, ErrorType.InvalidValue))
    }

    @Test(expected = NullPointerException::class)
    fun `testGetValueAsUnit() throws NullPointerException if nothing is stored`() {
        val metric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "timespan_metric",
            sendInPings = listOf("store1"),
            timeUnit = TimeUnit.Millisecond
        )
        metric.testGetValueAsUnit()
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
        assertTrue(metric.testGetValueAsUnit("store2") >= 0)
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
        assertTrue(metric.testGetValueAsUnit("store2") >= 0)
        // TODO(bug 1556963)
        // assertEquals(1, testGetNumRecordedErrors(metric, ErrorType.InvalidValue))
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
        val value = metric.testGetValueAsUnit()

        metric.stop()

        assertEquals(value, metric.testGetValueAsUnit())
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
        assertEquals(6, metric.testGetValueAsUnit())
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
        assertEquals(6, metric.testGetValueAsUnit())

        metric.start()
        metric.stop()
        val value = metric.testGetValueAsUnit()
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
        val value = metric.testGetValueAsUnit()

        metric.setRawNanos(timespanNanos)

        assertEquals(value, metric.testGetValueAsUnit())
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

        assertNotEquals(timespanNanos, metric.testGetValueAsUnit())
    }
}
