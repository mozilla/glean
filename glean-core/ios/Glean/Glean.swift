/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation
import UIKit

private typealias GleanBaseline = GleanMetrics.GleanBaseline
private typealias GleanInternalMetrics = GleanMetrics.GleanInternalMetrics
private typealias GleanValidation = GleanMetrics.GleanValidation
private typealias Pings = GleanMetrics.Pings

/// Public exported type identifying individual timers for `TimingDistributionMetricType`
public typealias GleanTimerId = TimerId

class OnGleanEventsImpl: OnGleanEvents {
    let glean: Glean

    init(glean: Glean) {
        self.glean = glean
    }

    func onInitializeFinished() {
        // Run this off the main thread,
        // as it will trigger a ping submission,
        // which itself will trigger `triggerUpload()` on this class.
        Dispatchers.shared.launchAsync {
            self.glean.observer = GleanLifecycleObserver()
        }
        self.glean.initialized = true
    }

    func triggerUpload() {
        // If uploading is disabled, we need to send the deletion-request ping
        HttpPingUploader.launch(configuration: self.glean.configuration!, self.glean.testingMode.value)
    }

    func startMetricsPingScheduler() -> Bool {
        self.glean.metricsPingScheduler = MetricsPingScheduler(self.glean.testingMode.value)
        // Check for overdue metrics pings
        return self.glean.metricsPingScheduler!.schedule()
    }

    func cancelUploads() {
        // intentionally left empty
    }
}

public struct BuildInfo {
    var buildDate: DateComponents

    public init(buildDate: DateComponents) {
        self.buildDate = buildDate
    }
}

/// The main Glean API.
///
/// This is exposed through the global `Glean.shared` object.
public class Glean {
    /// The main Glean object.
    ///
    /// ```swift
    /// Glean.shared.initialize(uploadEnabled: true)
    /// ```
    public static let shared = Glean()

    var metricsPingScheduler: MetricsPingScheduler?

    var initialized: Bool = false

    // Are we in testing mode?
    internal var testingMode = AtomicBoolean(false)

