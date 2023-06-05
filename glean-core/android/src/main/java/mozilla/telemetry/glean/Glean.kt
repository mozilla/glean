/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.app.ActivityManager
import android.content.Context
import android.os.Build
import android.os.Process
import android.util.Log
import androidx.annotation.VisibleForTesting
import androidx.lifecycle.ProcessLifecycleOwner
import kotlinx.coroutines.Job
import kotlinx.coroutines.MainScope
import kotlinx.coroutines.launch
import mozilla.telemetry.glean.GleanMetrics.GleanValidation
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.internal.* // ktlint-disable no-wildcard-imports
import mozilla.telemetry.glean.net.BaseUploader
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.MetricsPingScheduler
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.utils.ThreadUtils
import mozilla.telemetry.glean.utils.calendarToDatetime
import mozilla.telemetry.glean.utils.canWriteToDatabasePath
import mozilla.telemetry.glean.utils.getLocaleTag
import org.json.JSONObject
import java.io.File
import java.util.Calendar

/**
 * Public exported type identifying individual timers for
 * [TimingDistributionMetricType][mozilla.telemetry.glean.private.TimingDistributionMetricType].
 */
typealias GleanTimerId = mozilla.telemetry.glean.internal.TimerId

data class BuildInfo(val versionCode: String, val versionName: String, val buildDate: Calendar)

internal class OnGleanEventsImpl(val glean: GleanInternalAPI) : OnGleanEvents {
    override fun initializeFinished() {
        // Only set up the lifecycle observers if we don't provide a custom
        // data path.
        if (!glean.isCustomDataPath) {
            MainScope().launch {
                ProcessLifecycleOwner.get().lifecycle.addObserver(glean.gleanLifecycleObserver)
            }
        }

        glean.initialized = true

        if (glean.testingMode) {
            glean.afterInitQueue.forEach { block ->
                block()
            }
        }
    }

    override fun triggerUpload() {
        PingUploadWorker.enqueueWorker(glean.applicationContext)
    }

    override fun startMetricsPingScheduler(): Boolean {
        // If we pass a custom data path, the metrics ping schedule should not
        // be setup.
        if (glean.isCustomDataPath) {
            glean.metricsPingScheduler?.cancel()
            return false
        }

        glean.metricsPingScheduler = MetricsPingScheduler(glean.applicationContext, glean.buildInfo)
        return glean.metricsPingScheduler!!.schedule()
    }

    override fun cancelUploads() {
        // Cancel any pending workers here so that we don't accidentally upload or
        // collect data after the upload has been disabled.
        glean.metricsPingScheduler?.cancel()
        // Cancel any pending workers here so that we don't accidentally upload
        // data after the upload has been disabled.
        PingUploadWorker.cancel(glean.applicationContext)
    }

    override fun shutdown() {
        // Android doesn't warn us about shutdown, so we don't try.
    }
}

/**
 * The main Glean API.
 *
 * This is exposed through the global [Glean] object.
 */
@Suppress("TooManyFunctions")
open class GleanInternalAPI internal constructor() {
    companion object {
        private const val LOG_TAG: String = "glean/Glean"
        private const val LANGUAGE_BINDING_NAME: String = "Kotlin"
        internal const val GLEAN_DATA_DIR: String = "glean_data"
    }

    internal var initialized: Boolean = false

    internal lateinit var configuration: Configuration

    // This is the wrapped http uploading mechanism: provides base functionalities
    // for logging and delegates the actual upload to the implementation in
    // the `Configuration`.
    internal lateinit var httpClient: BaseUploader

    internal lateinit var applicationContext: Context

    // Note: we set `applicationContext` early during startup so this should be fine.
    internal val gleanLifecycleObserver by lazy { GleanLifecycleObserver() }

    private lateinit var gleanDataDir: File

    // Are we in testing mode?
    internal var testingMode: Boolean = false
        private set // Keep the setter private to this class.

