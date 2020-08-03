/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore

import android.app.Application
import android.util.Log
import mozilla.telemetry.glean.Glean
import org.mozilla.samples.gleancore.GleanMetrics.Basic
import org.mozilla.samples.gleancore.GleanMetrics.Test
import org.mozilla.samples.gleancore.GleanMetrics.Custom
import org.mozilla.samples.gleancore.GleanMetrics.LegacyIds
import org.mozilla.samples.gleancore.GleanMetrics.Pings
import java.util.UUID

private const val TAG = "Glean"

class GleanApplication : Application() {

    override fun onCreate() {
        super.onCreate()

        // Register the sample application's custom pings.
        Glean.registerPings(Pings)

        // Set a "fake" legacy client id for the purpose of testing the deletion-request ping payload
        LegacyIds.clientId.set(UUID.fromString("01234567-89ab-cdef-0123-456789abcdef"))

        // Initialize the Glean library. Ideally, this is the first thing that
        // must be done right after enabling logging.
        Glean.initialize(applicationContext = applicationContext, uploadEnabled = true)

        Test.timespan.start()

        Custom.counter.add()

        // Set a sample value for a metric.
        Basic.os.set("Android")

        Log.i(TAG, "Glean initialized")
    }
}
