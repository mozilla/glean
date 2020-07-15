/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.config

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
@Structure.FieldOrder(
    "dataDir",
    "packageName",
    "languageBindingName",
    "uploadEnabled",
    "maxEvents",
    "delayPingLifetimeIO"
)
internal class FfiConfiguration(
    dataDir: String,
    packageName: String,
    languageBindingName: String,
    uploadEnabled: Boolean,
    maxEvents: Int? = null,
    delayPingLifetimeIO: Boolean
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
    public var languageBindingName: String = languageBindingName
    @JvmField
    public var uploadEnabled: Byte = uploadEnabled.toByte()
    @JvmField
    public var maxEvents: IntByReference = if (maxEvents == null) IntByReference() else IntByReference(maxEvents)
    @JvmField
    public var delayPingLifetimeIO: Byte = delayPingLifetimeIO.toByte()

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
 * @property maxEvents the number of events to store before the events ping is sent
 * @property httpClient The HTTP client implementation to use for uploading pings.
 * @property channel the release channel the application is on, if known. This will be
 *           sent along with all the pings, in the `client_info` section.
 */
data class Configuration @JvmOverloads constructor(
    val serverEndpoint: String = DEFAULT_TELEMETRY_ENDPOINT,
    val channel: String? = null,
    val maxEvents: Int? = null,
    // NOTE: since only simple object or strings can be made `const val`s, if the
    // default values for the lines below are ever changed, they are required
    // to change in the public constructor below.
    val httpClient: PingUploader = HttpURLConnectionUploader()
) {
    companion object {
        /**
         * The default server pings are sent to.
         */
        const val DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
    }
}
