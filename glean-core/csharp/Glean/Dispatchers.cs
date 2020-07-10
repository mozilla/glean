// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;
using System.Threading.Tasks.Schedulers;
using Serilog;
using static Mozilla.Glean.Utils.GleanLogger;

namespace Mozilla.Glean
{
    internal static class Dispatchers
    {
        /// <summary>
        /// This is the tag used for logging from this class.
        /// </summary>
        private const string LogTag = "glean/Dispatchers";

        /// <summary>
        /// This is the number of milliseconds that are allowed for the initial
        /// tasks queue to process all of the queued tasks.
        /// </summary>
        private const int QueueProcessingTimeoutMs = 5000;

        /// <summary>
        /// This is the maximum number of tasks that will be queued before
        /// Glean is initialized.
        /// </summary>
        internal const int MaxQueueSize = 100;

        /// <summary>
        /// When true, tasks will be executed synchronously.
        /// </summary>
        internal static bool TestingMode { get; set; } = false;

        // Private backing field for QueueInitialTasks.
        private static int _queueInitialTasks = 1;
        /// <summary>
        /// When true, tasks will be queued and not ran until triggered by
        /// calling FlushQueuedInitialTasks.  This uses an int backing field
        /// in order to take advantage of Interlocked.Exchange for thread
        /// safety.
        /// </summary>
        internal static bool QueueInitialTasks
        {
            get => _queueInitialTasks == 1;
            set => Interlocked.Exchange(ref _queueInitialTasks, value ? 1 : 0);
        }

        // Create a scheduler that uses a single thread.
        private static readonly LimitedConcurrencyLevelTaskScheduler
            apiScheduler = new LimitedConcurrencyLevelTaskScheduler(1);

        // Create a new TaskFactory and pass it the scheduler.
        private static readonly TaskFactory factory = new TaskFactory(apiScheduler);

        /// <summary>
        /// This Queue holds the initial Actions that are launched before Glean
        /// is initialized.
        /// </summary>
        internal static ConcurrentQueue<Action> preInitActionQueue =
            new ConcurrentQueue<Action>();

        /// <summary>
        /// The number of Actions added to the queue beyond the MaxQueueSize.
        /// </summary>
        private static int overflowCount = 0;

        /// <summary>
        /// A logger configured for this class.
        /// </summary>
        private static readonly ILogger Log = GetLogger(LogTag);

        /// <summary>
        /// Launch a block of work asynchronously.
        ///
        /// Takes an Action and launches it using the TaskFactory ensuring the
        /// tasks are executed serially in the order the were launched.
        ///
        /// If `QueueInitialTasks` is enabled, then the operation will be
        /// created and added to the `preInitActionQueue` but not executed until
        /// flushed.
        ///
        /// If `TestingMode` is enabled, then `LaunchAPI` will execute the task
        /// immediately and synchronously to avoid asynchronous issues in tests.
        /// </summary>
        /// <param name="action">The Action to invoke</param>
        /// <returns>A Task or null if queued or run synchronously</returns>
        internal static Task LaunchAPI(Action action)
        {
            Task task = null;

            // Wrap the provided action in a try/catch block: we don't want to
            // break execution if something throws.
            Action safeAction = () => {
                try
                {
                    action.Invoke();
                }
                catch (Exception e)
                {
                    Log.Error(e, "Exception thrown by task and swallowed.");

                }
            };

            if (QueueInitialTasks)
            {
                // If we are queuing, typically before Glean has been
                // initialized, then we should just add the action to
                // the queue.
                AddActionToQueue(safeAction);
            }
            else
            {
                if (!TestingMode)
                {
                    // If we are not queuing we can go ahead and execute the
                    // task asynchronously                    
                    task = factory.StartNew(safeAction);
                }
                else
                {
                    // If we are in testing mode, then go ahead and await
                    // the task to ensure synchronous execution.
                    safeAction.Invoke();
                }
            }

            return task;
        }

        /// <summary>
        /// Stops queueing Actions and processes any Actions in the queue. 
        /// </summary>
        internal static void FlushQueuedInitialTasks()
        {
            factory.StartNew(() =>
            {                
                QueueInitialTasks = false;

                while (preInitActionQueue.TryDequeue(out Action action))
                {
                    action.Invoke();
                }                
            }).Wait();

            // This must happen after `QueueInitialTasks = false` is run, or
            // it would be added to a full task queue and be silently dropped.
            if (overflowCount > 0)
            {
                //TODO GleanError.preinitTasksOverflow
                //         .addSync(MaxQueueSize + OverflowCount)
            }
        }

        internal static Task ExecuteTask(Action action)
        {
            if (TestingMode)
            {
                // If we are in testing mode, then invoke the action and return
                // null
                action.Invoke();
                return null;
            }
            else
            {
                // **NOTE** This does not ensure that this task is executed at
                // the front of the apiScheduler. This will be addressed with
                // https://bugzilla.mozilla.org/show_bug.cgi?id=1646750
                return factory.StartNew(action);
            }
        }

        /// <summary>
        /// Helper function to add an Action to the queue and monitor the
        /// MaxQueueSize.
        /// </summary>
        /// <param name="task">The Task to add to the queue</param>
        [MethodImpl(MethodImplOptions.Synchronized)]
        private static void AddActionToQueue(Action action)
        {
            if (preInitActionQueue.Count >= MaxQueueSize)
            {
                Log.Error("Exceeded maximum queue size, discarding task");

                // This value ends up in the `preinit_tasks_overflow` metric,
                // but we can't record directly there, because that would only
                // add the recording to an already-overflowing task queue and
                // would be silently dropped.
                overflowCount += 1;
                return;
            }

            if (TestingMode)
            {
                Log.Information("Task queued for execution in test mode");
            }
            else
            {
                Log.Information("Task queued for execution and delayed until flushed");
            }
            
            preInitActionQueue.Enqueue(action);
        }

        /// <summary>
        /// Helper function to ensure the Glean SDK is being used in testing
        /// mode and async jobs are being run synchronously. This should be
        /// called from every method of the testing API to make sure that the
        /// results of the main API can be tested as expected.
        /// </summary>
        public static void AssertInTestingMode()
        {
            Debug.Assert(TestingMode);
        }        
    }
}
