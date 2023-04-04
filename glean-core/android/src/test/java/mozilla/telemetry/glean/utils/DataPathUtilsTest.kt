/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package mozilla.telemetry.glean.utils

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DataPathUtilsTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    @Test
    fun `Cannot write to invalid database path`() {
        val customDataPath = ""
        assertFalse(canWriteToDatabasePath(customDataPath))
    }

    @Test
    fun `Can write to valid database path`() {
        val dataPath = context.applicationInfo.dataDir + "/valid_db_path"
        assertTrue(canWriteToDatabasePath(dataPath))
    }
}
