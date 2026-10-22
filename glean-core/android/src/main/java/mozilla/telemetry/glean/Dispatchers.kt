/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.util.Log
import androidx.annotation.VisibleForTesting
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.SupervisorJob
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.channels.ClosedSendChannelException
import kotlinx.coroutines.launch
import kotlinx.coroutines.newSingleThreadContext
import kotlinx.coroutines.runBlocking

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

    class DelayedTaskQueue(
        private val coroutineScope: CoroutineScope,
    ) {
        internal val channel = Channel<Int>()

        // When true, jobs will be run synchronously
        internal var testingMode = false

        init {
            // We put a first task on it that waits to receive something.
            // We close that channel in `flushQueuedInitialTasks` which will unblock this task.
            //
            // Receiving/Closing the channel is the signal the task is unblocked
            @Suppress("SwallowedException")
            this.executeTask {
                try {
                    runBlocking { channel.receive() }
                } catch (e: ClosedSendChannelException) {
                    // intentionally left empty.
                    // The channel is closed by `flushQueuedInitialTasks`
                }
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
         * Launch a block of work asynchronously.
         *
         * If the queue is still blocked, this will run at a later point.
         */
        fun launch(block: () -> Unit) {
            coroutineScope.launch {
                block()
            }

            if (this.testingMode) {
                this.launchBlocking { }
            }
        }

        /**
         * Launch a block of work, wait and return its result
         */
        fun <T> launchBlocking(block: () -> T): T {
            val channel = Channel<T>()
            coroutineScope.launch {
                runBlocking {
                    channel.send(block())
                }
            }

            return runBlocking { channel.receive() }
        }

        /**
         * Stops queueing tasks and processes any tasks in the queue.
         *
         * Processing happens on the current thread synchronously.
         *
         * Since [queueInitialTasks] is set to false prior to processing the queue,
         * newly launched tasks should be executed immediately rather than added to the queue.
         */
        @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class)
        fun flushQueuedInitialTasks() {
            if (!this.channel.isClosedForSend) {
                runBlocking {
                    this@DelayedTaskQueue.channel.send(1)
                }
                this.channel.close()
            }
        }

        internal fun executeTask(block: suspend CoroutineScope.() -> Unit): Job? = coroutineScope.launch(block = block)
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
    var Delayed = DelayedTaskQueue(
        CoroutineScope(
            newSingleThreadContext("GleanMetricPool") + supervisorJob,
        ),
    )
}
