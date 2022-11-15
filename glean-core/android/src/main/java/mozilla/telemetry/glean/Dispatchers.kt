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

    // This job is used to make sure the API `CoroutineContext` does not cancel
    // children jobs when exceptions are thrown in children coroutines.
    private val supervisorJob = SupervisorJob()

    /**
     * A coroutine scope to make it easy to dispatch API calls off the main thread.
     * This needs to be a `var` so that our tests can override this.
     */
    @OptIn(kotlinx.coroutines.DelicateCoroutinesApi::class)
    var API = WaitableCoroutineScope(
        CoroutineScope(
            newSingleThreadContext("GleanAPIPool") + supervisorJob
        )
    )
}
