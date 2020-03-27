/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.testing

import android.content.Context
import androidx.annotation.VisibleForTesting
import androidx.work.Configuration
import androidx.work.testing.WorkManagerTestInitHelper
import mozilla.telemetry.glean.Glean
import org.junit.rules.TestWatcher
import org.junit.runner.Description
import java.util.concurrent.Executors

/**
 * This implements a JUnit rule for writing tests for Glean SDK metrics.
 *
 * The rule takes care of sending Glean SDK pings to a local server, at the
 * address: "http://localhost:<port>".
 *
 * This is useful for Android instrumented tests, where we don't want to
 * initialize Glean more than once but still want to send pings to a local
 * server for validation.
 *
 * Example usage:
 *
 * ```
 * // Add the following lines to you test class.
 * @get:Rule
 * val gleanRule = GleanTestLocalServer(3785)
 * ```
 *
 * @property context the application context
 * @param localPort the port of the local ping server
 */
@VisibleForTesting(otherwise = VisibleForTesting.NONE)
class GleanTestLocalServer(
    val context: Context,
    private val localPort: Int
) : TestWatcher() {
    /**
     * Invoked when a test is about to start.
     */
    override fun starting(description: Description?) {
        Glean.testSetLocalEndpoint(localPort)

        val config = Configuration.Builder()
            // Use a single thread executor rather than the default test
            // executor which runs on the main thread as we cannot make background
            // upload tasks run on that thread. Otherwise the application will crash
            // with a "networking on the main thread" exception.
            .setExecutor(Executors.newSingleThreadExecutor())
            .build()

        // Initialize WorkManager for instrumentation tests.
        WorkManagerTestInitHelper.initializeTestWorkManager(context, config)
    }
}
