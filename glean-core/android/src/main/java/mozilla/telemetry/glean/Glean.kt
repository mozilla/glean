/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.util.Log
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.utils.getLocaleTag
import java.io.File
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.MetricHandle
import mozilla.telemetry.glean.rust.RustError
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import mozilla.telemetry.glean.GleanMetrics.GleanBaseline
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.scheduler.PingUploadWorker

open class GleanInternalAPI internal constructor () {
    companion object {
        private val LOG_TAG: String = "glean-kotlin"
    }

    // `internal` so this can be modified for testing
    internal var handle: MetricHandle = 0L

    internal lateinit var configuration: Configuration

    private lateinit var gleanDataDir: File

    /**
     * Initialize Glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using Glean. A message is logged to error and no changes are made
     * to the state if initialize is called a more than once.
     *
     * A LifecycleObserver will be added to send pings when the application goes
     * into the background.
     *
     * @param applicationContext [Context] to access application features, such
     * as shared preferences
     * @param configuration A Glean [Configuration] object with global settings.
     */
    fun initialize(applicationContext: Context, configuration: Configuration = Configuration()) {
        if (isInitialized()) {
            Log.e(LOG_TAG, "Glean should not be initialized multiple times")
            return
        }

        this.configuration = configuration

        this.gleanDataDir = File(applicationContext.applicationInfo.dataDir, "glean_data")
        handle = LibGleanFFI.INSTANCE.glean_initialize(this.gleanDataDir.path, applicationContext.packageName)

        // TODO: on glean-legacy we perform other actions before initialize the metrics (e.g.
        // init the engines), then init the core metrics, and finally kick off the metrics
        // schedulers. We should do something similar here as well.
        initializeCoreMetrics(applicationContext)
    }

    /**
     * Returns true if the Glean library has been initialized.
     */
    internal fun isInitialized(): Boolean {
        if (handle == 0L) {
            return false
        }

        val initialized = LibGleanFFI.INSTANCE.glean_is_initialized(handle)
        return initialized.toInt() != 0
    }

    /**
     * Initialize the core metrics internally managed by Glean (e.g. client id).
     */
    private fun initializeCoreMetrics(applicationContext: Context) {
        // Set a few more metrics that will be sent as part of every ping.
        // Please note that the following metrics must be set synchronously, so
        // that they are guaranteed to be available with the first ping that is
        // generated. We use an internal only API to do that.
        GleanBaseline.locale.setSync(getLocaleTag())
        GleanInternalMetrics.os.setSync("Android")
        // https://developer.android.com/reference/android/os/Build.VERSION
        GleanInternalMetrics.androidSdkVersion.setSync(Build.VERSION.SDK_INT.toString())
        GleanInternalMetrics.osVersion.setSync(Build.VERSION.RELEASE)
        // https://developer.android.com/reference/android/os/Build
        GleanInternalMetrics.deviceManufacturer.setSync(Build.MANUFACTURER)
        GleanInternalMetrics.deviceModel.setSync(Build.MODEL)
        GleanInternalMetrics.architecture.setSync(Build.SUPPORTED_ABIS[0])

        configuration.channel?.let {
            GleanInternalMetrics.appChannel.setSync(it)
        }

        try {
            val packageInfo = applicationContext.packageManager.getPackageInfo(
                    applicationContext.packageName, 0
            )
            @Suppress("DEPRECATION")
            GleanInternalMetrics.appBuild.setSync(packageInfo.versionCode.toString())

            GleanInternalMetrics.appDisplayVersion.setSync(
                    packageInfo.versionName?.let { it } ?: "Unknown"
            )
        } catch (e: PackageManager.NameNotFoundException) {
            Log.e(
                LOG_TAG,
                "Could not get own package info, unable to report build id and display version"
            )
            throw AssertionError("Could not get own package info, aborting init")
        }
    }

    /**
     * Enable or disable Glean collection and upload.
     *
     * Metric collection is enabled by default.
     *
     * When uploading is disabled, metrics aren't recorded at all and no data
     * is uploaded.
     *
     * When disabling, all pending metrics, events and queued pings are cleared.
     *
     * When enabling, the core Glean metrics are recreated.
     *
     * @param enabled When true, enable metric collection.
     */
    fun setUploadEnabled(enabled: Boolean) {
        // logger.info("Metrics enabled: $enabled")
        // val origUploadEnabled = uploadEnabled
        // uploadEnabled = enabled
        // if (isInitialized() && origUploadEnabled != enabled) {
        //     onChangeUploadEnabled(enabled)
        // }
        Log.e(LOG_TAG, "setUploadEnabled is a stub")
        // TODO: stub
    }

    /**
     * Get whether or not Glean is allowed to record and upload data.
     */
    fun getUploadEnabled(): Boolean {
        Log.e(LOG_TAG, "getUploadEnabled is a stub")
        // TODO: stub
        return false
    }

    /**
     * Get the data directory for glean.
     */
    internal fun getDataDir(): File {
        return this.gleanDataDir
    }

    fun collect(pingName: String) {
        val s = LibGleanFFI.INSTANCE.glean_ping_collect(handle, pingName)!!
        LibGleanFFI.INSTANCE.glean_str_free(s)
    }

    /**
     * Handle the background event and send the appropriate pings.
     */
    fun handleBackgroundEvent() {
        sendPing("baseline")
        sendPing("events")
    }

    internal fun sendPing(pingName: String) {
        val queued = LibGleanFFI.INSTANCE.glean_send_ping(
            handle,
            pingName,
            (configuration.logPings).toByte()
        )
        if (queued.toBoolean()) {
            PingUploadWorker.enqueueWorker()
        }
    }

    /**
     * Should be called from all users of the Glean testing API.
     *
     * This makes all asynchronous work synchronous so we can test the results of the
     * API synchronously.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun enableTestingMode() {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTestingMode(enabled = true)
    }

    /**
     * Test-only method to destroy the owned glean-core handle.
     */
    internal fun testDestroyGleanHandle() {
        if (!isInitialized()) {
            // We don't need to destroy the Glean handle: it wasn't initialized.
            return
        }

        val e = RustError.ByReference()
        LibGleanFFI.INSTANCE.glean_destroy_glean(handle, e)
        handle = 0L
    }
}

object Glean : GleanInternalAPI()
