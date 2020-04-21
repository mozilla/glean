/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.testing

import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.Glean
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.utils.AndroidBuildInfo
import org.junit.rules.RuleChain
import org.junit.rules.TemporaryFolder
import org.junit.rules.TestRule
import org.junit.rules.TestWatcher
import org.junit.runner.Description
import org.junit.runners.model.Statement

/**
 * This implements a JUnit rule for writing tests for Glean SDK metrics.
 *
 * The rule takes care of resetting the Glean SDK between tests and
 * initializing all the required dependencies.
 *
 * Example usage:
 *
 * ```
 * // Add the following lines to you test class.
 * @get:Rule
 * val gleanRule = GleanTestRule(ApplicationProvider.getApplicationContext())
 * ```
 *
 * @param configToUse an optional [Configuration] to initialize the Glean SDK with
 */
@VisibleForTesting(otherwise = VisibleForTesting.NONE)
class GleanTestNoUploads(
    private val configToUse: Configuration = Configuration()
) : TestRule {
    private val temporaryFolder = TemporaryFolder()

    override fun apply(base: Statement, description: Description): Statement {
        return RuleChain
            .outerRule(temporaryFolder)
            .around(object : TestWatcher(){
                /**
                 * Invoked when a test is about to start.
                 */
                override fun starting(description: Description?) {
                    Glean.resetGlean(
                        context = StubAndroidContext(
                            temporaryFolder.newFolder().absolutePath,
                            "org.mozilla.gleantestnoupload"
                        ),
                        config = configToUse,
                        clearStores = true,
                        buildInfo = object : AndroidBuildInfo {
                            override fun getSdkVersion(): String = "10"
                            override fun getVersionString(): String = "glean-test-1.0"
                            override fun getDeviceManufacturer(): String = "glean stubs, inc"
                            override fun getDeviceModel(): String = "stub-dev-mk1"
                            override fun getPreferredABI(): String = "gleanabi-v1"
                        }
                    )
                }
            })
            .apply(base, description)
    }
}
