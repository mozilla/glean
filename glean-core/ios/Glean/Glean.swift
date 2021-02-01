/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

private typealias GleanBaseline = GleanMetrics.GleanBaseline
private typealias GleanInternalMetrics = GleanMetrics.GleanInternalMetrics
private typealias GleanValidation = GleanMetrics.GleanValidation
private typealias Pings = GleanMetrics.Pings

/// Public exported type identifying individual timers for `TimingDistributionMetricType`
public typealias GleanTimerId = UInt64

/// The main Glean API.
///
/// This is exposed through the global `Glean.shared` object.
// swiftlint:disable type_body_length
public class Glean {
    /// The main Glean object.
    ///
    /// ```swift
    /// Glean.shared.initialize(uploadEnabled: true)
    /// ```
    public static let shared = Glean()

    var metricsPingScheduler: MetricsPingScheduler = MetricsPingScheduler()

    var initialized: Bool = false
    // Set when `initialize()` returns.
    // This allows to detect calls that happen before `Glean.shared.initialize()` was called.
    // Note: The initialization might still be in progress, as it runs in a separate thread.
    var initFinished = AtomicBoolean(false)

    private var debugViewTag: String?
    private var sourceTags: [String]?
    var logPings: Bool = false
    var configuration: Configuration?
    private var observer: GleanLifecycleObserver?

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/Glean"
        static let languageBindingName = "Swift"
    }

    private var pingTypeQueue = [PingBase]()

    private let logger = Logger(tag: Constants.logTag)

    // Cache variable for checking if running in main process.  Also used to override for tests in
    // order to simulate not running in the main process.  DO NOT SET EXCEPT IN TESTS!
    var isMainProcess: Bool?

    private init() {
        // intentionally left private, no external user can instantiate a new global object.

        // Enable logging in the Rust library
        glean_enable_logging()
    }

    deinit {
        self.initialized = false
    }

    // swiftlint:disable function_body_length cyclomatic_complexity
    /// Initialize the Glean SDK.
    ///
    /// This should only be initialized once by the application, and not by
    /// libraries using the Glean SDK. A message is logged to error and no
    /// changes are made to the state if initialize is called a more than
    /// once.
    ///
    /// A LifecycleObserver will be added to submit pings when the application goes
    /// into the foreground and background.
    ///
    /// - parameters:
    ///     * uploadEnabled: A `Bool` that enables or disables telemetry.
    ///       If disabled, all persisted metrics, events and queued pings (except
    ///       first_run_date and first_run_hour) are cleared.
    ///     * configuration: A Glean `Configuration` object with global settings.
    public func initialize(uploadEnabled: Bool,
                           configuration: Configuration = Configuration()) {
        // In certain situations Glean.initialize may be called from a process other than the main
        // process such as an embedded extension. In this case we want to just return.
        // See https://bugzilla.mozilla.org/show_bug.cgi?id=1625157 for more information.
        if !checkIsMainProcess() {
            logger.error("Attempted to initialize Glean on a process other than the main process")
            return
        }

        if self.isInitialized() {
            logger.error("Glean should not be initialized multiple times")
            return
        }

        self.configuration = configuration

        // Execute startup off the main thread
        Dispatchers.shared.launchConcurrent {
            self.initialized = withFfiConfiguration(
                // The FileManager returns `file://` URLS with absolute paths.
                // The Rust side expects normal path strings to be used.
                // `relativePath` for a file URL gives us the absolute filesystem path.
                dataDir: getGleanDirectory().relativePath,
                packageName: AppInfo.name,
                languageBindingName: Constants.languageBindingName,
                uploadEnabled: uploadEnabled,
                configuration: configuration
            ) { cfg in
                var cfg = cfg
                return glean_initialize(&cfg).toBool()
            }

            // If initialization of Glean fails, bail out and don't initialize further
            if !self.initialized {
                return
            }

            if let debugViewTag = self.debugViewTag {
                _ = self.setDebugViewTag(debugViewTag)
            }

            if self.logPings {
                self.setLogPings(self.logPings)
            }

            if let sourceTags = self.sourceTags {
                _ = self.setSourceTags(sourceTags)
            }

            // Get the current value of the dirty flag so we know whether to
            // send a dirty startup baseline ping below.  Immediately set it to
            // `false` so that dirty startup pings won't be sent if Glean
            // initialization does not complete successfully.
            // It is set to `true` again when registering the lifecycle observer.
            let isDirtyFlagSet = glean_is_dirty_flag_set().toBool()
            glean_set_dirty_flag(false.toByte())

            // Register builtin pings.
            // Unfortunately we need to manually list them here to guarantee they are registered synchronously
            // before we need them.
            // We don't need to handle the deletion-request ping. It's never touched from the language implementation.
            glean_register_ping_type(Pings.shared.baseline.handle)
            glean_register_ping_type(Pings.shared.metrics.handle)
            glean_register_ping_type(Pings.shared.events.handle)

            // If any pings were registered before initializing, do so now
            for ping in self.pingTypeQueue {
                // We're registering pings synchronously here,
                // as this whole `initialize` block already runs off the main thread.
                glean_register_ping_type(ping.handle)
            }

            // If this is the first time ever the Glean SDK runs, make sure to set
            // some initial core metrics in case we need to generate early pings.
            // The next times we start, we would have them around already.
            let isFirstRun = glean_is_first_run().toBool()
            if isFirstRun {
                self.initializeCoreMetrics()
            }

            // Deal with any pending events so we can start recording new ones
            let pingSubmitted = glean_on_ready_to_submit_pings().toBool()

            // We need to enqueue the ping uploader in these cases:
            // 1. Pings were submitted through Glean and it is ready to upload those pings;
            // 2. Upload is disabled, to upload a possible deletion-request ping.
            if pingSubmitted || !uploadEnabled {
                HttpPingUploader.launch(configuration: configuration)
            }

            // Check for overdue metrics pings
            self.metricsPingScheduler.schedule()

            // Check if the "dirty flag" is set. That means the product was probably
            // force-closed. If that's the case, submit a 'baseline' ping with the
            // reason "dirty_startup". We only do that from the second run.
            if !isFirstRun && isDirtyFlagSet {
                self.submitPingByNameSync(
                    pingName: "baseline",
                    reason: "dirty_startup"
                )
            }

            // From the second time we run, after all startup pings are generated,
            // make sure to clear `lifetime: application` metrics and set them again.
            // Any new value will be sent in newly generted pings after startup.
            // NOTE: we are adding this directly to the serialOperationQueue which
            // bypasses the queue for initial tasks, otherwise this could get lost
            // if the initial tasks queue overflows.
            if !isFirstRun {
                glean_clear_application_lifetime_metrics()
                self.initializeCoreMetrics()
            }

            // Signal the RLB dispatcher to unblock, if any exists.
            glean_flush_rlb_dispatcher()
            // Signal Dispatcher that init is complete
            Dispatchers.shared.flushQueuedInitialTasks()

            self.observer = GleanLifecycleObserver()
        }

        self.initFinished.value = true
    }

    // swiftlint:enable function_body_length cyclomatic_complexity

    /// Initialize the core metrics internally managed by Glean (e.g. client id).
    private func initializeCoreMetrics() {
        // Set a few more metrics that will be sent as part of every ping.
        // Please note that the following metrics must be set synchronously, so
        // that they are guaranteed to be available with the first ping that is
        // generated. We use an internal only API to do that.

        GleanInternalMetrics.osVersion.setSync(UIDevice.current.systemVersion)
        GleanInternalMetrics.deviceManufacturer.setSync(Sysctl.manufacturer)
        GleanInternalMetrics.deviceModel.setSync(Sysctl.model)
        GleanInternalMetrics.architecture.setSync(Sysctl.machine)
        GleanInternalMetrics.locale.setSync(getLocaleTag())

        if let channel = self.configuration?.channel {
            GleanInternalMetrics.appChannel.setSync(channel)
        }

        GleanInternalMetrics.appBuild.setSync(AppInfo.buildId)
        GleanInternalMetrics.appDisplayVersion.setSync(AppInfo.displayVersion)
    }

    /// Enable or disable Glean collection and upload.
    ///
    /// Metric collection is enabled by default.
    ///
    /// When uploading is disabled, metrics aren't recorded at all and no data
    /// is uploaded.
    ///
    /// When disabling, all pending metrics, events and queued pings are cleared.
    ///
    /// When enabling, the core Glean metrics are recreated.
    ///
    /// - parameters:
    ///     * enabled: When true, enable metric collection.
    public func setUploadEnabled(_ enabled: Bool) {
        if !self.initFinished.value {
            let msg = """
            Changing upload enabled before Glean is initialized is not supported.
            Pass the correct state into `Glean.initialize()`.
            See documentation at https://mozilla.github.io/glean/book/user/general-api.html#initializing-the-glean-sdk
            """
            self.logger.error(msg)

            return
        }
        // Changing upload enabled always happens asynchronous.
        // That way it follows what a user expect when calling it inbetween other calls:
        // It executes in the right order.
        //
        // Because the dispatch queue is halted until Glean is fully initialized
        // we can safely enqueue here and it will execute after initialization.
        Dispatchers.shared.launchAPI {
            // glean_set_upload_enabled might delete all of the queued pings.
            // Currently a ping uploader could be scheduled ahead of this,
            // at which point it will pick up scheduled pings before the setting was toggled.
            // Or it is scheduled afterwards and will not schedule or find any left-over pings to send.

            let originalEnabled = self.internalGetUploadEnabled()
            glean_set_upload_enabled(enabled.toByte())

            if !enabled {
                Dispatchers.shared.cancelBackgroundTasks()
            }

            if !originalEnabled && enabled {
                // If uploading is being re-enabled, we have to restore the
                // application-lifetime metrics.
                self.initializeCoreMetrics()
            }

            if originalEnabled && !enabled {
                // If uploading is disabled, we need to send the deletion-request ping
                HttpPingUploader.launch(configuration: self.configuration!)
            }
        }
    }

    /// Get whether or not Glean is allowed to record and upload data.
    ///
    /// Caution: the result is only correct if Glean is already initialized.
    ///
    /// **THIS METHOD IS DEPRECATED.**
    /// Applications should not rely on Glean's internal state.
    /// Upload enabled status should be tracked by the application and communicated to Glean if it changes.
    @available(*, deprecated,
               message: "Upload enabled should be tracked by the application and communicated to Glean if it changes")
    public func getUploadEnabled() -> Bool {
        return internalGetUploadEnabled()
    }

    /// Get whether or not Glean is allowed to record and upload data.
    ///
    /// Caution: the result is only correct if Glean is already initialized.
    func internalGetUploadEnabled() -> Bool {
        if isInitialized() {
            return glean_is_upload_enabled().toBool()
        } else {
            return false
        }
    }

    /// Used to indicate that an experiment is running.
    ///
    /// Glean will add an experiment annotation that is sent with pings.  This information is _not_
    /// persisted between runs.
    ///
    /// - parameters:
    ///     * experimentId: The id of the active experiment (maximum 100 bytes).
    ///     * branch: The branch of the experiment (maximum 100 bytes).
    ///     * extra: Optional metadata to output with the ping.
    public func setExperimentActive(experimentId: String, branch: String, extra: [String: String]?) {
        // The Dictionary is sent over FFI as a pair of arrays, one containing the
        // keys, and the other containing the values.
        // Keys and values are passed over the FFI boundary as arrays of strings, so
        // it is necessary to separate the dictionary into appropriate arrays.
        var keys = [String]()
        var values = [String]()
        if let extras = extra {
            for item in extras {
                keys.append(item.key)
                values.append(item.value)
            }
        }

        withArrayOfCStrings(keys) { keys in
            withArrayOfCStrings(values) { values in
                // We dispatch this asynchronously so that, if called before the Glean SDK is
                // initialized, it doesn't get ignored and will be replayed after init.
                Dispatchers.shared.launchAPI {
                    glean_set_experiment_active(
                        experimentId,
                        branch,
                        keys,
                        values,
                        Int32(extra?.count ?? 0)
                    )
                }
            }
        }
    }

    /// Used to indicate that an experiment is no longer running.
    ///
    /// - parameters:
    ///     * experimentsId: The id of the experiment to deactivate.
    public func setExperimentInactive(experimentId: String) {
        // We dispatch this asynchronously so that, if called before the Glean SDK is
        // initialized, it doesn't get ignored and will be replayed after init.
        Dispatchers.shared.launchAPI {
            glean_set_experiment_inactive(experimentId)
        }
    }

    /// Tests wheter an experiment is active, for testing purposes only.
    ///
    /// - parameters:
    ///     * experimentId: The id of the experiment to look for.
    ///
    /// - returns: `true` if the experiment is active and reported in pings.
    public func testIsExperimentActive(experimentId: String) -> Bool {
        Dispatchers.shared.assertInTestingMode()
        return glean_experiment_test_is_active(experimentId).toBool()
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Get recorded experiment data for a given `experimentId`.
    ///
    /// - parameters:
    ///     * experimentId: The id of the experiment to look for.
    ///
    /// - returns: `RecordedExperimentData` if the experiment is active and reported in pings, `nil` otherwise.
    public func testGetExperimentData(experimentId: String) -> RecordedExperimentData? {
        Dispatchers.shared.assertInTestingMode()
        let jsonString = String(
            freeingGleanString: glean_experiment_test_get_data(experimentId)
        )

        if let jsonData: Data = jsonString.data(using: .utf8, allowLossyConversion: false) {
            if let json = try? JSONSerialization.jsonObject(with: jsonData, options: []) as? [String: Any] {
                let experimentData = RecordedExperimentData(json: json)

                return experimentData
            }
        }

        return nil
    }

    /// Returns true if the Glean SDK has been initialized.
    func isInitialized() -> Bool {
        return self.initialized
    }

    /// Handle foreground event and submit appropriate pings
    func handleForegroundEvent() {
        Dispatchers.shared.launchAPI {
            glean_handle_client_active()
            // The above call may generate pings, so we need to trigger
            // the uploader. It's fine to trigger it if no ping was generated:
            // it will bail out.
            HttpPingUploader.launch(configuration: self.configuration!)
        }

        // Start the timespan for the new activity period.
        GleanBaseline.duration.start()
        GleanValidation.foregroundCount.add(1)
    }

    /// Handle background event and submit appropriate pings
    func handleBackgroundEvent() {
        // We're going to background, so store how much time we spent
        // on foreground.
        GleanBaseline.duration.stop()

        Dispatchers.shared.launchAPI {
            glean_handle_client_inactive()
            // The above call may generate pings, so we need to trigger
            // the uploader. It's fine to trigger it if no ping was generated:
            // it will bail out.
            HttpPingUploader.launch(configuration: self.configuration!)
        }
    }

    /// Collect and submit a ping by name for eventual uploading
    ///
    /// - parameters:
    ///     * pingName: Name of ping to submit.
    ///     * reason: The reason the ping is being submitted. Must be one of the strings
    ///       defined in the reasons field in the ping metadata.
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// If the ping currently contains no content, it will not be assembled and
    /// queued for sending.
    func submitPingByName(pingName: String, reason: String? = nil) {
        // Queue submitting the ping behind all other metric operations to include them in the ping
        Dispatchers.shared.launchAPI {
            self.submitPingByNameSync(pingName: pingName, reason: reason)
        }
    }

    /// Collect and submit a ping by name for eventual uploading, synchronously
    ///
    /// - parameters:
    ///     * pingName: Name of the ping to submit.
    ///     * reason: The reason the ping is being submitted. Must be one of the strings
    ///       defined in the reasons field in the ping metadata.
    ///
    /// The ping content is assembled as soon as possible, but upload is not
    /// guaranteed to happen immediately, as that depends on the upload
    /// policies.
    ///
    /// If the ping currently contains no content, it will not be assembled and
    /// queued for sending.
    func submitPingByNameSync(pingName: String, reason: String? = nil) {
        if !self.isInitialized() {
            self.logger.error("Glean must be initialized before sending pings")
            return
        }

        if !self.internalGetUploadEnabled() {
            self.logger.info("Glean disabled: not submitting any pings")
            return
        }

        let submittedPing = glean_submit_ping_by_name(
            pingName,
            reason
        )

        if submittedPing != 0 {
            if let config = self.configuration {
                HttpPingUploader.launch(configuration: config)
            }
        }
    }

    func submitPing(_ ping: PingBase, reason: String? = nil) {
        return self.submitPingByName(pingName: ping.name, reason: reason)
    }

    /// Register the pings generated from `pings.yaml` with the Glean SDK.
    ///
    /// - parameters:
    ///     * pings: The `Pings` object generated for your library or application
    ///              by the Glean SDK.
    public func registerPings(_: Any) {
        // Instantiating the Pings object to send this function is enough to
        // call the constructor and have it registered through [Glean.registerPingType].
        NSLog("Registering pings")
    }

    /// Register a `Ping` in the registry associated with this `Glean` object.
    func registerPingType(_ pingType: PingBase) {
        // If this happens after Glean.initialize is called (and returns),
        // we dispatch ping registration on the thread pool.
        // Registering a ping should not block the application.
        // Submission itself is also dispatched, so it will always come after the registration.
        if self.initFinished.value {
            Dispatchers.shared.launchAPI {
                glean_register_ping_type(pingType.handle)
            }
        }

        // We need to keep track of pings, so they get re-registered after a reset.
        // This state is kept across Glean resets, which should only ever happen in test mode.
        // Or by the instrumentation tests (`connectedAndroidTest`), which relaunches the application activity,
        // but not the whole process, meaning globals, such as the ping types, still exist from the old run.
        // It's a set and keeping them around forever should not have much of an impact.
        self.pingTypeQueue.append(pingType)
    }

    /// Set a tag to be applied to headers when uploading pings for debug view.
    /// This is only meant to be used internally by the `GleanDebugActivity`.
    ///
    /// - parameters:
    ///     * value: The value of the tag, which must be a valid HTTP header value.
    func setDebugViewTag(_ value: String) -> Bool {
        if self.isInitialized() {
            return glean_set_debug_view_tag(value).toBool()
        } else {
            debugViewTag = value
            return true
        }
    }

    /// Set the log_pings debug option,
    /// when this option is `true` the pings that are successfully submitted get logged.
    ///
    /// - parameters:
    ///     * value: The value of the option.
    func setLogPings(_ value: Bool) {
        if self.isInitialized() {
            glean_set_log_pings(value.toByte())
        } else {
            logPings = value
        }
    }

    func setSourceTags(_ value: [String]) -> Bool {
        if self.isInitialized() {
            let len = value.count
            return withArrayOfCStrings(value) { value in
                glean_set_source_tags(value, Int32(len)).toBool()
            }
        } else {
            sourceTags = value
            return true
        }
    }

    /// When applications are launched using the custom URL scheme, this helper function will process
    /// the URL and parse the debug commands
    ///
    /// - parameters:
    ///     * url: A `URL` object containing the Glean debug commands as query parameters
    ///
    /// There are 3 available commands that you can use with the Glean SDK debug tools
    ///
    /// - `logPings`: If "true", will cause pings that are submitted successfully to also be echoed to the device's log
    /// - `debugViewTag`:  This command expects a string to tag the pings with and redirects them to the Debug View
    /// - `sendPing`: This command expects a string name of a ping to force immediate collection and submission of.
    ///
    /// The structure of the custom URL uses the following format:
    ///
    /// `<protocol>://glean?<command 1>=<paramter 1>&<command 2>=<parameter 2> ...`
    ///
    /// Where:
    ///
    /// - `<protocol>://` is the "Url Scheme" that has been added for the app and doesn't matter to Glean.
    /// - `glean` is required for the Glean SDK to recognize the command is meant for it to process.
    /// - `?` indicating the beginning of the query.
    /// - `<command>=<parameter>` are instances of the commands listed above  separated by `&`.
    ///
    /// There are a few things to consider when creating the custom URL:
    ///
    /// - Invalid commands will trigger an error and be ignored.
    /// - Not all commands are requred to be encoded in the URL, you can mix and match the commands that you need.
    /// - Special characters should be properly URL encoded and escaped since this needs to represent a valid URL.
    public func handleCustomUrl(url: URL) {
        GleanDebugUtility.handleCustomUrl(url: url)
    }

    /// Returns true if running in the base application process, otherwise returns false
    private func checkIsMainProcess() -> Bool {
        if isMainProcess == nil {
            if let packageType = Bundle.main.object(forInfoDictionaryKey: "CFBundlePackageType") as? String {
                // This is the bundle type reported by embedded application extensions and so we test for it to
                // make sure that we are not running in an extension.
                //
                // For more info on XPC services see:
                // https://developer.apple.com/library/archive/documentation/MacOSX/Conceptual/BPSystemStartup/Chapters/CreatingXPCServices.html
                //
                // For more info on CFBundlePackageType see:
                // https://developer.apple.com/documentation/bundleresources/information_property_list/cfbundlepackagetype
                // and
                // https://developer.apple.com/library/archive/documentation/General/Reference/InfoPlistKeyReference/Articles/CoreFoundationKeys.html#//apple_ref/doc/uid/20001431-111321
                isMainProcess = packageType != "XPC!"
            } else {
                // The `CFBundlePackageType` doesn't get set in tests so we return true to indicate that we are
                // running in the main process.
                isMainProcess = true
            }
        }

        return isMainProcess!
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Returns true if a ping by this name is in the ping registry.
    public func testHasPingType(_ pingName: String) -> Bool {
        return glean_test_has_ping_type(pingName).toBool()
    }

    /// Test-only method to destroy the owned glean-core handle.
    func testDestroyGleanHandle() {
        if !isInitialized() {
            // We don't need to destroy Glean: it wasn't initialized.
            return
        }

        glean_destroy_glean()
        // Reset all state.
        Dispatchers.shared.setTaskQueueing(enabled: true)
        self.initFinished.value = false
        self.initialized = false
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Enable test mode.
    ///
    /// This makes all asynchronous work synchronous so we can test the results of the
    /// API synchronously.
    public func enableTestingMode() {
        Dispatchers.shared.setTestingMode(enabled: true)
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Resets the Glean state and trigger init again.
    ///
    /// - parameters:
    ///     * configuration: the `Configuration` to init Glean with
    ///     * clearStores: if true, clear the contents of all stores
    public func resetGlean(configuration: Configuration = Configuration(),
                           clearStores: Bool,
                           uploadEnabled: Bool = true) {
        enableTestingMode()

        if isInitialized() && clearStores {
            // Clear all the stored data.
            glean_test_clear_all_stores()
        }

        // Init Glean.
        testDestroyGleanHandle()
        // Enable ping logging for all tests
        setLogPings(true)
        initialize(uploadEnabled: uploadEnabled, configuration: configuration)
    }
}
