// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Threading;
using System.Threading.Tasks;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Collections.Concurrent;

namespace Mozilla.Glean
{
    internal class Dispatchers
    {
        /// <summary>
        /// This is the tag used for logging from this class
        /// </summary>
        private const string LogTag = "glean/Dispatchers";

        /// <summary>
        /// This is the number of seconds that are allowed for the initial
        /// tasks queue to process all of the queued tasks.
        /// </summary>
        private const int QueueProcessingTimeout = 5;

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
        /// calling FlushQueuedInitialTasks.
        /// </summary>
        internal static bool QueueInitialTasks
        {
            get => _queueInitialTasks == 1;
            set => Interlocked.Exchange(ref _queueInitialTasks, value ? 1 : 0);
        }

        // Create a ConcurrentExclusiveSchedulerPair object.
        // All API tasks will be executed on the exclusive part of the scheduler
        // in order to ensure single threaded behavior
        private static readonly
            ConcurrentExclusiveSchedulerPair taskSchedulerPair =
            new ConcurrentExclusiveSchedulerPair();

        /// <summary>
        /// This Queue holds the initial tasks that are launched before Glean is
        /// initialized.
        /// </summary>
        internal static ConcurrentQueue<Action> taskQueue =
            new ConcurrentQueue<Action>();

        /// <summary>
        /// The number of tasks added to the queue beyond the MaxQueueSize.
        /// </summary>
        private static int OverflowCount { get; set; } = 0;

        /// <summary>
        /// Provides the token source for cancellation tokens used to cancel
        /// running background tasks.
        /// </summary>
        private static readonly CancellationTokenSource tokenSource =
            new CancellationTokenSource();

        /// <summary>
        /// This is the token that is used along with the tasks in order to be
        /// able to cancel them.
        /// </summary>
        private static CancellationToken cancellationToken = tokenSource.Token;

        /// <summary>
        /// Launch a block of work asynchronously.
        ///
        /// Takes an Action and launches it as a Task on the ExclusiveScheduler
        /// of the ConcurrentExclusiveSchedulerPair which ensures the tasks are
        /// executed serially in the order the were launched.
        ///
        /// **Note:** Tasks that should be processed in order and finish before
        /// successive tasks are run should be launched using the `LaunchAPI`
        /// function.  This includes all metric recording functions. For
        /// launching of tasks that need to be processed asynchronously but
        /// should not block other tasks, see `LaunchConcurrent`.
        ///
        /// If `QueueInitialTasks` is enabled, then the operation will be
        /// created and added to the `taskQueue` but not executed until flushed.
        ///
        /// If `TestingMode` is enabled, then `LaunchAPI` will execute the task
        /// immediately and synchronously to avoid asynchronous issues in tests.
        /// </summary>
        /// <param name="action">The Action to invoke</param>
        /// <returns>A Task or null if queued or run synchronously</returns>
        internal static Task LaunchAPI(Action action)
        {
            Task task = null;

            if (QueueInitialTasks)
            {
                // If we are queuing tasks, typically before Glean has been
                // initialized, then we should just add the created Task to the
                // taskQueue.
                AddActionToQueue(action);
            }
            else
            {
                if (!TestingMode)
                {
                    // If we are not queuing initial tasks, we can go ahead and
                    // execute the task asynchronously on the ExclusiveScheduler                    
                    task = Task.Factory.StartNew(
                        () =>
                        {
                            // In order to prevent tasks from causing exceptions
                            // we wrap the action invocation in try/catch
                            try
                            {
                                action.Invoke();
                            }
                            catch (Exception)
                            {
                                //TODO Exception eaten by Glean
                            }
                        },
                        cancellationToken,
                        TaskCreationOptions.None,
                        taskSchedulerPair.ExclusiveScheduler);
                }
                else
                {
                    // If we are in testing mode, then go ahead and invoke the
                    // action to ensure synchronous execution.
                    action.Invoke();
                }
            }

            return task;
        }

