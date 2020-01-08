/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.net

import com.google.common.base.Strings
import mozilla.telemetry.glean.BuildConfig
import mozilla.telemetry.glean.config.Configuration
import mozilla.components.support.test.any
import mozilla.components.support.test.argumentCaptor
import mozilla.components.support.test.eq
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.`when`
import org.mockito.Mockito.spy
import org.mockito.Mockito.verify
import org.robolectric.RobolectricTestRunner
import java.util.Calendar
import java.util.TimeZone

@RunWith(RobolectricTestRunner::class)
class BaseUploaderTest {
    private val testPath: String = "/some/random/path/not/important"
    private val testPing: String = "{ 'ping': 'test' }"
    private val testDefaultConfig = Configuration().copy(
        userAgent = "Glean/Test 25.0.2"
    )

    /**
     * A stub uploader class that does not upload anything.
     */
    private class TestUploader : PingUploader {
        override fun upload(url: String, data: String, headers: HeadersList): Boolean {
            return false
        }
    }

    @Test
    fun `upload() must get called with the full submission path`() {
        val uploader = spy<BaseUploader>(BaseUploader(TestUploader()))

        val expectedUrl = testDefaultConfig.serverEndpoint + testPath
        uploader.doUpload(testPath, testPing, testDefaultConfig)
        verify(uploader).upload(eq(expectedUrl), any(), any())
    }

    @Test
    fun `All headers are correctly reported for upload`() {
        val uploader = spy<BaseUploader>(BaseUploader(TestUploader()))
        `when`(uploader.getCalendarInstance()).thenAnswer {
            val fakeNow = Calendar.getInstance()
            fakeNow.clear()
            fakeNow.timeZone = TimeZone.getTimeZone("GMT")
            fakeNow.set(2015, 6, 11, 11, 0, 0)
            fakeNow
        }

        uploader.doUpload(testPath, testPing, testDefaultConfig)
        val headersCaptor = argumentCaptor<HeadersList>()

        val expectedUrl = testDefaultConfig.serverEndpoint + testPath
        verify(uploader).upload(eq(expectedUrl), eq(testPing), headersCaptor.capture())

        val expectedHeaders = mapOf(
            "Content-Type" to "application/json; charset=utf-8",
            "Date" to "Sat, 11 Jul 2015 11:00:00 GMT",
            "User-Agent" to "Glean/Test 25.0.2",
            "X-Client-Type" to "Glean",
            "X-Client-Version" to BuildConfig.LIBRARY_VERSION
        )

        expectedHeaders.forEach { (headerName, headerValue) ->
            assertEquals(
                headerValue,
                headersCaptor.value.find { it.first == headerName }!!.second
            )
        }
    }

    @Test
    fun `X-Debug-ID header is correctly added when pingTag is not null`() {
        val uploader = spy<BaseUploader>(BaseUploader(TestUploader()))

        val debugConfig = testDefaultConfig.copy(
            pingTag = "this-ping-is-tagged"
        )

        uploader.doUpload(testPath, testPing, debugConfig)
        val headersCaptor = argumentCaptor<HeadersList>()
        verify(uploader).upload(any(), any(), headersCaptor.capture())

        assertEquals(
            "this-ping-is-tagged",
            headersCaptor.value.find { it.first == "X-Debug-ID" }!!.second
        )
    }

    @Test
    fun `splitPingForLog() correctly divides large pings`() {
        // This makes 10 full chunks plus a chunk with 10 bytes (1 message) to test breaking out
        // a smaller last chunk
        val testString = "*TestData*"
        val testData = Strings.repeat(testString, BaseUploader.MAX_LOG_PAYLOAD_SIZE_BYTES + 1)
        val testPath = "test/Glean"
        val testChunk = Strings.repeat(
            testString,
            BaseUploader.MAX_LOG_PAYLOAD_SIZE_BYTES / testString.length
        )

        val chunks = BaseUploader.splitPingForLog(testData, testPath)

        assertEquals("Must have correct number of chunks", 11, chunks.size)
        var curChunk = 0
        for (chunk in chunks) {
            val calculatedHeaderMsg =
                "Glean ping to URL: $testPath [Part ${curChunk + 1} of ${chunks.size}]\n"
            // Need to re-add the /n here since `lines()` strips it
            val actualHeaderMsg = "${chunk.lines()[0]}\n"
            assertEquals(
                "First line must contain correct header message",
                calculatedHeaderMsg,
                actualHeaderMsg
            )

            if (chunk === chunks.last()) {
                assertEquals(
                    "Must have correct size of chunk",
                    testString.length + calculatedHeaderMsg.length,
                    chunk.length
                )
                assertEquals(
                    "Must have correct content",
                    "$calculatedHeaderMsg$testString",
                    chunk
                )
            } else {
                assertEquals(
                    "Must have correct size of chunk",
                    BaseUploader.MAX_LOG_PAYLOAD_SIZE_BYTES + calculatedHeaderMsg.length,
                    chunk.length
                )
                assertEquals(
                    "Must have correct content",
                    "$calculatedHeaderMsg$testChunk",
                    chunk
                )
            }

            curChunk += 1
        }
    }
}
