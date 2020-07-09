// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Collections.Generic;
using System.Globalization;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Mozilla.Glean.FFI;
using Mozilla.Glean.Net;
using Mozilla.Glean.Private;
using Serilog;
using static Mozilla.Glean.GleanMetrics.GleanInternalMetricsOuter;
using static Mozilla.Glean.GleanPings.GleanInternalPingsOuter;
using static Mozilla.Glean.Utils.GleanLogger;

namespace Mozilla.Glean
{
    /// <summary>
    /// The Glean Gneral API.
    /// </summary>
    public sealed class Glean
    {
        // Initialize the singleton using the `Lazy` facilities.
        private static readonly Lazy<Glean>
          lazy = new Lazy<Glean>(() => new Glean());
        public static Glean GleanInstance => lazy.Value;

        private bool initialized = false;

        // Keep track of ping types that have been registered before Glean is initialized.
        private HashSet<PingTypeBase> pingTypeQueue = new HashSet<PingTypeBase>();

        private Configuration configuration;

        // This is the wrapped http uploading mechanism: provides base functionalities
        // for logging and delegates the actual upload to the implementation in
        // the `Configuration`.
        private BaseUploader httpClient;

        // The version of the application sending Glean data.
        private string applicationVersion;

        /// <summary>
        /// This is the tag used for logging from this class.
        /// </summary>
        private const string LogTag = "glean/Glean";

        /// <summary>
        /// This is the name of the language used by this Glean binding.
        /// </summary>
        private readonly static string LanguageBindingName = "C#";

        /// <summary>
        /// A logger configured for this class
        /// </summary>
        private static readonly ILogger Log = GetLogger(LogTag);

        private Glean()
        {
            // Private constructor to disallow instantiation since
            // this is meant to be a singleton. It only wires up the
            // glean-core logging on the Rust side.
            LibGleanFFI.glean_enable_logging();
        }

