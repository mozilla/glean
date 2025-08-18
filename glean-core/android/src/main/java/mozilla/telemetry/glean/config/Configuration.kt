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
 * @property enableEventTimestamps Whether to add a wallclock timestamp to all events.
 * @property experimentationId An experimentation identifier derived by the application
 *           to be sent with all pings.
 * @property enableInternalPings Whether to enable internal pings.
 * @property delayPingLifetimeIo Whether Glean should delay persistence of data from metrics with ping lifetime.
 * @property pingLifetimeThreshold Write count threshold when to auto-flush. `0` disables it.
 * @property pingLifetimeMaxTime After what time to auto-flush (in milliseconds). 0 disables it.
 * @property pingSchedule A ping schedule map.
 *           Maps a ping name to a list of pings to schedule along with it.
 *           Only used if the ping's own ping schedule list is empty.
 */
data class Configuration
    @JvmOverloads
    constructor(
        val serverEndpoint: String = DEFAULT_TELEMETRY_ENDPOINT,
        val channel: String? = null,
        val maxEvents: Int? = null,
        // NOTE: since only simple object or strings can be made `const val`s, if the
        // default values for the lines below are ever changed, they are required
        // to change in the public constructor below.
        val httpClient: PingUploader = HttpURLConnectionUploader(),
        val dataPath: String? = null,
        val logLevel: LevelFilter? = null,
        val enableEventTimestamps: Boolean = true,
        val experimentationId: String? = null,
        val enableInternalPings: Boolean = true,
        val delayPingLifetimeIo: Boolean = true,
        val pingLifetimeThreshold: Int = 1000,
        val pingLifetimeMaxTime: Int = 0,
        val pingSchedule: Map<String, List<String>> = emptyMap(),
    ) {
        companion object {
            /**
             * The default server pings are sent to.
             */
            const val DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
        }
    }
