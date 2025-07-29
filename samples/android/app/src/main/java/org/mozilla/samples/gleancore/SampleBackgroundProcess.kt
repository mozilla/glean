/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package org.mozilla.samples.gleancore

import android.app.Service
import android.content.Intent
import android.os.Binder
import android.os.IBinder
import android.util.Log
import mozilla.telemetry.glean.BuildInfo
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.config.Configuration
import org.mozilla.samples.gleancore.GleanMetrics.Custom
import org.mozilla.samples.gleancore.GleanMetrics.Pings
import java.io.File
import java.util.Calendar

/**
 * A simple sample background service for the purpose of testing Glean running in a background
 * process. This service records a counter and then submits a ping as it starts.
 */
class SampleBackgroundProcess : Service() {
    /**
     * Required override, don't need to do anything here so we
     * just return a default Binder
     */
    override fun onBind(intent: Intent?): IBinder {
        return Binder()
    }

    /**
     * Entry point when the Service gets started by ServiceIntent
     */
    override fun onStartCommand(
        intent: Intent?,
        flags: Int,
        startId: Int,
    ): Int {
        Log.i(TAG, "Service Started by Intent")

        initializeGlean()

        Custom.bgCounter.add()
        Pings.background.submit()

        return super.onStartCommand(intent, flags, startId)
    }

    /**
     * Initialize Glean for the background process with a custom data path
     */
    private fun initializeGlean() {
        val customDataPath = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR).path
        Log.i(TAG, "Initializing Glean on background process with path: $customDataPath")

        Glean.registerPings(Pings)
        Glean.initialize(
            applicationContext = this.applicationContext,
            uploadEnabled = true,
            // GleanBuildInfo can only be generated for application,
            // We are in a library so we have to build it ourselves.
            buildInfo =
                BuildInfo(
                    "0.0.1",
                    "0.0.1",
                    Calendar.getInstance(),
                ),
            configuration =
                Configuration(
                    channel = "sample",
                    // When initializing Glean from outside the main process,
                    // we need to provide it with a dataPath manually.
                    dataPath = customDataPath,
                ),
        )

        Log.i(TAG, "Initialized Glean in background service")
    }

    companion object {
        // A custom data path to use for the background service
        internal const val GLEAN_DATA_DIR: String = "sample_background_service"

        // A log tag for background service log messages
        internal const val TAG: String = "sample_bg_service"
    }
}
