/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.glean_rs

import android.os.Bundle
import android.util.Log
import android.support.v7.app.AppCompatActivity
import kotlinx.android.synthetic.main.activity_main.*
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.private.BooleanMetricType
import mozilla.telemetry.glean.private.Lifetime

internal object Metrics {
    /**
     * internal metric
     */
    val isStarted: BooleanMetricType by lazy {
        BooleanMetricType(
            disabled = false,
            category = "core",
            lifetime = Lifetime.Application,
            name = "is_started",
            sendInPings = listOf("baseline")
        )
    }
}


private const val TAG = "Glean.rs"

open class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        // Generate an event when user clicks on the button.
        buttonGenerateData.setOnClickListener {
            Log.i(TAG, "increment happened")
            Metrics.isStarted.set(true)
            Glean.collect("baseline")
        }
    }
}
