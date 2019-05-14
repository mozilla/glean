/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.util.Log
import android.content.Context
import android.content.pm.PackageManager
import android.os.Build
import mozilla.telemetry.glean.utils.getLocaleTag
import java.io.File
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.MetricHandle
import mozilla.telemetry.glean.rust.RustError
import org.mozilla.gleancore.GleanMetrics.GleanBaseline
import org.mozilla.gleancore.GleanMetrics.GleanInternalMetrics

open class GleanInternalAPI internal constructor () {
    companion object {
        private val LOG_TAG: String = "glean-kotlin"
    }

    // `internal` so this can be modified for testing
    internal var handle: MetricHandle = 0L

    /**
     * Initialize glean.
     *
     * This should only be initialized once by the application, and not by
     * libraries using glean.
     */
    fun initialize(applicationContext: Context) {
        val dataDir = File(applicationContext.applicationInfo.dataDir, "glean_data")
        Log.i(LOG_TAG, "data dir: $dataDir")

        if (isInitialized()) {
            return
        }

        handle = LibGleanFFI.INSTANCE.glean_initialize(dataDir.path, applicationContext.packageName)

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
        // TODO: we should make sure to store the data below before any ping
        // is generated and sent. In a-c's Glean, we rely on the StorageEngine(s)
        // access to do so. Once we make the metric type API async, this won't work
        // anymore.
        GleanBaseline.locale.set(getLocaleTag())
        GleanInternalMetrics.os.set("Android")
        // https://developer.android.com/reference/android/os/Build.VERSION
        GleanInternalMetrics.androidSdkVersion.set(Build.VERSION.SDK_INT.toString())
        GleanInternalMetrics.osVersion.set(Build.VERSION.RELEASE)
        // https://developer.android.com/reference/android/os/Build
        GleanInternalMetrics.deviceManufacturer.set(Build.MANUFACTURER)
        GleanInternalMetrics.deviceModel.set(Build.MODEL)
        GleanInternalMetrics.architecture.set(Build.SUPPORTED_ABIS[0])

        /*
        configuration.channel?.let {
            StringsStorageEngine.record(GleanInternalMetrics.appChannel, it)
        }*/

        try {
            val packageInfo = applicationContext.packageManager.getPackageInfo(
                    applicationContext.packageName, 0
            )
            @Suppress("DEPRECATION")
            GleanInternalMetrics.appBuild.set(packageInfo.versionCode.toString())

            GleanInternalMetrics.appDisplayVersion.set(
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

    fun collect(pingName: String) {
        val e = RustError.ByReference()
        val s = LibGleanFFI.INSTANCE.glean_ping_collect(handle, pingName, e)!!
        LibGleanFFI.INSTANCE.glean_str_free(s)
    }

    /**
     * Handle the background event and send the appropriate pings.
     */
    fun handleBackgroundEvent() {
        sendPing("baseline")
        sendPing("events")
    }

    private fun sendPing(pingName: String) {
        LibGleanFFI.INSTANCE.glean_send_ping(handle, pingName)
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
