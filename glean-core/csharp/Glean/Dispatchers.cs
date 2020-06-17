// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Collections.Concurrent;
using System.Collections.Generic;
using System.Diagnostics;
using System.Runtime.CompilerServices;
using System.Threading;
using System.Threading.Tasks;

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
        private const int QueueProcessingTimeout = 5000;

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

            if (QueueInitialTasks)
            {
                // If we are queuing, typically before Glean has been
                // initialized, then we should just add the action to
                // the queue.
                AddActionToQueue(action);
            }
            else
            {
                if (!TestingMode)
                {
                    // If we are not queuing we can go ahead and execute the
                    // task asynchronously                    
                    task = factory.StartNew(() =>
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
                    });
                }
                else
                {
                    // If we are in testing mode, then go ahead and await
                    // the task to ensure synchronous execution.
                    factory.StartNew(action).Wait();
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
                // This is not invoked on the apiScheduler to ensure that it
                // gets executed now, ahead of anything on the queue. The task
                // is returned so that it can be awaited if needed.
                return Task.Factory.StartNew(action);
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
                //TODO Log.e(LOG_TAG, "Exceeded maximum queue size, discarding task")

                // This value ends up in the `preinit_tasks_overflow` metric,
                // but we can't record directly there, because that would only
                // add the recording to an already-overflowing task queue and
                // would be silently dropped.
                overflowCount += 1;
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

        // Provides a task scheduler that ensures a maximum concurrency level while
        // running on top of the thread pool.  Copied from example provided here:
        // https://docs.microsoft.com/en-us/dotnet/api/system.threading.tasks.taskscheduler?view=netcore-3.1
        private class LimitedConcurrencyLevelTaskScheduler : TaskScheduler
        {
            // Indicates whether the current thread is processing work items.
            [ThreadStatic]
            private static bool _currentThreadIsProcessingItems;

            // The list of tasks to be executed
            // protected by lock(_tasks)
            private readonly LinkedList<Task> _tasks = new LinkedList<Task>(); 

            // The maximum concurrency level allowed by this scheduler.
            private readonly int _maxDegreeOfParallelism;

            // Indicates whether the scheduler is currently processing work items.
            private int _delegatesQueuedOrRunning = 0;

            // Creates a new instance with the specified degree of parallelism.
            public LimitedConcurrencyLevelTaskScheduler(int maxDegreeOfParallelism)
            {
                if (maxDegreeOfParallelism < 1)
                {
                    throw new ArgumentOutOfRangeException("maxDegreeOfParallelism");
                }
                _maxDegreeOfParallelism = maxDegreeOfParallelism;
            }

            // Queues a task to the scheduler.
            protected sealed override void QueueTask(Task task)
            {
                // Add the task to the list of tasks to be processed.  If there aren't enough
                // delegates currently queued or running to process tasks, schedule another.
                lock (_tasks)
                {
                    _tasks.AddLast(task);
                    if (_delegatesQueuedOrRunning < _maxDegreeOfParallelism)
                    {
                        ++_delegatesQueuedOrRunning;
                        NotifyThreadPoolOfPendingWork();
                    }
                }
            }

            // Inform the ThreadPool that there's work to be executed for this scheduler.
            private void NotifyThreadPoolOfPendingWork()
            {
                ThreadPool.UnsafeQueueUserWorkItem(_ =>
                {
                    // Note that the current thread is now processing work items.
                    // This is necessary to enable inlining of tasks into this thread.
                    _currentThreadIsProcessingItems = true;
                    try
                    {
                        // Process all available items in the queue.
                        while (true)
                        {
                            Task item;
                            lock (_tasks)
                            {
                                // When there are no more items to be processed,
                                // note that we're done processing, and get out.
                                if (_tasks.Count == 0)
                                {
                                    --_delegatesQueuedOrRunning;
                                    break;
                                }

                                // Get the next item from the queue
                                item = _tasks.First.Value;
                                _tasks.RemoveFirst();
                            }

                            // Execute the task we pulled out of the queue
                            base.TryExecuteTask(item);
                        }
                    }
                    // We're done processing items on the current thread
                    finally { _currentThreadIsProcessingItems = false; }
                }, null);
            }

            // Attempts to execute the specified task on the current thread.
            protected sealed override bool TryExecuteTaskInline(Task task,
                bool taskWasPreviouslyQueued)
            {
                // If this thread isn't already processing a task, we don't support inlining
                if (!_currentThreadIsProcessingItems) return false;

                // If the task was previously queued, remove it from the queue
                if (taskWasPreviouslyQueued)
                    // Try to run the task.
                    if (TryDequeue(task))
                        return base.TryExecuteTask(task);
                    else
                        return false;
                else
                    return base.TryExecuteTask(task);
            }

            // Attempt to remove a previously scheduled task from the scheduler.
            protected sealed override bool TryDequeue(Task task)
            {
                lock (_tasks) return _tasks.Remove(task);
            }

            // Gets the maximum concurrency level supported by this scheduler.
            public sealed override int MaximumConcurrencyLevel
            {
                get { return _maxDegreeOfParallelism; }
            }

            // Gets an enumerable of the tasks currently scheduled on this scheduler.
            protected sealed override IEnumerable<Task> GetScheduledTasks()
            {
                bool lockTaken = false;
                try
                {
                    Monitor.TryEnter(_tasks, ref lockTaken);
                    if (lockTaken) return _tasks;
                    else throw new NotSupportedException();
                }
                finally
                {
                    if (lockTaken) Monitor.Exit(_tasks);
                }
            }
        }
    }
}