        /// <summary>
        /// Initialize the Glean SDK.
        ///
        /// This should only be initialized once by the application, and not by
        /// libraries using the Glean SDK. A message is logged to error and no
        /// changes are made to the state if initialize is called a more than
        /// once.
        ///
        /// This method must be called from the main thread.
        /// </summary>
        /// <param name="applicationId">The application id to use when sending pings.</param>
        /// <param name="applicationVersion">The version of the application sending
        /// Glean data.</param>
        /// <param name="uploadEnabled">A `bool` that determines whether telemetry is enabled.
        /// If disabled, all persisted metrics, events and queued pings (except first_run_date)
        /// are cleared.</param>
        /// <param name="configuration">A Glean `Configuration` object with global settings</param>
        /// <param name="dataDir">The path to the Glean data directory.</param>
        [MethodImpl(MethodImplOptions.Synchronized)]
        public void Initialize(
            string applicationId,
            string applicationVersion,
            bool uploadEnabled,
            Configuration configuration,
            string dataDir
            )
        {
            /*
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

            this.applicationContext = applicationContext*/

            if (IsInitialized()) {
                Log.Error("Glean should not be initialized multiple times");
                return;
            }

            this.configuration = configuration;
            this.applicationVersion = applicationVersion;
            httpClient = new BaseUploader(configuration.httpClient);
            // this.gleanDataDir = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR)

            Dispatchers.ExecuteTask(() =>
            {
                RegisterPings(GleanInternalPings);

                IntPtr maxEventsPtr = IntPtr.Zero;
                if (configuration.maxEvents != null)
                {
                    maxEventsPtr = Marshal.AllocHGlobal(sizeof(int));
                    // It's safe to call `configuration.maxEvents.Value` because we know
                    // `configuration.maxEvents` is not null.
                    Marshal.WriteInt32(maxEventsPtr, configuration.maxEvents.Value);
                }

                LibGleanFFI.FfiConfiguration cfg = new LibGleanFFI.FfiConfiguration
                {
                    data_dir = dataDir,
                    package_name = applicationId,
                    language_binding_name = LanguageBindingName,
                    upload_enabled = uploadEnabled,
                    max_events = maxEventsPtr,
                    delay_ping_lifetime_io = false
                };

                // To work around a bug in the version of Mono shipped with Unity 2019.4.1f1,
                // copy the FFI configuration structure to unmanaged memory and pass that over
                // to glean-core, otherwise calling `glean_initialize` will crash and have
                // `__icall_wrapper_mono_struct_delete_old` in the stack. See bug 1648784 for
                // more details.
                IntPtr ptrCfg = Marshal.AllocHGlobal(Marshal.SizeOf(cfg));
                Marshal.StructureToPtr(cfg, ptrCfg, false);

                initialized = LibGleanFFI.glean_initialize(ptrCfg) != 0;

                // This is safe to call even if `maxEventsPtr = IntPtr.Zero`.
                Marshal.FreeHGlobal(maxEventsPtr);
                // We were able to call `glean_initialize`, free the memory allocated for the
                // FFI configuration object.
                Marshal.FreeHGlobal(ptrCfg);

                // If initialization of Glean fails we bail out and don't initialize further.
                if (!initialized)
                {
                    return;
                }

                // If any pings were registered before initializing, do so now.
                // We're not clearing this queue in case Glean is reset by tests.
                lock (this)
                {
                    foreach (var ping in pingTypeQueue)
                    {
                        RegisterPingType(ping);
                    }
                }

                // If this is the first time ever the Glean SDK runs, make sure to set
                // some initial core metrics in case we need to generate early pings.
                // The next times we start, we would have them around already.
                bool isFirstRun = LibGleanFFI.glean_is_first_run() != 0;
                if (isFirstRun)
                {
                    InitializeCoreMetrics();
                }


                // Deal with any pending events so we can start recording new ones
                bool pingSubmitted = LibGleanFFI.glean_on_ready_to_submit_pings() != 0;

                // We need to enqueue the BaseUploader in these cases:
                // 1. Pings were submitted through Glean and it is ready to upload those pings;
                // 2. Upload is disabled, to upload a possible deletion-request ping.
                if (pingSubmitted || !uploadEnabled)
                {
                    httpClient.TriggerUpload(configuration);
                }
                /*
                // Set up information and scheduling for Glean owned pings. Ideally, the "metrics"
                // ping startup check should be performed before any other ping, since it relies
                // on being dispatched to the API context before any other metric.
                metricsPingScheduler = MetricsPingScheduler(applicationContext)
                metricsPingScheduler.schedule()

                // Check if the "dirty flag" is set. That means the product was probably
                // force-closed. If that's the case, submit a 'baseline' ping with the
                // reason "dirty_startup". We only do that from the second run.
                if (!isFirstRun && LibGleanFFI.INSTANCE.glean_is_dirty_flag_set().toBoolean()) {
                    submitPingByNameSync("baseline", "dirty_startup")
                    // Note: while in theory we should set the "dirty flag" to true
                    // here, in practice it's not needed: if it hits this branch, it
                    // means the value was `true` and nothing needs to be done.
                }*/

                // From the second time we run, after all startup pings are generated,
                // make sure to clear `lifetime: application` metrics and set them again.
                // Any new value will be sent in newly generated pings after startup.
                if (!isFirstRun)
                {
                    LibGleanFFI.glean_clear_application_lifetime_metrics();
                    InitializeCoreMetrics();
                }

                // Signal Dispatcher that init is complete
                Dispatchers.FlushQueuedInitialTasks();
                /*
                // At this point, all metrics and events can be recorded.
                // This should only be called from the main thread. This is enforced by
                // the @MainThread decorator and the `assertOnUiThread` call.
                MainScope().launch {
                    ProcessLifecycleOwner.get().lifecycle.addObserver(gleanLifecycleObserver)
                }*/
            });
        }

        /// <summary>
        /// Whether or not the Glean SDK has been initialized.
        /// </summary>
        /// <returns>Returns true if the Glean SDK has been initialized.</returns>
        internal bool IsInitialized()
        {
            return initialized;
        }

        /// <summary>
        /// Enable or disable Glean collection and upload.
        ///
        /// Metric collection is enabled by default.
        ///
        /// When uploading is disabled, metrics aren't recorded at all and no data
        /// is uploaded.
        ///
        /// When disabling, all pending metrics, events and queued pings are cleared
        /// and a `deletion-request` is generated.
        ///
        /// When enabling, the core Glean metrics are recreated.
        /// </summary>
        /// <param name="enabled">When `true`, enable metric collection.</param>
        public void SetUploadEnabled(bool enabled)
        {
            // Changing upload enabled always happens asynchronous.
            // That way it follows what a user expect when calling it inbetween other calls:
            // It executes in the right order.
            //
            // Because the dispatch queue is halted until Glean is fully initialized
            // we can safely enqueue here and it will execute after initialization.
            Dispatchers.LaunchAPI(() => {
                bool originalEnabled = this.GetUploadEnabled();
                LibGleanFFI.glean_set_upload_enabled(enabled);

                if (!enabled)
                {
                    // Cancel any pending workers here so that we don't accidentally upload or
                    // collect data after the upload has been disabled.
                    // TODO: metricsPingScheduler.cancel()

                    // Cancel any pending workers here so that we don't accidentally upload
                    // data after the upload has been disabled.
                    httpClient.CancelUploads();
                }

                if (!originalEnabled && enabled)
                {
                    // If uploading is being re-enabled, we have to restore the
                    // application-lifetime metrics.
                    InitializeCoreMetrics();
                }

                if (originalEnabled && !enabled)
                {
                    // If uploading is disabled, we need to send the deletion-request ping
                    httpClient.TriggerUpload(configuration);
                }
            });
        }

