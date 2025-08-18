/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.GleanBuildInfo
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

    @Before
    fun setup() {
        Glean.testDestroyGleanHandle()
        WorkManagerTestInitHelper.initializeTestWorkManager(context)
    }

    @After
    fun cleanup() {
        Glean.testDestroyGleanHandle()

        // This closes the database to help prevent leaking it during tests.
        // See Bug1719905 for more info.
        WorkManagerTestInitHelper.closeWorkDatabase()
    }

    private fun forceInitGlean() {
        Glean.enableTestingMode()
        Glean.initialize(context, true, Configuration(), GleanBuildInfo.buildInfo)
    }

    @Test
    fun `LabeledMetricTypes must allow accumulation before Glean inits`() {
        val counterMetric = CounterMetricType(
            CommonMetricData(
                disabled = false,
                category = "test.telemetry",
                lifetime = Lifetime.APPLICATION,
                name = "pre_init_counter",
                sendInPings = listOf("metrics"),
            ),
        )

        val labeledCounterMetric = LabeledMetricType(
            disabled = false,
            category = "test.telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "pre_init_counter",
            sendInPings = listOf("metrics"),
            subMetric = counterMetric,
        )

        labeledCounterMetric["label1"].add(1)

        forceInitGlean()

        assertEquals(1, labeledCounterMetric["label1"].testGetValue())
    }
}
