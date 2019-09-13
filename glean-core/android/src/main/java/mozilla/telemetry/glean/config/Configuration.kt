/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.config

import mozilla.telemetry.glean.BuildConfig
import mozilla.telemetry.glean.rust.toByte

import com.sun.jna.Structure
import com.sun.jna.ptr.IntByReference
import mozilla.telemetry.glean.net.HttpURLConnectionUploader
import mozilla.telemetry.glean.net.PingUploader

/**
 * Define the order of fields as laid out in memory.
 * **CAUTION**: This must match _exactly_ the definition on the Rust side.
 *  If this side is changed, the Rust side need to be changed, too.
 */
@Structure.FieldOrder("dataDir", "packageName", "uploadEnabled", "maxEvents")
internal class FfiConfiguration(
    dataDir: String,
    packageName: String,
    uploadEnabled: Boolean,
    maxEvents: Int? = null
) : Structure() {
    /**
     * Expose all structure fields as actual fields,
     * in order for Structure to turn them into the right memory representiation
     */

    @JvmField
    public var dataDir: String = dataDir
    @JvmField
    public var packageName: String = packageName
    @JvmField
    public var uploadEnabled: Byte = uploadEnabled.toByte()
    @JvmField
    public var maxEvents: IntByReference = if (maxEvents == null) IntByReference() else IntByReference(maxEvents)

    init {
        // Force UTF-8 string encoding when passing strings over the FFI
        this.stringEncoding = "UTF-8"
    }
}

/**
 * The Configuration class describes how to configure Glean.
 *
 * @property serverEndpoint the server pings are sent to. Please note that this is
 *           is only meant to be changed for tests.
 * @property userAgent the user agent used when sending pings, only to be used internally.
 * @property maxEvents the number of events to store before the events ping is sent
 * @property logPings whether to log ping contents to the console. This is only meant to be used
 *           internally by the `GleanDebugActivity`.
 * @property httpClient The HTTP client implementation to use for uploading pings.
 * @property pingTag String tag to be applied to headers when uploading pings for debug view.
 *           This is only meant to be used internally by the `GleanDebugActivity`.
 * @property channel the release channel the application is on, if known. This will be
 *           sent along with all the pings, in the `client_info` section.
 */
data class Configuration internal constructor(
    val serverEndpoint: String,
    val userAgent: String = DEFAULT_USER_AGENT,
    val maxEvents: Int? = null,
    val logPings: Boolean = DEFAULT_LOG_PINGS,
    // NOTE: since only simple object or strings can be made `const val`s, if the
    // default values for the lines below are ever changed, they are required
    // to change in the public constructor below.
    val httpClient: PingUploader = HttpURLConnectionUploader(),
    val pingTag: String? = null,
    val channel: String? = null
) {
    /**
     * Configuration for Glean.
     *
     * @param maxEvents the number of events to store before the events ping is sent
     * @param httpClient The HTTP client implementation to use for uploading pings.
     * @param channel the release channel the application is on, if known. This will be
     *           sent along with all the pings, in the `client_info` section.
     */
    // This is the only public constructor this class should have. It should only
    // expose things we want to allow external applications to change. Every test
    // only or internal configuration option should be added to the above primary internal
    // constructor and only initialized with a proper default when calling the primary
    // constructor from the secondary, public one, below.
    constructor(
        maxEvents: Int? = null,
        httpClient: PingUploader = HttpURLConnectionUploader(),
        channel: String? = null
    ) : this (
        serverEndpoint = DEFAULT_TELEMETRY_ENDPOINT,
        userAgent = DEFAULT_USER_AGENT,
        maxEvents = maxEvents,
        logPings = DEFAULT_LOG_PINGS,
        httpClient = httpClient,
        pingTag = null,
        channel = channel
    )

    companion object {
        /**
         * The default server pings are sent to.
         */
        const val DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
        /**
         * The default user agent used when sending pings.
         */
        const val DEFAULT_USER_AGENT = "Glean/${BuildConfig.LIBRARY_VERSION} (Android)"
        /**
         * Whether to log pings by default.
         */
        const val DEFAULT_LOG_PINGS = false
    }
}