        /// <summary>
        /// Get whether or not Glean is allowed to record and upload data.
        ///
        /// Caution: the result is only correct if Glean is already initialized.
        /// </summary>
        /// <returns>`true` if Glean is allowed to record and upload data.</returns>
        public bool GetUploadEnabled()
        {
            if (IsInitialized())
            {
                return LibGleanFFI.glean_is_upload_enabled() != 0;
            }
            else
            {
                return true;
            }
        }

        /// <summary>
        /// TEST ONLY FUNCTION.
        /// Resets the Glean state and triggers init again.
        /// </summary>
        /// <param name="applicationId">The application id to use when sending pings.</param>
        /// <param name="applicationVersion">The version of the application sending
        /// Glean data.</param>
        /// <param name="uploadEnabled">A `bool` that determines whether telemetry is enabled.
        /// If disabled, all persisted metrics, events and queued pings (except first_run_date)
        /// are cleared.</param>
        /// <param name="configuration">A Glean `Configuration` object with global settings</param>
        /// <param name="dataDir">The path to the Glean data directory.</param>
        /// <param name="clearStores">If `true` clear the contents of all stores.</param>
        internal void Reset(
            string applicationId,
            string applicationVersion,
            bool uploadEnabled,
            Configuration configuration,
            string dataDir,
            bool clearStores = true)
        {
            Dispatchers.TestingMode = true;

            if (IsInitialized() && clearStores)
            {
                // Clear all the stored data.
                LibGleanFFI.glean_test_clear_all_stores();
            }

            // TODO: isMainProcess = null

            // Init Glean.
            TestDestroyGleanHandle();
            Initialize(applicationId, applicationVersion, uploadEnabled, configuration, dataDir);
        }

        /// <summary>
        /// TEST ONLY FUNCTION.
        /// Destroy the owned glean-core handle.
        /// </summary>
        internal void TestDestroyGleanHandle()
        {
            if (!IsInitialized())
            {
                // We don't need to destroy Glean: it wasn't initialized.
                return;
            }

            LibGleanFFI.glean_destroy_glean();
            initialized = false;
        }

        private void InitializeCoreMetrics()
        {
            // The `Environment.OSVersion` object will return a version that represents an approximate
            // version of the OS. As the MSDN docs state, this is unreliable:
            // https://docs.microsoft.com/en-us/dotnet/api/system.environment.osversion?redirectedfrom=MSDN&view=netstandard-2.0
            // However, there's really nothing we could do about it. Unless the product using the
            // Glean SDK correctly marks their manifest as Windows 10 compatible, this API will report
            // Windows 8 version (6.2). See the remarks section here:
            // https://docs.microsoft.com/en-gb/windows/win32/api/winnt/ns-winnt-osversioninfoexa?redirectedfrom=MSDN#remarks
            try
            {
                GleanInternalMetrics.osVersion.SetSync(Environment.OSVersion.Version.ToString());
            }
            catch (InvalidOperationException)
            {
                GleanInternalMetrics.osVersion.SetSync("Unknown");
            }

            // Possible values for `RuntimeInformation.OSArchitecture` are documented here:
            // https://docs.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.architecture?view=netstandard-2.0
            GleanInternalMetrics.architecture.SetSync(RuntimeInformation.OSArchitecture.ToString());

            try
            {
                // CurrentUiCulture is used for representing the locale of the UI, coming from the OS,
                // while CurrentCulture is the general locale used for other things (e.g. currency).
                GleanInternalMetrics.locale.SetSync(CultureInfo.CurrentUICulture.Name);
            }
            catch (Exception)
            {
                GleanInternalMetrics.locale.SetSync("und");
            }

            if (configuration.channel != null)
            {
                GleanInternalMetrics.appChannel.SetSync(configuration.channel);
            }

            GleanInternalMetrics.appDisplayVersion.SetSync(applicationVersion);
            GleanInternalMetrics.appBuild.SetSync(configuration.buildId ?? "Unknown");
        }

