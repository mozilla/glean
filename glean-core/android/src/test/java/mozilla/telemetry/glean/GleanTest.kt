/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.Before
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.io.File

@RunWith(RobolectricTestRunner::class)
class GleanTest {

    @Before
    fun setUp() {
        resetGlean()
    }

    @Test
    fun `send a ping`() {
        val context: Context = ApplicationProvider.getApplicationContext()
        val pingPath = File(context.applicationInfo.dataDir, "glean_data/pings")

        Glean.handleBackgroundEvent()
        // Only the baseline ping should have been written
        assertEquals(1, pingPath.listFiles()?.size)
    }
}
