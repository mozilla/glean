/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.test.core.app.ApplicationProvider
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import org.robolectric.RobolectricTestRunner
import java.io.File

@RunWith(RobolectricTestRunner::class)
class GleanTest {
    @Test
    fun `simple smoke test`() {
        GleanInternalAPI().initialize(ApplicationProvider.getApplicationContext())
    }

    @Test
    fun `send a ping`() {
        val context: Context = ApplicationProvider.getApplicationContext()
        val glean = GleanInternalAPI()
        glean.initialize(context)
        glean.handleBackgroundEvent()
        val pingPath = File(context.applicationInfo.dataDir, "glean_data/pings")
        val path = pingPath.toString()
        assertEquals(2, pingPath.listFiles()?.size)
    }
}
