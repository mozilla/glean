/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore

import android.app.Application
import android.util.Log
import mozilla.telemetry.glean.Glean

private const val TAG = "Glean"

class GleanApplication : Application() {

    override fun onCreate() {
        super.onCreate()

        // Initialize the Glean library. Ideally, this is the first thing that
        // must be done right after enabling logging.
        Glean.initialize(applicationContext)
        Log.i(TAG, "glean initialized")
    }
}
