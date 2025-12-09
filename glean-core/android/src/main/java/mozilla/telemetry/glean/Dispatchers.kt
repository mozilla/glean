/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import androidx.annotation.VisibleForTesting
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.launch
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.runBlocking
import java.util.concurrent.ConcurrentLinkedQueue
import java.util.concurrent.atomic.AtomicBoolean

internal object Dispatchers {
    class WaitableCoroutineScope(
        private val coroutineScope: CoroutineScope,
    ) {
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
        internal fun executeTask(block: suspend CoroutineScope.() -> Unit): Job? =
            when {
                testingMode -> {
                    runBlocking {
                        block()
                    }
                    null
                }

                else -> {
                    coroutineScope.launch(block = block)
                }
            }
    }

    class DelayedTaskQueue {
        // When true, jobs will be queued and not ran until triggered by calling
        // flushQueuedInitialTasks()
        private var queueInitialTasks = AtomicBoolean(true)

        // Use a [ConcurrentLinkedQueue] to take advantage of its thread safety and no locking
        internal val taskQueue: ConcurrentLinkedQueue<() -> Unit> = ConcurrentLinkedQueue()

        /**
         * Launch a block of work synchronously or queue it if not yet unblocked.
         *
         * If [queueInitialTasks] true
         * then the work will be queued and executed when [flushQueuedInitialTasks] is called.
         * If [queueInitialTasks] is false the block is executed synchronously.
         */
        fun launch(block: () -> Unit) {
            val queueTasks = synchronized(this) {
                queueInitialTasks.get()
            }

            if (queueTasks) {
                addTaskToQueue(block)
            } else {
                block()
            }
        }

        /**
         * Stops queueing tasks and processes any tasks in the queue.
         *
         * Processing happens on the current thread synchronously.
         *
         * Since [queueInitialTasks] is set to false prior to processing the queue,
         * newly launched tasks should be executed immediately rather than added to the queue.
         */
        internal fun flushQueuedInitialTasks() {
            val dispatcherObject = this

            val queueCopy: ConcurrentLinkedQueue<() -> Unit> = ConcurrentLinkedQueue()
            synchronized(dispatcherObject) {
                queueCopy.addAll(taskQueue)
                taskQueue.clear()

                queueCopy.forEach { task ->
                    task()
                }

                queueInitialTasks.set(false)
            }
        }

        /**
         * Helper function to add task to queue.
         */
        @Synchronized
        private fun addTaskToQueue(block: () -> Unit) {
            taskQueue.add(block)
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
    @Suppress("ktlint:standard:property-naming")
    var API = WaitableCoroutineScope(
        CoroutineScope(
            newSingleThreadContext("GleanAPIPool") + supervisorJob,
        ),
    )

    @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class, kotlinx.coroutines.ExperimentalCoroutinesApi::class)
    @Suppress("ktlint:standard:property-naming")
    var Delayed = DelayedTaskQueue()
}
