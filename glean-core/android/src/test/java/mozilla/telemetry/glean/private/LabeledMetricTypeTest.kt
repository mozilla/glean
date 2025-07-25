/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class LabeledMetricTypeTest {
    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `test labeled counter type`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        labeledCounterMetric["label1"].add(1)
        labeledCounterMetric["label2"].add(2)

        // Record a regular non-labeled counter. This isn't normally
        // possible with the generated code because the subMetric is private,
        // but it's useful to test here that it works.
        counterMetric.add(3)

        assertEquals(1, labeledCounterMetric["label1"].testGetValue())
        assertEquals(2, labeledCounterMetric["label2"].testGetValue())
        assertEquals(3, counterMetric.testGetValue())
    }

    @Test
    fun `test __other__ label with predefined labels`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
            labels = setOf("foo", "bar", "baz"),
        )

        labeledCounterMetric["foo"].add(1)
        labeledCounterMetric["foo"].add(2)
        labeledCounterMetric["bar"].add(1)
        labeledCounterMetric["not_there"].add(1)
        labeledCounterMetric["also_not_there"].add(1)
        labeledCounterMetric["not_me"].add(1)

        assertEquals(3, labeledCounterMetric["foo"].testGetValue())
        assertEquals(1, labeledCounterMetric["bar"].testGetValue())
        assertNull(labeledCounterMetric["baz"].testGetValue())
        // The rest all lands in the __other__ bucket
        assertEquals(3, labeledCounterMetric["not_there"].testGetValue())
    }

    @Test
    fun `test __other__ label without predefined labels`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        for (i in 0..20) {
            labeledCounterMetric["label_$i"].add(1)
        }
        // Go back and record in one of the real labels again
        labeledCounterMetric["label_0"].add(1)

        assertEquals(2, labeledCounterMetric["label_0"].testGetValue())
        for (i in 1..15) {
            assertEquals(1, labeledCounterMetric["label_$i"].testGetValue())
        }
        assertEquals(5, labeledCounterMetric["__other__"].testGetValue())
    }

    @Test
    fun `test __other__ label without predefined labels before Glean initialization`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        // Make sure Glean isn't initialized, and turn task queueing on
        Glean.testDestroyGleanHandle()

        for (i in 0..20) {
            labeledCounterMetric["label_$i"].add(1)
        }
        // Go back and record in one of the real labels again
        labeledCounterMetric["label_0"].add(1)

        // Initialize glean
        resetGlean(clearStores = false)

        assertEquals(2, labeledCounterMetric["label_0"].testGetValue())
        for (i in 1..15) {
            assertEquals(1, labeledCounterMetric["label_$i"].testGetValue())
        }
        assertEquals(5, labeledCounterMetric["__other__"].testGetValue())
    }

    @Test
    fun `Ensure invalid labels on labeled counter go to __other__`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        // These are actually fine now.
        labeledCounterMetric["notSnakeCase"].add(1)
        labeledCounterMetric[""].add(1)
        labeledCounterMetric["with/slash"].add(1)
        labeledCounterMetric["this_string_has_more_than_thirty_characters"].add(1)
        labeledCounterMetric["Møøse"].add(1)

        assertEquals(
            0,
            labeledCounterMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            null,
            labeledCounterMetric["__other__"].testGetValue(),
        )

        // More than 111 characters? Not okay.
        labeledCounterMetric["1".repeat(112)].add(1)
        assertEquals(
            1,
            labeledCounterMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            1,
            labeledCounterMetric["__other__"].testGetValue(),
        )
    }

    @Test
    fun `Ensure invalid labels on labeled boolean go to __other__`() {
        val booleanMetric = BooleanMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_boolean_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledBooleanMetric = LabeledMetricType<BooleanMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_boolean_metric",
            sendInPings = listOf("metrics"),
            subMetric = booleanMetric,
        )

        // These are actually fine now.
        labeledBooleanMetric["notSnakeCase"].set(true)
        labeledBooleanMetric[""].set(true)
        labeledBooleanMetric["with/slash"].set(true)
        labeledBooleanMetric["this_string_has_more_than_thirty_characters"].set(true)
        labeledBooleanMetric["Møøse"].set(true)

        assertEquals(
            0,
            labeledBooleanMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            null,
            labeledBooleanMetric["__other__"].testGetValue(),
        )

        // More than 112 characters? Not okay.
        labeledBooleanMetric["1".repeat(112)].set(true)
        assertEquals(
            1,
            labeledBooleanMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            true,
            labeledBooleanMetric["__other__"].testGetValue(),
        )
    }

    @Test
    fun `Ensure invalid labels on labeled string go to __other__`() {
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_string_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledStringMetric = LabeledMetricType<StringMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics"),
            subMetric = stringMetric,
        )

        // These are actually fine now.
        labeledStringMetric["notSnakeCase"].set("foo")
        labeledStringMetric[""].set("foo")
        labeledStringMetric["with/slash"].set("foo")
        labeledStringMetric["this_string_has_more_than_thirty_characters"].set("foo")
        labeledStringMetric["Møøse"].set("foo")

        assertEquals(
            0,
            labeledStringMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            null,
            labeledStringMetric["__other__"].testGetValue(),
        )

        // More than 111 characters? Not okay.
        labeledStringMetric["1".repeat(112)].set("foo")
        assertEquals(
            1,
            labeledStringMetric.testGetNumRecordedErrors(
                ErrorType.INVALID_LABEL,
            ),
        )
        assertEquals(
            "foo",
            labeledStringMetric["__other__"].testGetValue(),
        )
    }

    @Test
    fun `Test labeled string metric type`() {
        val stringMetric = StringMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_string_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledStringMetric = LabeledMetricType<StringMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics"),
            subMetric = stringMetric,
        )

        labeledStringMetric["label1"].set("foo")
        labeledStringMetric["label2"].set("bar")

        assertEquals("foo", labeledStringMetric["label1"].testGetValue()!!)
        assertEquals("bar", labeledStringMetric["label2"].testGetValue()!!)
    }

    @Test
    fun `Test labeled boolean metric type`() {
        val booleanMetric = BooleanMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_boolean_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledBooleanMetric = LabeledMetricType<BooleanMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_boolean_metric",
            sendInPings = listOf("metrics"),
            subMetric = booleanMetric,
        )

        labeledBooleanMetric["label1"].set(false)
        labeledBooleanMetric["label2"].set(true)

        assertFalse(labeledBooleanMetric["label1"].testGetValue()!!)
        assertTrue(labeledBooleanMetric["label2"].testGetValue()!!)
    }

    @Test
    fun `Test labeled quantity metric type`() {
        val quantityMetric = QuantityMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_quantity_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledQuantityMetric = LabeledMetricType<QuantityMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_quantity_metric",
            sendInPings = listOf("metrics"),
            subMetric = quantityMetric,
        )

        labeledQuantityMetric["label1"].set(42)
        labeledQuantityMetric["label2"].set(43)

        assertEquals(42, labeledQuantityMetric["label1"].testGetValue()!!)
        assertEquals(43, labeledQuantityMetric["label2"].testGetValue()!!)
    }

    @Test(expected = IllegalStateException::class)
    fun `Test that labeled events are an exception`() {
        val eventMetric = EventMetricType<NoExtras>(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_event_metric",
                sendInPings = listOf("metrics"),
            ),
            allowedExtraKeys = emptyList(),
        )

        val labeledEventMetric = LabeledMetricType<EventMetricType<NoExtras>>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_event_metric",
            sendInPings = listOf("metrics"),
            subMetric = eventMetric,
        )

        labeledEventMetric["label1"]
    }

    // SKIPPED `test seen labels get reloaded from disk`
    // REASON This is tested on the Rust side. The Kotlin side has no way to inject data into the database.

    @Test
    fun `test recording to static labels by label index`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
            labels = setOf("foo", "bar", "baz"),
        )

        // Increment using a label name first.
        labeledCounterMetric["foo"].add(2)

        // Now only use label indices: "foo" first.
        labeledCounterMetric[0].add(1)
        // Then "bar".
        labeledCounterMetric[1].add(1)
        // Then some out of bound index: will go to "__other__".
        labeledCounterMetric[100].add(100)

        // Use the testing API to get the values for the labels.
        assertEquals(3, labeledCounterMetric["foo"].testGetValue())
        assertEquals(1, labeledCounterMetric["bar"].testGetValue())
        assertEquals(100, labeledCounterMetric["__other__"].testGetValue())
    }

    @Test
    fun `rapidly re-creating labeled metrics does not crash`() {
        // Regression test for bug 1733757.
        // The new implementation is different now,
        // but it still does the caching, so this is now a stress test of that implementation.

        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_nocrash_counter",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_nocrash",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
            labels = setOf("foo"),
        )

        // We go higher than the maximum of `(1<<15)-1 = 32767`.
        val maxAttempts = 1 shl 16
        for (ignored in 1..maxAttempts) {
            labeledCounterMetric["foo"].add(1)
        }

        assertEquals(maxAttempts, labeledCounterMetric["foo"].testGetValue())
        assertEquals(0, labeledCounterMetric.testGetNumRecordedErrors(ErrorType.INVALID_LABEL))
    }

    @Test
    fun `test labeled metric testGetLabeledValues`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "labeled_counter_metric",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        labeledCounterMetric["label1"].add(1)
        labeledCounterMetric["label2"].add(2)

        val labeledValues = labeledCounterMetric.testGetValue()
        assertEquals(2, labeledValues.size)
        assertEquals(1, labeledValues["telemetry.labeled_counter_metric/label1"])
        assertEquals(2, labeledValues["telemetry.labeled_counter_metric/label2"])
    }
}
