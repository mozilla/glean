/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@file:Suppress("MatchingDeclarationName")

package mozilla.telemetry.glean

import android.content.Context
import android.os.SystemClock
import android.content.pm.PackageInfo
import android.content.pm.PackageManager
import androidx.test.core.app.ApplicationProvider
import androidx.work.WorkInfo
import androidx.work.WorkManager
import androidx.work.testing.WorkManagerTestInitHelper
import kotlinx.coroutines.isActive
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeout
import org.json.JSONObject
import org.mockito.ArgumentMatchers
import org.mockito.Mockito
import mozilla.telemetry.glean.config.Configuration
import mozilla.telemetry.glean.scheduler.PingUploadWorker
import mozilla.telemetry.glean.private.PingTypeBase
import mozilla.telemetry.glean.private.TimeUnit
import mozilla.telemetry.glean.utils.decompressGZIP
import mozilla.telemetry.glean.utils.getISOTimeString
import mozilla.telemetry.glean.scheduler.MetricsPingScheduler
import okhttp3.mockwebserver.Dispatcher
import okhttp3.mockwebserver.MockResponse
import okhttp3.mockwebserver.MockWebServer
import okhttp3.mockwebserver.RecordedRequest
import org.junit.Assert
import org.mockito.ArgumentCaptor
import org.mockito.Mockito.spy
import org.robolectric.shadows.ShadowLog
import java.io.ByteArrayInputStream
import java.util.Calendar
import java.util.UUID
import java.util.concurrent.ExecutionException

/**
 * Checks ping content against the Glean ping schema.
 *
 * This uses the Python utility, glean_parser, to perform the actual checking.
 * This is installed in its own Miniconda environment as part of the build
 * configuration by glean-gradle-plugin.
 *
 * @param content The JSON content of the ping
 * @throws AssertionError If the JSON content is not valid
 */
internal fun checkPingSchema(content: JSONObject) {
    val os = System.getProperty("os.name")?.toLowerCase()
    val pythonExecutable =
        if (os?.indexOf("win")?.compareTo(0) == 0)
            "${BuildConfig.GLEAN_MINICONDA_DIR}/python"
        else
            "${BuildConfig.GLEAN_MINICONDA_DIR}/bin/python"

    val proc = ProcessBuilder(
        listOf(
            pythonExecutable,
            "-m",
            "glean_parser",
            "check",
            "-s",
            "${BuildConfig.GLEAN_PING_SCHEMA_PATH}"
        )
    ).redirectOutput(ProcessBuilder.Redirect.INHERIT)
        .redirectError(ProcessBuilder.Redirect.INHERIT)
    val process = proc.start()

    val jsonString = content.toString()
    with(process.outputStream.bufferedWriter()) {
        write(jsonString)
        newLine()
        flush()
        close()
    }

    val exitCode = process.waitFor()
    Assert.assertEquals("glean_parser check failed with exit code $exitCode", 0, exitCode)
}

/**
 * Checks ping content against the Glean ping schema.
 *
 * This uses the Python utility, glean_parser, to perform the actual checking.
 * This is installed in its own Miniconda environment as part of the build
 * configuration by glean-gradle-plugin.
 *
 * @param content The JSON content of the ping
 * @return the content string, parsed into a JSONObject
 * @throws AssertionError If the JSON content is not valid
 */
internal fun checkPingSchema(content: String): JSONObject {
    val jsonContent = JSONObject(content)
    checkPingSchema(jsonContent)
    return jsonContent
}

/**
 * Collects a specified ping type and checks it against the Glean ping schema.
 *
 * @param ping The ping to check
 * @return the ping contents, in a JSONObject
 * @throws AssertionError If the JSON content is not valid
 */
internal fun collectAndCheckPingSchema(ping: PingTypeBase): JSONObject {
    val jsonString = Glean.testCollect(ping)!!
    return checkPingSchema(jsonString)
}

/**
 * Resets the Glean state and trigger init again.
 *
 * @param context the application context to init Glean with
 * @param config the [Configuration] to init Glean with
 * @param clearStores if true, clear the contents of all stores
 */
