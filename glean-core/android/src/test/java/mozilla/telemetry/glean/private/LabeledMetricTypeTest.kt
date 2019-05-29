/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/*
import android.content.Context
import android.content.SharedPreferences
import androidx.test.core.app.ApplicationProvider
*/
import mozilla.telemetry.glean.collectAndCheckPingSchema
import mozilla.telemetry.glean.resetGlean
import mozilla.telemetry.glean.GleanMetrics.Pings
import org.junit.After
import org.junit.Before
import org.junit.Test
import org.junit.Ignore
import org.junit.runner.RunWith
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
/*
import java.util.UUID
import org.mockito.ArgumentMatchers.anyString
import org.mockito.ArgumentMatchers.eq
import org.mockito.Mockito.`when`
import org.mockito.Mockito.doAnswer
import org.mockito.Mockito.doReturn
import org.mockito.Mockito.mock
import org.mockito.Mockito.spy
*/
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class LabeledMetricTypeTest {
    /*
    private data class GenericMetricType(
        override val disabled: Boolean,
        override val category: String,
        override val lifetime: Lifetime,
        override val name: String,
        override val sendInPings: List<String>
    ) : CommonMetricData
    */

    @Before
    fun setup() {
        resetGlean()
    }

    @After
    fun resetGlobalState() {
        /*Glean.setUploadEnabled(true)*/
    }

    @Test
    fun `test labeled counter type`() {
        // CountersStorageEngine.clearAllStores()

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

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        // Do the same checks again on the JSON structure
        assertEquals(
            1,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")!!
                .get("label1")
        )
        assertEquals(
            2,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")!!
                .get("label2")
        )
        assertEquals(
            3,
            json.getJSONObject("counter")!!
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

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        // Do the same checks again on the JSON structure
        assertEquals(
            3,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("foo")
        )
        assertEquals(
            1,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")
                .get("bar")
        )
        assertEquals(
            3,
            json.getJSONObject("labeled_counter")!!
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

        val json = collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        // Do the same checks again on the JSON structure
        assertEquals(
            2,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")!!
                .get("label_0")
        )
        for (i in 1..15) {
            assertEquals(
                1,
                json.getJSONObject("labeled_counter")!!
                    .getJSONObject("telemetry.labeled_counter_metric")!!
                    .get("label_$i")
            )
        }
        assertEquals(
            5,
            json.getJSONObject("labeled_counter")!!
                .getJSONObject("telemetry.labeled_counter_metric")!!
                .get("__other__")
        )
    }

    @Test
    fun `Ensure invalid labels go to __other__`() {
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

        // TODO: 1551975
        /*assertEquals(*/
            /*4,*/
            /*ErrorRecording.testGetNumRecordedErrors(*/
                /*labeledCounterMetric,*/
                /*ErrorRecording.ErrorType.InvalidValue*/
            /*)*/
        /*)*/
        assertEquals(
            4,
            labeledCounterMetric["__other__"].testGetValue()
        )
    }

    @Ignore("Timespan metric not yet implemented")
    @Test
    fun `Test labeled timespan metric type`() {
        /*
        TimespansStorageEngine.clearAllStores()

        val timespanMetric = TimespanMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_timespan_metric",
            sendInPings = listOf("metrics"),
            timeUnit = TimeUnit.Nanosecond
        )

        val labeledTimespanMetric = LabeledMetricType<TimespanMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_timespan_metric",
            sendInPings = listOf("metrics"),
            subMetric = timespanMetric
        )

        labeledTimespanMetric["label1"].start(this)
        labeledTimespanMetric["label1"].stopAndSum(this)
        labeledTimespanMetric["label2"].start(this)
        labeledTimespanMetric["label2"].stopAndSum(this)

        assertTrue(labeledTimespanMetric["label1"].testHasValue())

        collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        */
    }

    @Ignore("UUID metric not yet implemented")
    @Test
    fun `Test labeled uuid metric type`() {
        /*
        val uuidMetric = UuidMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_uuid_metric",
            sendInPings = listOf("metrics")
        )

        val labeledUuidMetric = LabeledMetricType<UuidMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_uuid_metric",
            sendInPings = listOf("metrics"),
            subMetric = uuidMetric
        )

        UuidsStorageEngine.record(labeledUuidMetric["label1"], UUID.randomUUID())
        UuidsStorageEngine.record(labeledUuidMetric["label2"], UUID.randomUUID())

        collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        */
    }

    @Ignore("String List metric not yet implemented")
    @Test
    fun `Test labeled string list metric type`() {
        /*
        StringListsStorageEngine.clearAllStores()

        val stringListMetric = StringListMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_list_metric",
            sendInPings = listOf("metrics")
        )

        val labeledStringListMetric = LabeledMetricType<StringListMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.Application,
            name = "labeled_string_list_metric",
            sendInPings = listOf("metrics"),
            subMetric = stringListMetric
        )

        StringListsStorageEngine.set(labeledStringListMetric["label1"], listOf("a", "b", "c"))
        StringListsStorageEngine.set(labeledStringListMetric["label2"], listOf("a", "b", "c"))

        collectAndCheckPingSchema(Pings.metrics).getJSONObject("metrics")!!
        */
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

    @Ignore("Events are not yet implemented")
    @Test(expected = IllegalStateException::class)
    fun `Test that we labeled events are an exception`() {
        /*
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
        */
    }

    @Ignore("LabeledMetricType is a stub")
    @Test
    fun `test seen labels get reloaded from disk`() {
        /*
        val persistedSample = mapOf(
            "store1#telemetry.labeled_metric/label1" to 1,
            "store1#telemetry.labeled_metric/label2" to 1,
            "store1#telemetry.labeled_metric/label3" to 1
        )

        // Create a fake application context that will be used to load our data.
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenAnswer { persistedSample }
        `when`(context.getSharedPreferences(
            eq(MockGenericStorageEngine::class.java.canonicalName),
            eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)
        `when`(context.getSharedPreferences(
            eq("${MockGenericStorageEngine::class.java.canonicalName}.PingLifetime"),
            eq(Context.MODE_PRIVATE)
        )).thenReturn(ApplicationProvider.getApplicationContext<Context>()
            .getSharedPreferences("${MockGenericStorageEngine::class.java.canonicalName}.PingLifetime",
                Context.MODE_PRIVATE))

        val storageEngine = MockGenericStorageEngine()
        storageEngine.applicationContext = context

        val metric = GenericMetricType(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.User,
            name = "labeled_metric",
            sendInPings = listOf("store1")
        )

        val labeledMetric = LabeledMetricType<GenericMetricType>(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.User,
            name = "labeled_metric",
            sendInPings = listOf("store1"),
            subMetric = metric
        )

        val labeledMetricSpy = spy(labeledMetric)
        doReturn(storageEngine).`when`(labeledMetricSpy).getStorageEngineForMetric()

        doAnswer {
            metric.copy(name = it.arguments[0] as String)
        }.`when`(labeledMetricSpy).getMetricWithNewName(anyString())

        for (i in 4..20) {
            storageEngine.record(labeledMetricSpy["label$i"], 1)
        }
        // Go back and record in one of the real labels again
        storageEngine.record(labeledMetricSpy["label1"], 2)

        val snapshot = storageEngine.getSnapshot(storeName = "store1", clearStore = false)

        assertEquals(17, snapshot!!.size)
        assertEquals(2, snapshot.get("telemetry.labeled_metric/label1"))
        for (i in 2..15) {
            assertEquals(1, snapshot.get("telemetry.labeled_metric/label$i"))
        }
        assertEquals(1, snapshot.get("telemetry.labeled_metric/__other__"))
        */
    }
}
