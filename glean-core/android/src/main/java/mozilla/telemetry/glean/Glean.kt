/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.app.ActivityManager
import android.util.Log
import android.content.Context
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import android.os.Build
import android.os.Process
import androidx.annotation.MainThread
import androidx.annotation.VisibleForTesting
import androidx.lifecycle.ProcessLifecycleOwner
import com.sun.jna.StringArray
import kotlinx.coroutines.Job
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.launch
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.config.FfiConfiguration
import mozilla.telemetry.glean.utils.getLocaleTag
import java.io.File
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.getAndConsumeRustString
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.net.BaseUploader
import mozilla.telemetry.glean.private.PingTypeBase
import mozilla.telemetry.glean.private.RecordedExperimentData
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.scheduler.MetricsPingScheduler
import mozilla.telemetry.glean.utils.ThreadUtils
import org.json.JSONObject

/**
 * Public exported type identifying individual timers for
 * [TimingDistributionMetricType][mozilla.telemetry.glean.private.TimingDistributionMetricType].
 */
data class GleanTimerId internal constructor(internal val id: Long)

/**
 * The main Glean API.
 *
 * This is exposed through the global [Glean] object.
 */
@Suppress("TooManyFunctions")
open class GleanInternalAPI internal constructor () {
    companion object {
        private const val LOG_TAG: String = "glean/Glean"
        private const val LANGUAGE_BINDING_NAME: String = "Kotlin"
        internal const val GLEAN_DATA_DIR: String = "glean_data"
    }

    private var initialized: Boolean = false

    internal lateinit var configuration: Configuration

    // This is the wrapped http uploading mechanism: provides base functionalities
    // for logging and delegates the actual upload to the implementation in
    // the `Configuration`.
    internal lateinit var httpClient: BaseUploader

    private lateinit var applicationContext: Context

    // Note: we set `applicationContext` early during startup so this should be fine.
    private val gleanLifecycleObserver by lazy { GleanLifecycleObserver() }

    private lateinit var gleanDataDir: File

    // Keep track of this value before Glean is initialized
    private var debugViewTag: String? = null

    // Keep track of this value before Glean is initialized
    private var logPings: Boolean = false

    // Keep track of source tags if set before Glean is initialized.
    private var sourceTags: Set<String>? = null

    // This object holds data related to any persistent information about the metrics ping,
    // such as the last time it was sent and the store name
    internal lateinit var metricsPingScheduler: MetricsPingScheduler

    // Keep track of ping types that have been registered before Glean is initialized.
    internal val pingTypeQueue: MutableSet<PingTypeBase> = mutableSetOf()

    // This is used to cache the process state and is used by the function `isMainProcess()`
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal var isMainProcess: Boolean? = null

    // When sending pings to a test endpoint, we're probably in instrumented tests. In that
    // case pings are to be immediately submitted by the WorkManager.
    internal var isSendingToTestEndpoint: Boolean = false

    /**
     * Initialize the Glean SDK.
     *
     * This should only be initialized once by the application, and not by
     * libraries using the Glean SDK. A message is logged to error and no
     * changes are made to the state if initialize is called a more than
     * once.
     *
     * A LifecycleObserver will be added to send pings when the application goes
     * into foreground and background.
     *
     * This method must be called from the main thread.
     *
     * @param applicationContext [Context] to access application features, such
     * as shared preferences
     * @param uploadEnabled A [Boolean] that determines whether telemetry is enabled.
     *     If disabled, all persisted metrics, events and queued pings (except
     *     first_run_date) are cleared.
     * @param configuration A Glean [Configuration] object with global settings.
     */
    @Suppress("ReturnCount", "LongMethod", "ComplexMethod")
    @JvmOverloads
    @Synchronized
    @MainThread
    fun initialize(
        applicationContext: Context,
        uploadEnabled: Boolean,
        configuration: Configuration = Configuration()
    ) {
        // Glean initialization must be called on the main thread, or lifecycle
        // registration may fail. This is also enforced at build time by the
        // @MainThread decorator, but this run time check is also performed to
        // be extra certain.
        ThreadUtils.assertOnUiThread()

        // In certain situations Glean.initialize may be called from a process other than the main
        // process.  In this case we want initialize to be a no-op and just return.
        if (!isMainProcess(applicationContext)) {
            Log.e(LOG_TAG, "Attempted to initialize Glean on a process other than the main process")
            return
        }

        if (isInitialized()) {
            Log.e(LOG_TAG, "Glean should not be initialized multiple times")
            return
        }

        this.applicationContext = applicationContext

        this.configuration = configuration
        this.httpClient = BaseUploader(configuration.httpClient)
        this.gleanDataDir = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR)