        /// <summary>
        /// This function launches an Action as an asynchronous Task on a
        /// concurrently executed queue.
        ///
        /// **Note:** Tasks that need to be executed asynchronously but should
        /// not block other tasks such as recording data should use the
        /// `LaunchConcurrent` function. For example, tasks performed during
        /// initialization or an upload task could be executed concurrently.
        /// For tasks that need to be executed serially, see `LaunchAPI`.
        ///
        /// This function specifically ignores the `QueueInitialTasks` flag
        /// because the only tasks that should be launched by this are the ping
        /// upload schedulers and those should run regardless of the initialized
        /// state.
        ///
        /// If `TestingMode` is enabled, then `LaunchConcurrent` will just
        /// execute the task rather than adding it to the concurrent queue to
        /// avoid asynchronous issues while testing.
        /// </summary>
        /// <param name="action">The Action to perform</param>
        /// <returns>A Task or null if run synchronously</returns>
        internal static Task LaunchConcurrent(Action action)
        {
            Task task = null;

            if (TestingMode)
            {
                action.Invoke();
            }
            else
            {
                task = Task.Factory.StartNew(
                    () =>
                    {
                        // In order to prevent tasks from causing exceptions
                        // we wrap the action invocation in try/catch
                        try
                        {
                            action.Invoke();
                        }
                        catch (Exception)
                        {
                            //TODO Exception eaten by Glean
                        }
                    },
                    cancellationToken,
                    TaskCreationOptions.None,
                    taskSchedulerPair.ConcurrentScheduler);
            }

            return task;
        }

        /// <summary>
        /// Cancels any pending background tasks.
        /// </summary>
        internal static void CancelBackgroundTasks()
        {
            // Calling Complete on the taskSchedulerPair prevents it from
            // accepting any new tasks.
            taskSchedulerPair.Complete();

            // Cancel tasks associated with the tokenSource
            tokenSource.Cancel();

            // Clear the taskQueue
            taskQueue = new ConcurrentQueue<Action>();
        }

        /// <summary>
        /// Stops queueing tasks and processes any tasks in the queue. Since
        /// QueueInitialTasks is set to false prior to processing the queue, and
        /// this function launches tasks from the ExclusiveScheduler back onto
        /// the ExclusiveScheduler, the tasks should execute in order before any
        /// new tasks are executed.
        /// </summary>
        internal static void FlushQueuedInitialTasks()
        {
            // Set the flag first to guarantee new tasks are executed after
            // this one.
            QueueInitialTasks = false;

            var task = Task.Factory.StartNew(() =>
            {
                // Add the tasks to the ExclusiveScheduler and execute them
                // synchronously
                while (taskQueue.TryDequeue(out Action localValue))
                {
                    localValue.Invoke();
                }
            });

            // Wait for the initial tasks to execute
            task.Wait(QueueProcessingTimeout);

            if (OverflowCount > 0)
            {
                //TODO GleanError.preinitTasksOverflow
                //         .addSync(MaxQueueSize + OverflowCount)
            }
        }

        /// <summary>
        /// Helper function to add task to queue and monitor the MaxQueueSize
        /// </summary>
        /// <param name="task">The Task to add to the queue</param>
        [MethodImpl(MethodImplOptions.Synchronized)]
        private static void AddActionToQueue(Action action)
        {
            if (taskQueue.Count >= MaxQueueSize)
            {
                //TODO Log.e(LOG_TAG, "Exceeded maximum queue size, discarding task")

                // This value ends up in the `preinit_tasks_overflow` metric,
                // but we can't record directly there, because that would only
                // add the recording to an already-overflowing task queue and
                // would be silently dropped.
                OverflowCount += 1;
                return;
            }

            if (TestingMode)
            {
                //TODO Log.i(LOG_TAG, "Task queued for execution in test mode")
            }
            else
            {
                //TODO Log.i(LOG_TAG, "Task queued for execution and delayed until flushed")
            }

            taskQueue.Enqueue(action);
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
