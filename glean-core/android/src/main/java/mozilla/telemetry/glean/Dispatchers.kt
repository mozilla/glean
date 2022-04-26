/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.util.Log
import androidx.annotation.VisibleForTesting
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.ObsoleteCoroutinesApi
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.runBlocking
import mozilla.telemetry.glean.GleanMetrics.GleanError
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.atomic.AtomicBoolean

@ObsoleteCoroutinesApi
internal object Dispatchers {
    class WaitableCoroutineScope(private val coroutineScope: CoroutineScope) {
        // When true, jobs will be run synchronously
        internal var testingMode = false

        // When true, jobs will be queued and not ran until triggered by calling
        // flushQueuedInitialTasks()
        private var queueInitialTasks = AtomicBoolean(true)

        // Use a [ConcurrentLinkedQueue] to take advantage of it's thread safety and no locking
        internal val taskQueue: ConcurrentLinkedQueue<suspend CoroutineScope.() -> Unit> = ConcurrentLinkedQueue()

        companion object {
            private const val LOG_TAG = "glean/Dispatchers"

            // This value was chosen in order to allow several tasks to be queued for execution but
            // still be conservative of memory. This queue size is important for cases where
            // setUploadEnabled(false) is not called so that we don't continue to queue tasks and
            // waste memory.
            const val MAX_QUEUE_SIZE = 100
        }

        // The number of items that were added to the queue after it filled up.
        internal var overflowCount = 0

        /**
         * Launch a block of work asynchronously.
         *
         * * If [queueInitialTasks] is true, then the work will be queued and executed when
         * [flushQueuedInitialTasks] is called.
         *
         * If [setTestingMode] has enabled testing mode, the work will run
         * synchronously.
         *
         * @return [Job], or null if queued or run synchronously.
         */
        fun launch(
            block: suspend CoroutineScope.() -> Unit
        ): Job? {
            return if (queueInitialTasks.get()) {
                addTaskToQueue(block)
                null
            } else {
                executeTask(block)
            }
        }

        /**
         * Helper function to ensure the Glean SDK is being used in testing
         * mode and async jobs are being run synchronously.  This should be
         * called from every method in the testing API to make sure that the
         * results of the main API can be tested as expected.
         */
        @VisibleForTesting(otherwise = VisibleForTesting.NONE)
        fun assertInTestingMode() {
            assert(
                testingMode
            ) {
                "To use the testing API, apply the GleanTestRule to set up a disposable Glean " +
                "instance. e.g. GleanTestRule(ApplicationProvider.getApplicationContext())"
            }
        }

        /**
         * Enable testing mode, which makes all of the Glean SDK public API
         * synchronous.
         *
         * @param enabled whether or not to enable the testing mode
         */
        @VisibleForTesting(otherwise = VisibleForTesting.NONE)
        fun setTestingMode(enabled: Boolean) {
            testingMode = enabled
        }

        /**
         * Enable queueing mode, which causes tasks to be queued until launched by calling
         * [flushQueuedInitialTasks].
         *
         * @param enabled whether or not to queue tasks
         */
        @VisibleForTesting(otherwise = VisibleForTesting.NONE)
        fun setTaskQueueing(enabled: Boolean) {
            queueInitialTasks.set(enabled)
        }

        /**
         * Stops queueing tasks and processes any tasks in the queue. Since [queueInitialTasks] is
         * set to false prior to processing the queue, newly launched tasks should be executed
         * on the couroutine scope rather than added to the queue.
         */
        internal suspend fun flushQueuedInitialTasks() {
            val dispatcherObject = this
            // Dispatch a task to flush the pre-init tasks queue. By using `executeTask`
            // this will be executed as soon as possible, before other tasks are executed.
            // In test mode, this will make sure to execute things on the caller thread.
            executeTask {
                // Set the flag first as the first thing in this job. This will guarantee new jobs
                // are after this one, thus executed in order. The new jobs won't be added to
                // `taskQueue` but, rather, handled by the coroutineScope itself.
                queueInitialTasks.set(false)

                // Nothing should be added to this list. However, the flush could get called
                // while a `addTaskToQueue` is being executed. For this reason, we need synchronized
                // access to the queue. However, we can't simply wrap the task execution in a sync
                // block: suspending functions are not allowed inside critical sections.
                val queueCopy: ConcurrentLinkedQueue<suspend CoroutineScope.() -> Unit> = ConcurrentLinkedQueue()
                synchronized(dispatcherObject) {
                    queueCopy.addAll(taskQueue)
                    taskQueue.clear()
                }

                // Execute the tasks.
                queueCopy.forEach { task ->
                    // Task is a suspending function.
                    task()
                }

                // This must happen after `queueInitialTasks.set(false)` is run, or it
                // would be added to a full task queue and be silently dropped.
                if (overflowCount > 0) {
                    GleanError.preinitTasksOverflow.addSync(MAX_QUEUE_SIZE + overflowCount)
                }
            }?.join()
        }

        /**
         * Helper function to add task to queue as either a synchronous or asynchronous operation,
         * depending on whether [testingMode] is true.
         */
        @Synchronized
        private fun addTaskToQueue(block: suspend CoroutineScope.() -> Unit) {
            if (taskQueue.size >= MAX_QUEUE_SIZE) {
                Log.e(LOG_TAG, "Exceeded maximum queue size, discarding task")

                // This value ends up in the `preinit_tasks_overflow` metric, but we
                // can't record directly there, because that would only add
                // the recording to an already-overflowing task queue and would be
                // silently dropped.
                overflowCount += 1
                return
            }

            if (testingMode) {
                Log.i(LOG_TAG, "Task queued for execution in test mode")
            } else {
                Log.i(LOG_TAG, "Task queued for execution and delayed until flushed")
            }

            taskQueue.add(block)
        }

        /**
         * Helper function to execute the task as either an synchronous or asynchronous operation,
         * depending on whether [testingMode] is true.
         *
         * WARNING: THIS SHOULD ALMOST NEVER BE USED. IF IN DOUBT, USE [launch] INSTEAD.
         *
         * [launch] is useful for running tasks that might be called before initialization, but
         * need to actually run after initialization (which is true for most tasks in Glean).
         * This should only be called directly for tasks that must run immediately after
         * initialization.
         *
         * This has internal visibility only so that it can be called directly to
         * send queued events immediately at startup before any metric recording.
         */
        internal fun executeTask(block: suspend CoroutineScope.() -> Unit): Job? {
            return when {
                testingMode -> {
                    runBlocking {
                        block()
                    }
                    null
                }
                else -> coroutineScope.launch(block = block)
            }
        }
    }

    // This job is used to make sure the API `CoroutineContext` does not cancel
    // children jobs when exceptions are thrown in children coroutines.
    private val supervisorJob = SupervisorJob()

    /**
     * A coroutine scope to make it easy to dispatch API calls off the main thread.
     * This needs to be a `var` so that our tests can override this.
     */
    var API = WaitableCoroutineScope(CoroutineScope(
        newSingleThreadContext("GleanAPIPool") + supervisorJob
    ))
}