        /// <summary>
        /// Register the pings generated from `ManualPings` with the Glean SDK.
        /// </summary>
        /// <param name="pings"> The `Pings` object generated for your library or application
        /// by the Glean SDK.</param>
        private void RegisterPings(object pings)
        {
            // Instantiating the Pings object to send this function is enough to
            // call the constructor and have it registered through [Glean.registerPingType].
            Log.Information("Registering pings");
        }

        /// <summary>
        /// Handle the background event and send the appropriate pings.
        /// </summary>
        internal void HandleBackgroundEvent()
        {
            GleanInternalPings.baseline.Submit(BaselineReasonCodes.background);
            GleanInternalPings.events.Submit(EventsReasonCodes.background);
        }

        /// <summary>
        /// Collect and submit a ping for eventual upload.
        ///
        /// The ping content is assembled as soon as possible, but upload is not
        /// guaranteed to happen immediately, as that depends on the upload
        /// policies.
        ///
        /// If the ping currently contains no content, it will not be assembled and
        /// queued for sending.
        /// </summary>
        /// <param name="ping">Ping to submit.</param>
        /// <param name="reason">The reason the ping is being submitted.</param>
        internal void SubmitPing(PingTypeBase ping, string reason = null)
        {
            SubmitPingByName(ping.name, reason);
        }

        /// <summary>
        /// Collect and submit a ping for eventual upload by name.
        ///
        /// The ping will be looked up in the known instances of `PingType`. If the
        /// ping isn't known, an error is logged and the ping isn't queued for uploading.
        ///
        /// The ping content is assembled as soon as possible, but upload is not
        /// guaranteed to happen immediately, as that depends on the upload
        /// policies.
        ///
        /// If the ping currently contains no content, it will not be assembled and
        /// queued for sending, unless explicitly specified otherwise in the registry
        /// file.
        /// </summary>
        /// <param name="name">Name of the ping to submit.</param>
        /// <param name="reason">The reason the ping is being submitted.</param>
        internal void SubmitPingByName(string name, string reason = null)
        {
            Dispatchers.LaunchAPI(() =>
            {
                SubmitPingByNameSync(name, reason);
            });
        }

        /// <summary>
        /// Collect and submit a ping (by its name) for eventual upload, synchronously.
        ///
        /// The ping will be looked up in the known instances of `PingType`. If the
        /// ping isn't known, an error is logged and the ping isn't queued for uploading.
        ///
        /// The ping content is assembled as soon as possible, but upload is not
        /// guaranteed to happen immediately, as that depends on the upload
        /// policies.
        ///
        /// If the ping currently contains no content, it will not be assembled and
        /// queued for sending, unless explicitly specified otherwise in the registry
        /// file.
        /// </summary>
        /// <param name="name">Name of the ping to submit.</param>
        /// <param name="reason">The reason the ping is being submitted.</param>
        internal void SubmitPingByNameSync(string name, string reason = null)
        {
            if (!IsInitialized())
            {
                Log.Error("Glean must be initialized before submitting pings.");
                return;
            }

            if (!GetUploadEnabled())
            {
                Log.Error("Glean disabled: not submitting any pings.");
                return;
            }

            bool hasSubmittedPing = Convert.ToBoolean(LibGleanFFI.glean_submit_ping_by_name(name, reason));
            if (hasSubmittedPing)
            {
                httpClient.TriggerUpload(configuration);
            }
        }

        /// <summary>
        /// Register a [PingType] in the registry associated with this [Glean] object.
        /// </summary>
        [MethodImpl(MethodImplOptions.Synchronized)]
        internal void RegisterPingType(PingTypeBase pingType)
        {
            if (IsInitialized())
            {
                LibGleanFFI.glean_register_ping_type(
                    pingType.handle
                );
            }

            // We need to keep track of pings, so they get re-registered after a reset.
            // This state is kept across Glean resets, which should only ever happen in test mode.
            // Or by the instrumentation tests (`connectedAndroidTest`), which relaunches the application activity,
            // but not the whole process, meaning globals, such as the ping types, still exist from the old run.
            // It's a set and keeping them around forever should not have much of an impact.

            pingTypeQueue.Add(pingType);
        }

        /// <summary>
        /// TEST ONLY FUNCTION.
        /// Returns true if a ping by this name is in the ping registry.
        /// </summary>
        internal bool TestHasPingType(string pingName)
        {
            return LibGleanFFI.glean_test_has_ping_type(pingName) != 0;
        }
}
}
