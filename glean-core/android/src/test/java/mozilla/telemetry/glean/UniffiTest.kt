/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import android.content.Context
import androidx.lifecycle.Lifecycle
import androidx.lifecycle.LifecycleOwner
import androidx.lifecycle.LifecycleRegistry
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import kotlinx.coroutines.Dispatchers as KotlinDispatchers
import kotlinx.coroutines.launch
import kotlinx.coroutines.ObsoleteCoroutinesApi
import kotlinx.coroutines.runBlocking
import mozilla.telemetry.glean.GleanMetrics.GleanError
import mozilla.telemetry.glean.GleanMetrics.GleanInternalMetrics
import mozilla.telemetry.glean.GleanMetrics.Pings
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.internal.CounterMetric
import mozilla.telemetry.glean.internal.Lifetime
import mozilla.telemetry.glean.internal.CommonMetricData
import mozilla.telemetry.glean.rust.LibGleanFFI
import mozilla.telemetry.glean.rust.toBoolean
import mozilla.telemetry.glean.rust.toByte
import mozilla.telemetry.glean.scheduler.GleanLifecycleObserver
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.testing.GleanTestRule
import mozilla.telemetry.glean.utils.getLanguageFromLocale
import mozilla.telemetry.glean.utils.getLocaleTag
import org.json.JSONObject
import org.junit.After
import org.junit.Ignore
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Assert.assertFalse
import org.junit.Rule
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.Mockito.mock
import org.mockito.Mockito.spy
import org.robolectric.shadows.ShadowProcess
import org.robolectric.shadows.ShadowLog
import java.io.File
import java.util.Calendar
import java.util.Locale
import java.util.concurrent.TimeUnit

@ObsoleteCoroutinesApi
@RunWith(AndroidJUnit4::class)
class UniffiTest {
    private val context: Context
        get() = ApplicationProvider.getApplicationContext()

    //@get:Rule
    //val gleanRule = GleanTestRule(context)

    //@After
    //fun resetGlobalState() {
    //    Glean.setUploadEnabled(true)
    //}

    @Test
    fun smoke() {
        Glean.initializeInternal(context, uploadEnabled = true, Configuration(), null)

        val counterMetric = CounterMetric(CommonMetricData(
            disabled = false,
            category = "telemetry",
            lifetime = Lifetime.APPLICATION,
            name = "counter_metric",
            sendInPings = listOf("store1"),
            dynamicLabel = null
        ))

        counterMetric.add()

        assertEquals(1, counterMetric.testGetValue())
    }
}
