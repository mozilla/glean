/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.config

import mozilla.telemetry.glean.internal.LevelFilter
import mozilla.telemetry.glean.net.HttpURLConnectionUploader
import mozilla.telemetry.glean.net.PingUploader

/**
 * The Configuration class describes how to configure Glean.
 *
 * @property serverEndpoint the server pings are sent to. Please note that this is
 *           is only meant to be changed for tests.
 * @property maxEvents the number of events to store before the events ping is sent
 * @property httpClient The HTTP client implementation to use for uploading pings.
 * @property channel the release channel the application is on, if known. This will be
 *           sent along with all the pings, in the `client_info` section.
 * @property dataPath An optional [String] that specifies where to store data locally on the device.
 *           This should ONLY be used when setting up Glean on a non-main process.
 * @property logLevel An optional [LevelFilter] that controls how verbose the internal logging is.
 */
data class Configuration @JvmOverloads constructor(
    val serverEndpoint: String = DEFAULT_TELEMETRY_ENDPOINT,
    val channel: String? = null,
    val maxEvents: Int? = null,
    // NOTE: since only simple object or strings can be made `const val`s, if the
    // default values for the lines below are ever changed, they are required
    // to change in the public constructor below.
    val httpClient: PingUploader = HttpURLConnectionUploader(),
    val dataPath: String? = null,
    val logLevel: LevelFilter? = null,
) {
    companion object {
        /**
         * The default server pings are sent to.
         */
        const val DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
    }
}
