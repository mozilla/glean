/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.collectAndCheckPingSchema
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.testing.ErrorType
import mozilla.telemetry.glean.testing.GleanTestRule
import org.junit.Test
import org.junit.runner.RunWith
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Rule

@RunWith(AndroidJUnit4::class)
class LabeledMetricTypeTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @get:Rule
    val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())

    @Test
    fun `test labeled counter type`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric
        )

        labeledCounterMetric["label1"].add(1)
        labeledCounterMetric["label2"].add(2)

        // Record a regular non-labeled counter. This isn't normally
        // possible with the generated code because the subMetric is private,
        // but it's useful to test here that it works.
        counterMetric.add(3)

        assertTrue(labeledCounterMetric["label1"].testHasValue())
        assertEquals(1, labeledCounterMetric["label1"].testGetValue())

        assertTrue(labeledCounterMetric["label2"].testHasValue())
        assertEquals(2, labeledCounterMetric["label2"].testGetValue())

        assertTrue(counterMetric.testHasValue())
        assertEquals(3, counterMetric.testGetValue())

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")
        // Do the same checks again on the JSON structure
        assertEquals(
            1,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("label1")
        )
        assertEquals(
            2,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("label2")
        )
        assertEquals(
            3,
            json.getJSONObject("counter")
                .get("telemetry.labeled_counter_metric")
        )
    }

    @Test
    fun `test __other__ label with predefined labels`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
            labels = setOf("foo", "bar", "baz")
        )

        labeledCounterMetric["foo"].add(1)
        labeledCounterMetric["foo"].add(2)
        labeledCounterMetric["bar"].add(1)
        labeledCounterMetric["not_there"].add(1)
        labeledCounterMetric["also_not_there"].add(1)
        labeledCounterMetric["not_me"].add(1)

        assertEquals(3, labeledCounterMetric["foo"].testGetValue())
        assertEquals(1, labeledCounterMetric["bar"].testGetValue())
        assertFalse(labeledCounterMetric["baz"].testHasValue())
        // The rest all lands in the __other__ bucket
        assertEquals(3, labeledCounterMetric["not_there"].testGetValue())

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")
        // Do the same checks again on the JSON structure
        assertEquals(
            3,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("foo")
        )
        assertEquals(
            1,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("bar")
        )
        assertEquals(
            3,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("__other__")
        )
    }

    @Test
    fun `test __other__ label without predefined labels`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric
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

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")
        // Do the same checks again on the JSON structure
        assertEquals(
            2,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("label_0")
        )
        for (i in 1..15) {
            assertEquals(
                1,
                json.getJSONObject("labeled_counter")
                    .getJSONObject("telemetry.labeled_counter_metric")
                    .get("label_$i")
            )
        }
        assertEquals(
            5,
            json.getJSONObject("labeled_counter")
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("__other__")
        )
    }

    @Test
    fun `test __other__ label without predefined labels before Glean initialization`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric
        )

        // Make sure Glean isn't initialized, and turn task queueing on
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)

        for (i in 0..20) {
            labeledCounterMetric["label_$i"].add(1)
        }
        // Go back and record in one of the real labels again
        labeledCounterMetric["label_0"].add(1)

        // Initialize glean
        Glean.initialize(context, true)

        assertEquals(2, labeledCounterMetric["label_0"].testGetValue())
        for (i in 1..15) {
            assertEquals(1, labeledCounterMetric["label_$i"].testGetValue())
        }
        assertEquals(5, labeledCounterMetric["__other__"].testGetValue())
    }

    @Test
    fun `Ensure invalid labels on labeled counter go to __other__`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType<CounterMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric
        )

        labeledCounterMetric["notSnakeCase"].add(1)
        labeledCounterMetric[""].add(1)
        labeledCounterMetric["with/slash"].add(1)
        labeledCounterMetric["this_string_has_more_than_thirty_characters"].add(1)

        assertEquals(
            4,
            labeledCounterMetric.testGetNumRecordedErrors(
                ErrorType.InvalidLabel
            )
        )
        assertEquals(
            4,
            labeledCounterMetric["__other__"].testGetValue()
        )
    }

    @Test
    fun `Ensure invalid labels on labeled boolean go to __other__`() {
        val booleanMetric = BooleanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_boolean_metric",
            sendInPings = listOf("metrics")
        )

        val labeledBooleanMetric = LabeledMetricType<BooleanMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_boolean_metric",
            sendInPings = listOf("metrics"),
            subMetric = booleanMetric
        )

        labeledBooleanMetric["notSnakeCase"].set(true)
        labeledBooleanMetric[""].set(true)
        labeledBooleanMetric["with/slash"].set(true)
        labeledBooleanMetric["this_string_has_more_than_thirty_characters"].set(true)

        assertEquals(
            4,
            labeledBooleanMetric.testGetNumRecordedErrors(
                ErrorType.InvalidLabel
            )
        )
        assertEquals(
            true,
            labeledBooleanMetric["__other__"].testGetValue()
        )
    }

    @Test
    fun `Ensure invalid labels on labeled string go to __other__`() {
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics")
        )

        val labeledStringMetric = LabeledMetricType<StringMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics"),
            subMetric = stringMetric
        )

        labeledStringMetric["notSnakeCase"].set("foo")
        labeledStringMetric[""].set("foo")
        labeledStringMetric["with/slash"].set("foo")
        labeledStringMetric["this_string_has_more_than_thirty_characters"].set("foo")

        assertEquals(
            4,
            labeledStringMetric.testGetNumRecordedErrors(
                ErrorType.InvalidLabel
            )
        )
        assertEquals(
            "foo",
            labeledStringMetric["__other__"].testGetValue()
        )
    }

    @Test
    fun `Test labeled string metric type`() {
        val stringMetric = StringMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics")
        )

        val labeledStringMetric = LabeledMetricType<StringMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_metric",
            sendInPings = listOf("metrics"),
            subMetric = stringMetric
        )

        labeledStringMetric["label1"].set("foo")
        labeledStringMetric["label2"].set("bar")

        assertEquals("foo", labeledStringMetric["label1"].testGetValue())
        assertEquals("bar", labeledStringMetric["label2"].testGetValue())
    }

    @Test
    fun `Test labeled boolean metric type`() {
        val booleanMetric = BooleanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_list_metric",
            sendInPings = listOf("metrics")
        )

        val labeledBooleanMetric = LabeledMetricType<BooleanMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_list_metric",
            sendInPings = listOf("metrics"),
            subMetric = booleanMetric
        )

        labeledBooleanMetric["label1"].set(false)
        labeledBooleanMetric["label2"].set(true)

        assertFalse(labeledBooleanMetric["label1"].testGetValue())
        assertTrue(labeledBooleanMetric["label2"].testGetValue())
    }

    @Test(expected = IllegalStateException::class)
    fun `Test that labeled events are an exception`() {
        val eventMetric = EventMetricType<NoExtraKeys>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_event_metric",
            sendInPings = listOf("metrics")
        )

        val labeledEventMetric = LabeledMetricType<EventMetricType<NoExtraKeys>>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_event_metric",
            sendInPings = listOf("metrics"),
            subMetric = eventMetric
        )

        labeledEventMetric["label1"]
    }

    // SKIPPED `test seen labels get reloaded from disk`
    // REASON This is tested on the Rust side. The Kotlin side has no way to inject data into the database.

    @Test
    fun `test recording to static labels by label index`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_counter_metric",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
            labels = setOf("foo", "bar", "baz")
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
}
