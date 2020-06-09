// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.FFI;
using System;
using System.Runtime.CompilerServices;

// Make sure internals are accessible by the Glean test suites.
[assembly: InternalsVisibleTo("GleanTests")]

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
        public void Initialize(
            string applicationId,
            string applicationVersion,
            bool uploadEnabled,
            Configuration configuration,
            string dataDir
            )
        {
            Console.WriteLine("Glean.init dir: {0}", dataDir);

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

            if (isInitialized()) {
                Log.e(LOG_TAG, "Glean should not be initialized multiple times")
                return
            }

            this.applicationContext = applicationContext

            this.configuration = configuration
            this.httpClient = BaseUploader(configuration.httpClient)
            this.gleanDataDir = File(applicationContext.applicationInfo.dataDir, GLEAN_DATA_DIR)

            setUploadEnabled(uploadEnabled)
             */

            Dispatchers.LaunchAPI(() => {
                // TODO: registerPings(Pings)

                LibGleanFFI.FfiConfiguration cfg = new LibGleanFFI.FfiConfiguration
                {
                    data_dir = dataDir,
                    package_name = applicationId,
                    upload_enabled = uploadEnabled,
                    max_events = configuration.maxEvents ?? null,
                    delay_ping_lifetime_io = false
                };

                initialized = LibGleanFFI.glean_initialize(cfg) != 0;

                // If initialization of Glean fails we bail out and don't initialize further.
                if (!initialized)
                {
                    return;
                }

                /* TODO:
                // If any pings were registered before initializing, do so now.
                // We're not clearing this queue in case Glean is reset by tests.
                synchronized(this@GleanInternalAPI) {
                    pingTypeQueue.forEach { registerPingType(it) }
                }
                */

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

                // We need to enqueue the PingUploadWorker in these cases:
                // 1. Pings were submitted through Glean and it is ready to upload those pings;
                // 2. Upload is disabled, to upload a possible deletion-request ping.
                if (pingSubmitted || !uploadEnabled)
                {
                    //PingUploadWorker.enqueueWorker(applicationContext)
                    Console.WriteLine("TODO: trigger ping upload now");
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
                if (!isFirstRun) {
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
            // TODO: Glean.enableTestingMode()

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
            // TODO:
        }
    }
}
