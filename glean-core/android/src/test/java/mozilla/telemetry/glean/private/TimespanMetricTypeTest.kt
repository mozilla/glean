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
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Ignore
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.lang.NullPointerException

@RunWith(RobolectricTestRunner::class)
class TimespanMetricTypeTest {

    @Before
    fun setUp() {
        resetGlean()
    }

    @Ignore("TimespanMetricType is a stub")
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
        metric.start(this)
        metric.stopAndSum(this)

        // Check that data was properly recorded.
        assertTrue(metric.testHasValue())
        assertTrue(metric.testGetValue() >= 0)
    }

    @Ignore("TimespanMetricType is a stub")
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
        metric.start(this)
        metric.stopAndSum(this)

        // Let's also call cancel() to make sure it's a no-op.
        metric.cancel(this)

        // Check that data was not recorded.
        assertFalse("The API should not record a counter if metric is disabled",
            metric.testHasValue())
    }

    @Ignore("TimespanMetricType is a stub")
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
        metric.start(this)
        metric.cancel(this)
        metric.stopAndSum(this)

        // Check that data was not recorded.
        assertFalse("The API should not record a counter if metric is cancelled",
            metric.testHasValue())
        // TODO
        // assertEquals(1, testGetNumRecordedErrors(metric, ErrorType.InvalidValue))
    }

    @Ignore("TimespanMetricType is a stub")
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

    @Ignore("TimespanMetricType is a stub")
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
        metric.start(this)
        metric.stopAndSum(this)

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store2"))
        assertTrue(metric.testGetValue("store2") >= 0)
    }

    @Ignore("TimespanMetricType is a stub")
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
        metric.start(this)
        metric.start(this)
        metric.stopAndSum(this)

        // Check that data was properly recorded in the second ping.
        assertTrue(metric.testHasValue("store2"))
        assertTrue(metric.testGetValue("store2") >= 0)
        // TODO
        // assertEquals(1, testGetNumRecordedErrors(metric, ErrorType.InvalidValue))
    }
}
