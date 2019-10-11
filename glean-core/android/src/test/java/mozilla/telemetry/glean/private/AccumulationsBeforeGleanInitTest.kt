/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Test
import org.junit.runner.RunWith

/**
 * Note that this test file MUST NOT use the `GleanTestRule` as it requires metric
 * accumulation to happen before Glean is initialized.
 **/

@RunWith(AndroidJUnit4::class)
class AccumulationsBeforeGleanInitTest {

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
    }

    @Test
    fun `TimingDistributionMetricType must allow accumulation before Glean inits`() {
        val timingDistribution = TimingDistributionMetricType(
            disabled = false,
            category = "test.telemetry",
            lifetime = Lifetime.Application,
            name = "pre_init_counter",
            sendInPings = listOf("metrics")
        )

        val id = timingDistribution.start()
        timingDistribution.stopAndAccumulate(id)
    }
}
