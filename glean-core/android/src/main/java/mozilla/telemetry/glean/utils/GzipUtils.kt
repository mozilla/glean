/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import java.io.BufferedReader
import java.io.ByteArrayInputStream
import java.util.zip.GZIPInputStream

/**
 * Decompress the GZIP returned by the glean-core layer.
 *
 * @param data the gzipped [ByteArray] to decompress
 * @return a [String] containing the uncompressed data.
 */
fun decompressGZIP(data: ByteArray): String {
    return GZIPInputStream(ByteArrayInputStream(data)).bufferedReader().use(BufferedReader::readText)
}
