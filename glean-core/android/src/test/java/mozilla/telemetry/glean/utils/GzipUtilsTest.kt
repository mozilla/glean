/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package mozilla.telemetry.glean.utils

import org.junit.Test
import org.junit.Assert.assertEquals
import java.io.ByteArrayOutputStream
import java.util.zip.GZIPOutputStream

class GzipUtilsTest {
    private val testPing: String = "{ 'ping': 'test' }"

    @Test
    fun `gzip must be correctly decompressed`() {
        // Compress the test ping.
        val byteOutputStream = ByteArrayOutputStream()
        GZIPOutputStream(byteOutputStream).bufferedWriter(Charsets.UTF_8).use { it.write(testPing) }
        val compressedTestPing = byteOutputStream.toByteArray()

        //  Decompress the result and check if it's valid.
        val decompressedPing = decompressGZIP(compressedTestPing)
        assertEquals(testPing, decompressedPing)
    }
}
