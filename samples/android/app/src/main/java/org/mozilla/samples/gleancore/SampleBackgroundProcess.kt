package org.mozilla.samples.gleancore

import android.app.Service
import android.content.Context
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

class SampleBackgroundProcess: Service() {
    private var mBinder: Binder = Binder()

    override fun onBind(p0: Intent?): IBinder? {
        initializeGlean()

        Custom.bgCounter.add()
        Pings.background.submit()

        return mBinder
    }

    private fun initializeGlean() {
        val customDataPath = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR).path
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

        Log.i("sample_bg_service", "Initialized Glean in background service")
    }

    companion object {
        internal const val GLEAN_DATA_DIR: String = "sample_background_service"

        @JvmStatic
        fun startService(c: Context) {
            c.applicationContext.startService(
                Intent(c.applicationContext, SampleBackgroundProcess::class.java),
            )
        }
    }
}