internal fun resetGlean(
    context: Context = ApplicationProvider.getApplicationContext(),
    config: Configuration = Configuration(),
    clearStores: Boolean = true,
    redirectRobolectricLogs: Boolean = true,
    uploadEnabled: Boolean = true
) {
    if (redirectRobolectricLogs) {
        ShadowLog.stream = System.out
    }

    // We're using the WorkManager in a bunch of places, and Glean will crash
    // in tests without this line. Let's simply put it here.
    WorkManagerTestInitHelper.initializeTestWorkManager(context)
    // Always log pings for tests
    Glean.setLogPings(true)
    Glean.resetGlean(context, config, clearStores, uploadEnabled = uploadEnabled)
}

/**
 * Get a context that contains [PackageInfo.versionName] mocked to the passed value
 * or "glean.version.name" by default.
 *
 * @param versionName a [String] used as the display version (default: "glean.version.name").
 * @return an application [Context] that can be used to init Glean
 */
internal fun getContextWithMockedInfo(versionName: String = "glean.version.name"): Context {
    val context = Mockito.spy<Context>(ApplicationProvider.getApplicationContext<Context>())
    val packageInfo = Mockito.mock(PackageInfo::class.java)
    packageInfo.versionName = versionName
    val packageManager = Mockito.mock(PackageManager::class.java)
    Mockito.`when`(
        packageManager.getPackageInfo(
            ArgumentMatchers.anyString(),
            ArgumentMatchers.anyInt()
        )
    ).thenReturn(packageInfo)
    Mockito.`when`(context.packageManager).thenReturn(packageManager)
    return context
}

/**
 * Represents the Worker status returned by [getWorkerStatus]
 */
internal class WorkerStatus(val isEnqueued: Boolean, val workerId: UUID? = null)

/**
 * Helper function to check to see if a worker has been scheduled with the [WorkManager] and return
 * the status along with the worker ID in a [WorkerStatus] object.
 *
 * @param context the application [Context] to get the [WorkManager] instance for
 * @param tag a string representing the worker tag
 * @return [WorkerStatus] that contains the enqueued state along with the ID
 */
internal fun getWorkerStatus(context: Context, tag: String): WorkerStatus {
    val instance = WorkManager.getInstance(context)
    val statuses = instance.getWorkInfosByTag(tag)
    try {
        val workInfoList = statuses.get()
        for (workInfo in workInfoList) {
            val state = workInfo.state
            if ((state === WorkInfo.State.RUNNING) || (state === WorkInfo.State.ENQUEUED)) {
                return WorkerStatus(true, workInfo.id)
            }
        }
    } catch (e: ExecutionException) {
        // Do nothing but will return false
    } catch (e: InterruptedException) {
        // Do nothing but will return false
    }

    return WorkerStatus(false, null)
}

/**
 * Wait for a specifically tagged [WorkManager]'s Worker to be enqueued.
 *
 * @param context the application [Context] to get the [WorkManager] instance for
 * @param workTag the tag of the expected Worker
 * @param timeoutMillis how log before stopping the wait. This defaults to 5000ms (5 seconds).
 */
internal fun waitForEnqueuedWorker(
    context: Context,
    workTag: String,
    timeoutMillis: Long = 5000
) = runBlocking {
    withTimeout(timeoutMillis) {
        do {
            if (getWorkerStatus(context, workTag).isEnqueued) {
                return@withTimeout
            }
        } while (isActive)
    }
}

/**
 * Helper function to simulate WorkManager being triggered since there appears to be a bug in
 * the current WorkManager test utilites that prevent it from being triggered by a test.  Once this
 * is fixed, the contents of this can be amended to trigger WorkManager directly.
 *
 * @param context the application [Context] to get the [WorkManager] instance for
 */
