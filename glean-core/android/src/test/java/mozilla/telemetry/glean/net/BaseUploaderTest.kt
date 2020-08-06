/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import mozilla.telemetry.glean.any
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.eq
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.spy
import org.mockito.Mockito.verify
import org.robolectric.RobolectricTestRunner

@RunWith(RobolectricTestRunner::class)
class BaseUploaderTest {
    private val testPath: String = "/some/random/path/not/important"
    private val testPing: String = "{ 'ping': 'test' }"
    private val testHeaders: HeadersList = mutableListOf(Pair("X-Test-Glean", "nothing-to-see-here"))
    private val testDefaultConfig = Configuration()

    /**
     * A stub uploader class that does not upload anything.
     */
    private class TestUploader : PingUploader {
        override fun upload(url: String, data: ByteArray, headers: HeadersList): UploadResult {
            return UnrecoverableFailure
        }
    }

    @Test
    fun `upload() must get called with the full submission path`() {
        val uploader = spy<BaseUploader>(BaseUploader(TestUploader()))

        val expectedUrl = testDefaultConfig.serverEndpoint + testPath
        uploader.doUpload(testPath, testPing.toByteArray(Charsets.UTF_8), testHeaders, testDefaultConfig)
        verify(uploader).upload(eq(expectedUrl), any(), any())
    }
}
