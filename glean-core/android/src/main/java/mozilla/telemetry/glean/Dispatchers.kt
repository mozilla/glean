/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import androidx.annotation.VisibleForTesting
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.runBlocking
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.atomic.AtomicBoolean

internal object Dispatchers {
    class WaitableCoroutineScope(private val coroutineScope: CoroutineScope) {
        // When true, jobs will be run synchronously
        internal var testingMode = false

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
         * Helper function to execute the task as an asynchronous operation.
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

    class BlockedCoroutineScope(private val coroutineScope: CoroutineScope) {
        // When true, jobs will be queued and not ran until triggered by calling
        // flushQueuedInitialTasks()
        private var queueInitialTasks = AtomicBoolean(true)

        // Use a [ConcurrentLinkedQueue] to take advantage of it's thread safety and no locking
        internal val taskQueue: ConcurrentLinkedQueue<suspend CoroutineScope.() -> Unit> = ConcurrentLinkedQueue()

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
            }?.join()
        }

        /**
         * Helper function to add task to queue as either a synchronous or asynchronous operation,
         * depending on whether [testingMode] is true.
         */
        @Synchronized
        private fun addTaskToQueue(block: suspend CoroutineScope.() -> Unit) {
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
            return coroutineScope.launch(block = block)
        }

        internal fun executeBlocking(block: suspend CoroutineScope.() -> Unit): Unit {
            return runBlocking { block() }
        }
    }

    // This job is used to make sure the API `CoroutineContext` does not cancel
    // children jobs when exceptions are thrown in children coroutines.
    private val supervisorJob = SupervisorJob()

    /**
     * A coroutine scope to make it easy to dispatch API calls off the main thread.
     * This needs to be a `var` so that our tests can override this.
     */
    @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class, kotlinx.coroutines.ExperimentalCoroutinesApi::class)
    var API = WaitableCoroutineScope(
        CoroutineScope(
            newSingleThreadContext("GleanAPIPool") + supervisorJob,
        ),
    )

    @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class, kotlinx.coroutines.ExperimentalCoroutinesApi::class)
    var Queue = BlockedCoroutineScope(
        CoroutineScope(
            newSingleThreadContext("GleanAPIPool") + supervisorJob
        )
    )
}