    var configuration: Configuration?
    private var buildInfo: BuildInfo?
    fileprivate var observer: GleanLifecycleObserver?

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/Glean"
        static let languageBindingName = "Swift"
    }

    private let logger = Logger(tag: Constants.logTag)

    // Cache variable for checking if running in main process.  Also used to override for tests in
    // order to simulate not running in the main process.  DO NOT SET EXCEPT IN TESTS!
    var isMainProcess: Bool?

    private init() {
        // intentionally left private, no external user can instantiate a new global object.

        // Enable logging in the Rust library
        gleanEnableLogging()
    }

    deinit {
        self.initialized = false
    }

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
    ///       first_run_date) are cleared.
    ///     * configuration: A Glean `Configuration` object with global settings.
    public func initialize(uploadEnabled: Bool,
                           configuration: Configuration = Configuration(),
                           buildInfo: BuildInfo) {
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

        self.buildInfo = buildInfo
        self.configuration = configuration
        let cfg = InternalConfiguration(
            dataPath: getGleanDirectory().relativePath,
            applicationId: AppInfo.name,
            languageBindingName: Constants.languageBindingName,
            uploadEnabled: uploadEnabled,
            maxEvents: configuration.maxEvents.map { UInt32($0) },
            delayPingLifetimeIo: false,
            appBuild: "0.0.0",
            useCoreMps: false,
            trimDataToRegisteredPings: false
        )
        let clientInfo = getClientInfo(configuration, buildInfo: buildInfo)
        let callbacks = OnGleanEventsImpl(glean: self)
        gleanInitialize(cfg, clientInfo, callbacks)
    }

    /// Initialize the core metrics internally managed by Glean (e.g. client id).
    internal func getClientInfo(_ configuration: Configuration, buildInfo: BuildInfo) -> ClientInfoMetrics {
        return ClientInfoMetrics(
            appBuild: AppInfo.buildId,
            appDisplayVersion: AppInfo.displayVersion,
            appBuildDate: Datetime(from: buildInfo.buildDate),
            architecture: Sysctl.machine,
            osVersion: UIDevice.current.systemVersion,
            channel: configuration.channel,
            locale: getLocaleTag(),
            deviceManufacturer: Sysctl.manufacturer,
            deviceModel: Sysctl.model
        )
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
        gleanSetUploadEnabled(enabled)
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
    public func setExperimentActive(_ experimentId: String, branch: String, extra: [String: String]?) {
        let map = extra ?? [:]
        gleanSetExperimentActive(experimentId, branch, map)
    }

    /// Used to indicate that an experiment is no longer running.
    ///
    /// - parameters:
    ///     * experimentsId: The id of the experiment to deactivate.
    public func setExperimentInactive(_ experimentId: String) {
        gleanSetExperimentInactive(experimentId)
    }

    /// Tests wheter an experiment is active, for testing purposes only.
    ///
    /// - parameters:
    ///     * experimentId: The id of the experiment to look for.
    ///
    /// - returns: `true` if the experiment is active and reported in pings.
    public func testIsExperimentActive(_ experimentId: String) -> Bool {
        return gleanTestGetExperimentData(experimentId) != nil
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Get recorded experiment data for a given `experimentId`.
    ///
    /// - parameters:
    ///     * experimentId: The id of the experiment to look for.
    ///
    /// - returns: `RecordedExperiment` if the experiment is active and reported in pings, `nil` otherwise.
    public func testGetExperimentData(_ experimentId: String) -> RecordedExperiment? {
        return gleanTestGetExperimentData(experimentId)
    }

    /// Returns true if the Glean SDK has been initialized.
    func isInitialized() -> Bool {
        return self.initialized
    }

    /// Handle foreground event and submit appropriate pings
    func handleForegroundEvent() {
        gleanHandleClientActive()

        GleanValidation.foregroundCount.add(1)
    }

    /// Handle background event and submit appropriate pings
    func handleBackgroundEvent() {
        gleanHandleClientInactive()
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
    func submitPingByName(_ pingName: String, _ reason: String? = nil) {
        gleanSubmitPingByName(pingName, reason)
    }

    /// Register the pings generated from `pings.yaml` with the Glean SDK.
    ///
    /// - parameters:
    ///     * pings: The `Pings` object generated for your library or application
    ///              by the Glean SDK.
    public func registerPings(_: Any) {
        NSLog("Registering pings")
    }

    /// Set a tag to be applied to headers when uploading pings for debug view.
    /// This is only meant to be used internally by the `GleanDebugActivity`.
    ///
    /// - parameters:
    ///     * value: The value of the tag, which must be a valid HTTP header value.
    func setDebugViewTag(_ tag: String) -> Bool {
        return gleanSetDebugViewTag(tag)
    }

    /// Set the log_pings debug option,
    /// when this option is `true` the pings that are successfully submitted get logged.
    ///
    /// - parameters:
    ///     * value: The value of the option.
    func setLogPings(_ value: Bool) {
        gleanSetLogPings(value)
    }

    /// Set the source tags to be applied as headers when uploading pings.
    ///
    /// If any of the tags is invalid nothing will be set and this function will
    /// return `false`.
    /// If Glean is not initialized yet, tags will not be validated at this point.
    ///
    /// - parameters:
    ///    * tags: A list of tags, which must be valid HTTP header values.
    public func setSourceTags(_ tags: [String]) -> Bool {
        gleanSetSourceTags(tags)
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

    /// Test-only method to destroy the owned glean-core handle.
    func testDestroyGleanHandle(_ clearStores: Bool = false) {
        // If it was initialized this also clears the directory
        let dataPath = getGleanDirectory().relativePath
        gleanTestDestroyGlean(clearStores, dataPath)

        if !isInitialized() {
            // We don't need to destroy anything else: it wasn't initialized.
            return
        }

        // Reset all state
        gleanSetTestMode(false)
        self.testingMode.value = false
        self.initialized = false
        self.metricsPingScheduler = nil
    }

    /// PUBLIC TEST ONLY FUNCTION.
    ///
    /// Enable test mode.
    ///
    /// This makes all asynchronous work synchronous so we can test the results of the
    /// API synchronously.
    public func enableTestingMode() {
        self.testingMode.value = true
        gleanSetTestMode(true)
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
        // Init Glean.
        testDestroyGleanHandle(clearStores)

        // Enable test mode.
        enableTestingMode()
        // Enable ping logging for all tests
        setLogPings(true)

        let buildInfo = BuildInfo(
            buildDate: DateComponents(
                calendar: Calendar.current,
                timeZone: TimeZone(abbreviation: "UTC"),
                year: 2020,
                month: 1,
                day: 1,
                hour: 0,
                minute: 0,
                second: 0
            )
        )
        initialize(uploadEnabled: uploadEnabled, configuration: configuration, buildInfo: buildInfo)
    }
}