        // Execute startup off the main thread.
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.executeTask {
            registerPings(Pings)

            val cfg = FfiConfiguration(
                dataDir = gleanDataDir.path,
                packageName = applicationContext.packageName,
                languageBindingName = LANGUAGE_BINDING_NAME,
                uploadEnabled = uploadEnabled,
                maxEvents = configuration.maxEvents,
                delayPingLifetimeIO = false
            )

            initialized = LibGleanFFI.INSTANCE.glean_initialize(cfg).toBoolean()

            // If initialization of Glean fails we bail out and don't initialize further.
            if (!initialized) {
                return@executeTask
            }

            // The debug view tag might have been set before initialize,
            // get the cached value and set it.
            if (debugViewTag != null) {
                setDebugViewTag(debugViewTag!!)
            }

            // The log pings debug option might have been set before initialize,
            // get the cached value and set it.
            if (logPings) {
                setLogPings(logPings)
            }

            // The source tags might have been set before initialize,
            // get the cached value and set them.
            sourceTags?.let { setSourceTags(it) }

            // Get the current value of the dirty flag so we know whether to
            // send a dirty startup baseline ping below.  Immediately set it to
            // `false` so that dirty startup pings won't be sent if Glean
            // initialization does not complete successfully. It is set to `true`
            // again in the ON_START lifecycle event.
            val isDirtyFlagSet = LibGleanFFI.INSTANCE.glean_is_dirty_flag_set().toBoolean()
            LibGleanFFI.INSTANCE.glean_set_dirty_flag(false.toByte())

            // If any pings were registered before initializing, do so now.
            // We're not clearing this queue in case Glean is reset by tests.
            synchronized(this@GleanInternalAPI) {
                pingTypeQueue.forEach { registerPingType(it) }
            }

            // If this is the first time ever the Glean SDK runs, make sure to set
            // some initial core metrics in case we need to generate early pings.
            // The next times we start, we would have them around already.
            val isFirstRun = LibGleanFFI.INSTANCE.glean_is_first_run().toBoolean()
            if (isFirstRun) {
                initializeCoreMetrics(applicationContext)
            }

            // Deal with any pending events so we can start recording new ones
            val pingSubmitted = LibGleanFFI.INSTANCE.glean_on_ready_to_submit_pings().toBoolean()

            // We need to enqueue the PingUploadWorker in these cases:
            // 1. Pings were submitted through Glean and it is ready to upload those pings;
            // 2. Upload is disabled, to upload a possible deletion-request ping.
            if (pingSubmitted || !uploadEnabled) {
                PingUploadWorker.enqueueWorker(applicationContext)
            }

            // Set up information and scheduling for Glean owned pings. Ideally, the "metrics"
            // ping startup check should be performed before any other ping, since it relies
            // on being dispatched to the API context before any other metric.
            metricsPingScheduler = MetricsPingScheduler(applicationContext)
            metricsPingScheduler.schedule()

            // Check if the "dirty flag" is set. That means the product was probably
            // force-closed. If that's the case, submit a 'baseline' ping with the
            // reason "dirty_startup". We only do that from the second run.
            if (!isFirstRun && isDirtyFlagSet) {
                submitPingByNameSync("baseline", "dirty_startup")
            }

            // From the second time we run, after all startup pings are generated,
            // make sure to clear `lifetime: application` metrics and set them again.
            // Any new value will be sent in newly generated pings after startup.
            if (!isFirstRun) {
                LibGleanFFI.INSTANCE.glean_clear_application_lifetime_metrics()
                initializeCoreMetrics(applicationContext)
            }

            // Signal Dispatcher that init is complete
            Dispatchers.API.flushQueuedInitialTasks()

            // At this point, all metrics and events can be recorded.
            // This should only be called from the main thread. This is enforced by
            // the @MainThread decorator and the `assertOnUiThread` call.
            MainScope().launch {
                ProcessLifecycleOwner.get().lifecycle.addObserver(gleanLifecycleObserver)
            }
        }
    }

    /**
     * Returns true if the Glean SDK has been initialized.
     */
    internal fun isInitialized(): Boolean {
        return initialized
    }

    /**
     * Register the pings generated from `pings.yaml` with the Glean SDK.
     *
     * @param pings The `Pings` object generated for your library or application
     * by the Glean SDK.
     */
    fun registerPings(pings: Any) {
        // Instantiating the Pings object to send this function is enough to
        // call the constructor and have it registered through [Glean.registerPingType].
        Log.i(LOG_TAG, "Registering pings for ${pings.javaClass.canonicalName}")
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
        // Changing upload enabled always happens asynchronous.
        // That way it follows what a user expect when calling it inbetween other calls:
        // It executes in the right order.
        //
        // Because the dispatch queue is halted until Glean is fully initialized
        // we can safely enqueue here and it will execute after initialization.
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            val originalEnabled = getUploadEnabled()
            LibGleanFFI.INSTANCE.glean_set_upload_enabled(enabled.toByte())

            if (!enabled) {
                // Cancel any pending workers here so that we don't accidentally upload or
                // collect data after the upload has been disabled.
                metricsPingScheduler.cancel()
                // Cancel any pending workers here so that we don't accidentally upload
                // data after the upload has been disabled.
                PingUploadWorker.cancel(applicationContext)
            }

            if (!originalEnabled && enabled) {
                // If uploading is being re-enabled, we have to restore the
                // application-lifetime metrics.
                initializeCoreMetrics((this@GleanInternalAPI).applicationContext)
            }

            if (originalEnabled && !enabled) {
                // If uploading is disabled, we need to send the deletion-request ping
                PingUploadWorker.enqueueWorker(applicationContext)
            }
        }
    }

    /**
     * Get whether or not Glean is allowed to record and upload data.
     *
     * Caution: the result is only correct if Glean is already initialized.
     */
    fun getUploadEnabled(): Boolean {
        if (isInitialized()) {
            return LibGleanFFI.INSTANCE.glean_is_upload_enabled().toBoolean()
        } else {
            return true
        }
    }

    /**
     * Indicate that an experiment is running. Glean will then add an
     * experiment annotation to the environment which is sent with pings. This
     * information is not persisted between runs.
     *
     * @param experimentId The id of the active experiment (maximum 100 bytes)
     * @param branch The experiment branch (maximum 100 bytes)
     * @param extra Optional metadata to output with the ping
     */
    @JvmOverloads
    fun setExperimentActive(
        experimentId: String,
        branch: String,
        extra: Map<String, String>? = null
    ) {
        // The Map is sent over FFI as a pair of arrays, one containing the
        // keys, and the other containing the values.
        // In Kotlin, Map.keys and Map.values are not guaranteed to return the entries
        // in any particular order. Therefore, we iterate over the pairs together and
        // create the keys and values arrays step-by-step.
        var keys: StringArray? = null
        var values: StringArray? = null
        var numKeys = 0

        extra?.let {
            numKeys = extra.size
            val extraList = extra.toList()
            keys = StringArray(Array(extra.size) { extraList[it].first }, "utf-8")
            values = StringArray(Array(extra.size) { extraList[it].second }, "utf-8")
        }

        // We dispatch this asynchronously so that, if called before the Glean SDK is
        // initialized, it doesn't get ignored and will be replayed after init.
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_set_experiment_active(
                experimentId,
                branch,
                keys,
                values,
                numKeys
            )
        }
    }

    /**
     * Indicate that an experiment is no longer running.
     *
     * @param experimentId The id of the experiment to deactivate.
     */
    fun setExperimentInactive(experimentId: String) {
        // We dispatch this asynchronously so that, if called before the Glean SDK is
        // initialized, it doesn't get ignored and will be replayed after init.
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            LibGleanFFI.INSTANCE.glean_set_experiment_inactive(experimentId)
        }
    }

    /**
     * Tests whether an experiment is active, for testing purposes only.
     *
     * @param experimentId the id of the experiment to look for.
     * @return true if the experiment is active and reported in pings, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testIsExperimentActive(experimentId: String): Boolean {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return LibGleanFFI.INSTANCE.glean_experiment_test_is_active(experimentId).toBoolean()
    }

    /**
     * Utility function to get a String -> String [Map] out of a [JSONObject].
     */
    private fun getMapFromJSONObject(jsonRes: JSONObject): Map<String, String>? {
        return jsonRes.optJSONObject("extra")?.let {
            val map = mutableMapOf<String, String>()
            it.names()?.let { names ->
                for (i in 0 until names.length()) {
                    map[names.getString(i)] = it.getString(names.getString(i))
                }
            }
            map
        }
    }

    /**
    * Returns the stored data for the requested active experiment, for testing purposes only.
    *
    * @param experimentId the id of the experiment to look for.
    * @return the [RecordedExperimentData] for the experiment
    * @throws [NullPointerException] if the requested experiment is not active or data is corrupt.
    */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetExperimentData(experimentId: String): RecordedExperimentData {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        val ptr = LibGleanFFI.INSTANCE.glean_experiment_test_get_data(
            experimentId
        )!!

        var branchId: String
        var extraMap: Map<String, String>?

        try {
            // Parse and extract the fields from the JSON string here so
            // that we can always throw NullPointerException if something
            // goes wrong.
            val jsonRes = JSONObject(ptr.getAndConsumeRustString())
            branchId = jsonRes.getString("branch")
            extraMap = getMapFromJSONObject(jsonRes)
        } catch (e: org.json.JSONException) {
            throw NullPointerException()
        }

        return RecordedExperimentData(branchId, extraMap)
    }

    /**
     * Initialize the core metrics internally managed by Glean (e.g. client id).
     */
    private fun initializeCoreMetrics(applicationContext: Context) {
        // Set a few more metrics that will be sent as part of every ping.
        // Please note that the following metrics must be set synchronously, so
        // that they are guaranteed to be available with the first ping that is
        // generated. We use an internal only API to do that.
        // https://developer.android.com/reference/android/os/Build.VERSION
        GleanInternalMetrics.androidSdkVersion.setSync(Build.VERSION.SDK_INT.toString())
        GleanInternalMetrics.osVersion.setSync(Build.VERSION.RELEASE)
        // https://developer.android.com/reference/android/os/Build
        GleanInternalMetrics.deviceManufacturer.setSync(Build.MANUFACTURER)
        GleanInternalMetrics.deviceModel.setSync(Build.MODEL)
        GleanInternalMetrics.architecture.setSync(Build.SUPPORTED_ABIS[0])
        GleanInternalMetrics.locale.setSync(getLocaleTag())

        configuration.channel?.let {
            GleanInternalMetrics.appChannel.setSync(it)
        }

        val packageInfo: PackageInfo

        try {
            packageInfo = applicationContext.packageManager.getPackageInfo(
                    applicationContext.packageName, 0
            )
        } catch (e: PackageManager.NameNotFoundException) {
            Log.e(
                LOG_TAG,
                "Could not get own package info, unable to report build id and display version"
            )

            GleanInternalMetrics.appBuild.setSync("inaccessible")
            GleanInternalMetrics.appDisplayVersion.setSync("inaccessible")

            return
        }

        @Suppress("DEPRECATION")
        GleanInternalMetrics.appBuild.setSync(packageInfo.versionCode.toString())

        GleanInternalMetrics.appDisplayVersion.setSync(
            packageInfo.versionName?.let { it } ?: "Unknown"
        )
    }

    /**
     * Get the data directory for Glean.
     */
    internal fun getDataDir(): File {
        return this.gleanDataDir
    }

    /**
     * Collect a ping and return a string
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun testCollect(ping: PingTypeBase, reason: String? = null): String? {
        return LibGleanFFI.INSTANCE.glean_ping_collect(ping.handle, reason)?.getAndConsumeRustString()
    }

    /**
     * Handle the foreground event and send the appropriate pings.
     */
    internal fun handleForegroundEvent() {
        Pings.baseline.submit(Pings.baselineReasonCodes.foreground)
    }

    /**
     * Handle the background event and send the appropriate pings.
     */
    internal fun handleBackgroundEvent() {
        Pings.baseline.submit(Pings.baselineReasonCodes.background)
        Pings.events.submit(Pings.eventsReasonCodes.background)
    }

    /**
     * Collect and submit a ping for eventual upload.
     *
     * The ping content is assembled as soon as possible, but upload is not
     * guaranteed to happen immediately, as that depends on the upload
     * policies.
     *
     * If the ping currently contains no content, it will not be assembled and
     * queued for sending.
     *
     * @param ping Ping to submit.
     * @param reason The reason the ping is being submitted.
     * @return The async [Job] performing the work of assembling the ping
     */
    internal fun submitPing(ping: PingTypeBase, reason: String? = null): Job? {
        return submitPingByName(ping.name, reason)
    }

    /**
     * Collect and submit a ping for eventual upload by name.
     *
     * The ping will be looked up in the known instances of [PingType]. If the
     * ping isn't known, an error is logged and the ping isn't queued for uploading.
     *
     * The ping content is assembled as soon as possible, but upload is not
     * guaranteed to happen immediately, as that depends on the upload
     * policies.
     *
     * If the ping currently contains no content, it will not be assembled and
     * queued for sending, unless explicitly specified otherwise in the registry
     * file.
     *
     * @param pingName Name of the ping to submit.
     * @param reason The reason the ping is being submitted.
     * @return The async [Job] performing the work of assembling the ping
     */
    @Suppress("EXPERIMENTAL_API_USAGE")
    internal fun submitPingByName(pingName: String, reason: String? = null) = Dispatchers.API.launch {
        submitPingByNameSync(pingName, reason)
    }

    /**
     * Collect and submit a ping (by its name) for eventual upload, synchronously.
     *
     * The ping will be looked up in the known instances of [PingType]. If the
     * ping isn't known, an error is logged and the ping isn't queued for uploading.
     *
     * The ping content is assembled as soon as possible, but upload is not
     * guaranteed to happen immediately, as that depends on the upload
     * policies.
     *
     * If the ping currently contains no content, it will not be assembled and
     * queued for sending, unless explicitly specified otherwise in the registry
     * file.
     *
     * @param pingName Name of the ping to submit.
     * @param reason The reason the ping is being submitted.
     */
    internal fun submitPingByNameSync(pingName: String, reason: String? = null) {
        if (!isInitialized()) {
            Log.e(LOG_TAG, "Glean must be initialized before submitting pings.")
            return
        }

        if (!getUploadEnabled()) {
            Log.e(LOG_TAG, "Glean disabled: not submitting any pings.")
            return
        }

        val submittedPing = LibGleanFFI.INSTANCE.glean_submit_ping_by_name(
            pingName,
            reason
        ).toBoolean()

        if (submittedPing) {
            PingUploadWorker.enqueueWorker(applicationContext)
        }
    }

    /**
     * Set a tag to be applied to headers when uploading pings for debug view.
     *
     * If the tag is invalid it won't be set and this function will return `false`,
     * although if we are not initialized yet, there won't be any validation.
     *
     * This is only meant to be used internally by the `GleanDebugActivity`.
     *
     * @param value The value of the tag, which must be a valid HTTP header value.
     */
    internal fun setDebugViewTag(value: String): Boolean {
        if (isInitialized()) {
            return LibGleanFFI.INSTANCE.glean_set_debug_view_tag(value).toBoolean()
        } else {
            debugViewTag = value
            // When setting the debug view tag before initialization,
            // we don't validate the tag, thus this function always returns true.
            return true
        }
    }

    /**
     * Set the source tags to be applied as headers when uploading pings.
     *
     * If any of the tags is invalid nothing will be set and this function will
     * return `false`, although if we are not initialized yet, there won't be any validation.
     *
     * This is only meant to be used internally by the `GleanDebugActivity`.
     *
     * @param tags A list of tags, which must be valid HTTP header values.
     */
    internal fun setSourceTags(tags: Set<String>): Boolean {
        return if (isInitialized()) {
            val tagList = StringArray(tags.toList().toTypedArray(), "utf-8")
            LibGleanFFI.INSTANCE.glean_set_source_tags(tagList, tags.size).toBoolean()
        } else {
            sourceTags = tags
            // When setting the source tags before initialization,
            // we don't validate the tags, thus this function always returns true.
            true
        }
    }

    /**
     * Set the logPing debug option, when this is `true`
     * the payload of assembled ping requests get logged.
     *
     * This is only meant to be used internally by the `GleanDebugActivity`.
     *
     * @param value The value of the option.
     */
    internal fun setLogPings(value: Boolean) {
        if (isInitialized()) {
            return LibGleanFFI.INSTANCE.glean_set_log_pings(value.toByte())
        } else {
            logPings = value
        }
    }

    /**
     * TEST ONLY FUNCTION.
     * This is called by the GleanTestRule, to enable test mode.
     *
     * This makes all asynchronous work synchronous so we can test the results of the
     * API synchronously.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun enableTestingMode() {
        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.setTestingMode(enabled = true)
    }

    /**
     * TEST ONLY FUNCTION.
     * Resets the Glean state and trigger init again.
     *
     * @param context the application context to init Glean with
     * @param config the [Configuration] to init Glean with
     * @param clearStores if true, clear the contents of all stores
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun resetGlean(
        context: Context,
        config: Configuration,
        clearStores: Boolean,
        uploadEnabled: Boolean = true
    ) {
        Glean.enableTestingMode()

        if (isInitialized() && clearStores) {
            // Clear all the stored data.
            LibGleanFFI.INSTANCE.glean_test_clear_all_stores()
        }

        isMainProcess = null

        // Init Glean.
        Glean.testDestroyGleanHandle()
        // Always log pings for tests
        Glean.setLogPings(true)
        Glean.initialize(context, uploadEnabled, config)
    }

    /**
     * TEST ONLY FUNCTION.
     * Sets the server endpoint to a local address for ingesting test pings.
     *
     * The endpoint will be set as "http://localhost:<port>" and pings will be
     * immediately sent by the WorkManager.
     *
     * @param port the local address to send pings to
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun testSetLocalEndpoint(port: Int) {
        Glean.enableTestingMode()

        isSendingToTestEndpoint = true

        // We can't set the configuration unless we're initialized.
        assert(isInitialized())

        val endpointUrl = "http://localhost:$port"

        Glean.configuration = configuration.copy(serverEndpoint = endpointUrl)
    }

    /**
     * Test-only method to destroy the owned glean-core handle.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun testDestroyGleanHandle() {
        if (!isInitialized()) {
            // We don't need to destroy Glean: it wasn't initialized.
            return
        }

        LibGleanFFI.INSTANCE.glean_destroy_glean()
        initialized = false
    }

    /**
     * Register a [PingType] in the registry associated with this [Glean] object.
     */
    @Synchronized
    internal fun registerPingType(pingType: PingTypeBase) {
        if (this.isInitialized()) {
            LibGleanFFI.INSTANCE.glean_register_ping_type(
                pingType.handle
            )
        }

        // We need to keep track of pings, so they get re-registered after a reset.
        // This state is kept across Glean resets, which should only ever happen in test mode.
        // Or by the instrumentation tests (`connectedAndroidTest`), which relaunches the application activity,
        // but not the whole process, meaning globals, such as the ping types, still exist from the old run.
        // It's a set and keeping them around forever should not have much of an impact.

        pingTypeQueue.add(pingType)
    }

    /**
     * Returns true if a ping by this name is in the ping registry.
     *
     * For internal testing only.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun testHasPingType(pingName: String): Boolean {
        return LibGleanFFI.INSTANCE.glean_test_has_ping_type(pingName).toBoolean()
    }

    /**
     *  Returns true if we are running in the main process false otherwise.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun isMainProcess(context: Context): Boolean {
        if (isMainProcess != null) return isMainProcess as Boolean

        val pid = Process.myPid()
        val activityManager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager

        isMainProcess = (activityManager.runningAppProcesses?.any { processInfo ->
            (processInfo.pid == pid && processInfo.processName == context.packageName)
        }) ?: false

        return isMainProcess as Boolean
    }
}

/**
 * The main Glean object.
 *
 * ```
 * Glean.setUploadEnabled(true)
 * Glean.initialize(applicationContext)
 * ```
 */
object Glean : GleanInternalAPI()