internal fun triggerWorkManager(context: Context) {
    // Check that the work is scheduled
    val workerTag = PingUploadWorker.PING_WORKER_TAG
    val status = getWorkerStatus(context, workerTag)
    Assert.assertTrue("A scheduled $workerTag must exist",
        status.isEnqueued)

    // Trigger WorkManager using TestDriver
    val workManagerTestInitHelper = WorkManagerTestInitHelper.getTestDriver(context)
    workManagerTestInitHelper!!.setAllConstraintsMet(status.workerId!!)
}

/**
 * Create a mock webserver that accepts all requests.
 * @return a [MockWebServer] instance
 */
internal fun getMockWebServer(): MockWebServer {
    val server = MockWebServer()
    server.setDispatcher(object : Dispatcher() {
        override fun dispatch(request: RecordedRequest): MockResponse {
            return MockResponse().setBody("OK")
        }
    })
    return server
}

/**
 * Convenience method to get the body of a request as a String.
 * The UTF8 representation of the request body will be returned.
 * If the request body is gzipped, it will be decompressed first.
 *
 * @return a [String] containing the body of the request.
 */
fun RecordedRequest.getPlainBody(): String {
    return if (this.getHeader("Content-Encoding") == "gzip") {
        val bodyInBytes = ByteArrayInputStream(this.body.readByteArray()).readBytes()
        decompressGZIP(bodyInBytes)
    } else {
        this.body.readUtf8()
    }
}

/**
 * Ensure no overdue metrics ping is triggered on `Glean.initialize`.
 *
 * This sets a fake date and time and changes the metrics ping scheduler
 * to assume that now was the last time a metrics ping was sent.
 * This can be used when tests should receive other pings,
 * but don't want to deal with a potential overdue metrics ping first
 */
internal fun delayMetricsPing(context: Context) {
    // Set the current system time to a known datetime.
    val fakeNow = Calendar.getInstance()
    fakeNow.clear()
    fakeNow.set(2015, 6, 11, 2, 0, 0)
    SystemClock.setCurrentTimeMillis(fakeNow.timeInMillis)

    // Set the last sent date to yesterday.
    val mps = MetricsPingScheduler(context)

    mps.updateSentDate(getISOTimeString(fakeNow, truncateTo = TimeUnit.Day))
}

// The following Mockito fixups are copied over from support-test (Matchers.kt) from
// Android-Components. We copied them over since A-C uses the Glean SDK, and we don't
// want a dependency on A-C.

/**
 * Mockito matcher that matches <strong>anything</strong>, including nulls and varargs.
 *
 * (The version from Mockito doesn't work correctly with Kotlin code.)
 */
internal fun <T> any(): T {
    Mockito.any<T>()
    return uninitialized()
}

/**
 * Mockito matcher that matches if the argument is the same as the provided value.
 *
 * (The version from Mockito doesn't work correctly with Kotlin code.)
 */
internal fun <T> eq(value: T): T {
    return Mockito.eq(value) ?: value
}

/**
 * Mockito matcher that captures the passed argument.
 *
 * (The version from Mockito doesn't work correctly with Kotlin code.)
 */
fun <T> capture(value: ArgumentCaptor<T>): T {
    value.capture()
    return uninitialized()
}

@Suppress("UNCHECKED_CAST")
internal fun <T> uninitialized(): T = null as T

/**
 * Waits for ping with the given name to be received
 * in the test ping server.
 *
 * @param pingName the name of the ping to wait for
 * @param maxAttempts the maximum number of attempts
 */
fun waitForPingContent(
    pingName: String,
    pingReason: String?,
    server: MockWebServer,
    maxAttempts: Int = 3
): JSONObject?
{
    var parsedPayload: JSONObject? = null
    for (attempts in 1..maxAttempts) {
        val request = server.takeRequest(20L, java.util.concurrent.TimeUnit.SECONDS) ?: break
        val docType = request.path.split("/")[3]
        if (pingName == docType) {
            parsedPayload = JSONObject(request.getPlainBody())
            if (pingReason == null) {
                break
            }

            // If we requested a specific ping reason, look for it.
            val reason = parsedPayload.getJSONObject("ping_info").getString("reason")
            if (reason == pingReason) {
                break
            }
        }
    }

    return parsedPayload
}
