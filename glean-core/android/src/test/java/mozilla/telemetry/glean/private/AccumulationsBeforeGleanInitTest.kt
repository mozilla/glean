/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.Dispatchers
import mozilla.telemetry.glean.config.Configuration
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Note that this test file MUST NOT use the `GleanTestRule` as it requires metric
 * accumulation to happen before Glean is initialized.
 **/

@RunWith(AndroidJUnit4::class)
class AccumulationsBeforeGleanInitTest {

    val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @After
    @Before
    fun cleanup() {
        Glean.testDestroyGleanHandle()
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTaskQueueing(true)
        WorkManagerTestInitHelper.initializeTestWorkManager(context)
    }

    private fun forceInitGlean() {
        Glean.enableTestingMode()
        Glean.initialize(context, true, Configuration())
    }

    @Test
    fun `LabeledMetricTypes must allow accumulation before Glean inits`() {
        val counterMetric = CounterMetricType(
            disabled = false,
            category = "test.telemetry",
            lifetime = Lifetime.Application,
            name = "pre_init_counter",
            sendInPings = listOf("metrics")
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "test.telemetry",
            lifetime = Lifetime.Application,
            name = "pre_init_counter",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric
        )

        labeledCounterMetric["label1"].add(1)

        forceInitGlean()

        assertEquals(1, labeledCounterMetric["label1"].testGetValue())
    }
}
