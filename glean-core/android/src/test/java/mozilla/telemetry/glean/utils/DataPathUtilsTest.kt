/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package mozilla.telemetry.glean.utils

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DataPathUtilsTest {
    private val mockDataDir = "mock_dataDir_path"

    @Test
    fun `test can write to database path invalid path`() {
        val customDataPath = " invalid db path "
        assertFalse(canWriteToDatabasePath(mockDataDir, customDataPath))
    }

    fun `test can write to database path valid path`() {
        val customDataPath = "valid_db_path"
        assertTrue(canWriteToDatabasePath(mockDataDir, customDataPath))
    }

    fun `test generate glean storage path has correct slug`() {
        val customDataPath = "test_glean_data"
        val path = generateGleanStoragePath(mockDataDir, customDataPath)
        assertEquals(path.absolutePath.substringAfterLast("/"), customDataPath)
    }
}