    // This object holds data related to any persistent information about the metrics ping,
    // such as the last time it was sent and the store name
    internal var metricsPingScheduler: MetricsPingScheduler? = null

    internal val afterInitQueue: MutableList<() -> Unit> = mutableListOf()

    // This is used to cache the process state and is used by the function `isMainProcess()`
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal var isMainProcess: Boolean? = null

    // When sending pings to a test endpoint, we're probably in instrumented tests. In that
    // case pings are to be immediately submitted by the WorkManager.
    internal var isSendingToTestEndpoint: Boolean = false

    // Store the build information provided by the application.
    internal lateinit var buildInfo: BuildInfo

    internal var isCustomDataPath: Boolean = false

    init {
        gleanEnableLogging()
    }

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
     * @param buildInfo A Glean [BuildInfo] object with build-time metadata. This
     *     object is generated at build time by glean_parser at the import path
     *     ${YOUR_PACKAGE_ROOT}.GleanMetrics.GleanBuildInfo.buildInfo
     */
    @Suppress("ReturnCount", "LongMethod", "ComplexMethod")
    @JvmOverloads
    @Synchronized
    fun initialize(
        applicationContext: Context,
        uploadEnabled: Boolean,
        configuration: Configuration = Configuration(),
        buildInfo: BuildInfo
    ) {
        configuration.dataPath?.let { safeDataPath ->
            // When the `dataPath` is provided, we need to make sure:
            //   1. The database path provided is not `glean_data`.
            //   2. The database path is valid and writable.

            // The default database path is `{context.applicationInfo.dataDir}/glean_data`,
            // the background process and the main process cannot write to the same file.
            if (safeDataPath == File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR).absolutePath) {
                Log.e(
                    LOG_TAG,
                    "Attempted to initialize Glean with an invalid database path " +
                        "\"{context.applicationInfo.dataDir}/glean_data\" is reserved"
                )
                return
            }

            // Check that the database path we are trying to write to is valid and writable.
            if (!canWriteToDatabasePath(safeDataPath)) {
                Log.e(LOG_TAG, "Attempted to initialize Glean with an invalid database path")
                return
            }

            this.gleanDataDir = File(safeDataPath)
            this.isCustomDataPath = true
        } ?: run {
            // If no `dataPath` is provided, then we setup Glean as usual.
            //
            // If we don't initialize on the main thread lifecycle registration may fail when
            // initializing on the main process.
            ThreadUtils.assertOnUiThread()

            // In certain situations Glean.initialize may be called from a process other than
            // the main process. In this case we want initialize to be a no-op and just return.
            //
            // You can call Glean.initialize from a background process, but to do so you need
            // to specify a dataPath in the Glean configuration.
            if (!isMainProcess(applicationContext)) {
                Log.e(
                    LOG_TAG,
                    "Attempted to initialize Glean on a process other than the main process without a dataPath"
                )
                return
            }

            this.gleanDataDir = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR)
            this.isCustomDataPath = false
        }

        if (isInitialized()) {
            Log.e(LOG_TAG, "Glean should not be initialized multiple times")
            return
        }

        this.buildInfo = buildInfo
        this.applicationContext = applicationContext

        this.configuration = configuration
        this.httpClient = BaseUploader(configuration.httpClient)

        // Execute startup off the main thread.
        Dispatchers.API.executeTask {
            val cfg = InternalConfiguration(
                dataPath = gleanDataDir.path,
                applicationId = applicationContext.packageName,
                languageBindingName = LANGUAGE_BINDING_NAME,
                uploadEnabled = uploadEnabled,
                maxEvents = null,
                delayPingLifetimeIo = false,
                appBuild = "none",
                useCoreMps = false,
                trimDataToRegisteredPings = false,
                logLevel = configuration.logLevel,
                rateLimit = null
            )
            val clientInfo = getClientInfo(configuration, buildInfo)
            val callbacks = OnGleanEventsImpl(this@GleanInternalAPI)
            gleanInitialize(cfg, clientInfo, callbacks)
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
        gleanSetUploadEnabled(enabled)
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
        var map = extra ?: mapOf()
        gleanSetExperimentActive(experimentId, branch, map)
    }

    /**
     * Indicate that an experiment is no longer running.
     *
     * @param experimentId The id of the experiment to deactivate.
     */
    fun setExperimentInactive(experimentId: String) {
        gleanSetExperimentInactive(experimentId)
    }

    /**
     * Tests whether an experiment is active, for testing purposes only.
     *
     * @param experimentId the id of the experiment to look for.
     * @return true if the experiment is active and reported in pings, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testIsExperimentActive(experimentId: String): Boolean {
        return gleanTestGetExperimentData(experimentId) != null
    }

    /**
     * Returns the stored data for the requested active experiment, for testing purposes only.
     *
     * @param experimentId the id of the experiment to look for.
     * @return the [RecordedExperiment] for the experiment
     * @throws [NullPointerException] if the requested experiment is not active or data is corrupt.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetExperimentData(experimentId: String): RecordedExperiment {
        return gleanTestGetExperimentData(experimentId) ?: throw NullPointerException("Experiment data is not set")
    }

    /**
     * Initialize the core metrics internally managed by Glean (e.g. client id).
     */
    internal fun getClientInfo(configuration: Configuration, buildInfo: BuildInfo): ClientInfoMetrics {
        return ClientInfoMetrics(
            appBuild = buildInfo.versionCode,
            appDisplayVersion = buildInfo.versionName,
            appBuildDate = calendarToDatetime(buildInfo.buildDate),

            architecture = Build.SUPPORTED_ABIS[0],
            osVersion = Build.VERSION.RELEASE,

            channel = configuration.channel,

            // https://developer.android.com/reference/android/os/Build.VERSION
            androidSdkVersion = Build.VERSION.SDK_INT.toString(),
            // https://developer.android.com/reference/android/os/Build
            deviceManufacturer = Build.MANUFACTURER,
            deviceModel = Build.MODEL,
            locale = getLocaleTag()
        )
    }

    /**
     * Get the data directory for Glean.
     */
    internal fun getDataDir(): File {
        return this.gleanDataDir
    }

    /**
     * Handle the foreground event and send the appropriate pings.
     */
    internal fun handleForegroundEvent() {
        // Note that this is sending the length of the last foreground session
        // because it belongs to the baseline ping and that ping is sent every
        // time the app goes to background.
        gleanHandleClientActive()

        GleanValidation.foregroundCount.add(1)
    }

    /**
     * Handle the background event and send the appropriate pings.
     */
    internal fun handleBackgroundEvent() {
        gleanHandleClientInactive()
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
    internal fun submitPingByName(pingName: String, reason: String? = null) {
        gleanSubmitPingByName(pingName, reason)
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
    fun setDebugViewTag(value: String): Boolean {
        return gleanSetDebugViewTag(value)
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
    fun setSourceTags(tags: Set<String>): Boolean {
        return gleanSetSourceTags(tags.toList())
    }

    /**
     * Set configuration for metrics' disabled property, typically from a remote_settings
     * experiment or rollout.
     *
     * @param json Stringified JSON map of metrics and their associated `disabled` property.
     */
    fun setMetricsDisabledConfig(json: String) {
        // Let's convert this to the new API for backwards compatibility
        // In order to convert to the new API we need to flip all of the boolean values that
        // are contained in the map contained in the supplied JSON string. We do this by
        // parsing the string and then iterating through the keys to create a new object with
        // the boolean values inverted. Finally, we turn this back into a string to pass into
        // the `setMetricsEnabledConfig` function.
        val jsonData = JSONObject(json)
        var invertedJsonData = JSONObject()
        jsonData.keys().forEach {
            invertedJsonData.put(it, !jsonData.getBoolean(it))
        }
        setMetricsEnabledConfig(invertedJsonData.toString())
    }

    /**
     * Set configuration to override metrics' enabled/disabled state, typically from a remote_settings
     * experiment or rollout.
     *
     * @param json Stringified JSON map of metrics and their associated `disabled` property.
     */
    fun setMetricsEnabledConfig(json: String) {
        gleanSetMetricsEnabledConfig(json)
    }

    /**
     * Set the logPing debug option, when this is `true`
     * the payload of assembled ping requests get logged.
     *
     * This is only meant to be used internally by the `GleanDebugActivity`.
     *
     * @param value The value of the option.
     */
    fun setLogPings(value: Boolean) {
        gleanSetLogPings(value)
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
        this.setTestingMode(true)
    }

    /**
     * TEST ONLY FUNCTION.
     * This can be called by tests to change test mode on-the-fly.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun setTestingMode(enabled: Boolean) {
        this.testingMode = enabled
        gleanSetTestMode(enabled)
        Dispatchers.API.setTestingMode(enabled)
    }

    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun setDirtyFlag(flag: Boolean) {
        gleanSetDirtyFlag(flag)
    }

    /**
     * TEST ONLY FUNCTION.
     * Resets the Glean state and trigger init again.
     *
     * @param context the application context to init Glean with
     * @param config the [Configuration] to init Glean with
     * @param clearStores if true, clear the contents of all stores
     * @param uploadEnabled whether upload is enabled
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun resetGlean(
        context: Context,
        config: Configuration,
        clearStores: Boolean,
        uploadEnabled: Boolean = true
    ) {
        isMainProcess = null

        // Resetting MPS and uploader
        metricsPingScheduler?.cancel()
        PingUploadWorker.cancel(context)

        // Init Glean.
        val gleanDataDir = config.dataPath?.let { safeDataPath ->
            File(safeDataPath)
        } ?: run {
            File(context.applicationInfo.dataDir, GLEAN_DATA_DIR)
        }

        Glean.testDestroyGleanHandle(clearStores, gleanDataDir.path)
        // Enable test mode.
        Glean.enableTestingMode()
        // Always log pings for tests
        Glean.setLogPings(true)

        val buildInfo = BuildInfo(versionCode = "0.0.1", versionName = "0.0.1", buildDate = Calendar.getInstance())
        Glean.initialize(context, uploadEnabled, config, buildInfo)
    }

    /**
     * Run a task right after initialization.
     *
     * If initialization already happened the task runs immediately.
     * Otherwise it is queued and run after initialization finishes.
     */
    internal fun afterInitialize(block: () -> Unit) {
        // Queueing tasks after initialize is only allowed in test mode.
        assert(this.testingMode)

        if (isInitialized()) {
            block()
        } else {
            this.afterInitQueue.add(block)
        }
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

        Glean.afterInitialize {
            val endpointUrl = "http://localhost:$port"
            Glean.configuration = configuration.copy(serverEndpoint = endpointUrl)
        }
    }

    /**
     * Test-only method to destroy the owned glean-core handle.
     *
     * @param clearStores Whether to clear data after destroying the Glean object
     * @param dataPath The path to the data folder. Must be set if `clearStores` is `true`.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    internal fun testDestroyGleanHandle(clearStores: Boolean = false, dataPath: String? = null) {
        // If it was initialized this also clears the directory
        gleanTestDestroyGlean(clearStores, dataPath)

        if (!isInitialized()) {
            // We don't need to destroy anything else: it wasn't initialized.
            return
        }

        // Reset all state.
        gleanSetTestMode(false)
        testingMode = false
        initialized = false
    }

    /**
     *  Returns true if we are running in the main process false otherwise.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun isMainProcess(context: Context): Boolean {
        if (isMainProcess != null) return isMainProcess as Boolean

        val pid = Process.myPid()
        val activityManager = context.getSystemService(Context.ACTIVITY_SERVICE) as ActivityManager

        isMainProcess = (
            activityManager.runningAppProcesses?.any { processInfo ->
                (processInfo.pid == pid && processInfo.processName == context.packageName)
            }
            ) ?: false

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
