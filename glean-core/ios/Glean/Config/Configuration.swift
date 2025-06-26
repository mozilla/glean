/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// The Configuration struct describes how to configure Glean as well as providing convenience
/// property for calculating the `FfiConfiguration`
public struct Configuration {
    let serverEndpoint: String
    let maxEvents: Int32?
    let channel: String?
    let dataPath: String?
    let logLevel: LevelFilter?
    let enableEventTimestamps: Bool
    let experimentationId: String?
    let enableInternalPings: Bool
    let pingLifetimeThreshold: Int
    let pingLifetimeMaxTime: Int
    let pingSchedule: [String: [String]]
    let httpClient: PingUploader?

    struct Constants {
        static let defaultTelemetryEndpoint =
            "https://incoming.telemetry.mozilla.org"
    }

    /// Create a new Glean `Configuration` object
    ///
    /// - parameters:
    ///   * maxEvents the number of events to store before the events ping is sent.
    ///   * channel the release channel the application is on, if known.
    ///   This will be sent along with all the pings, in the `client_info` section.
    ///   * serverEndpoint the server endpoint Glean should send data to
    ///   * dataPath an optional String that specifies where to store data locally on the device.
    ///   This should ONLY be used when setting up Glean on a non-main process.
    ///   * logLevel an optional log level that controls how verbose the internal logging is.
    ///   * enableEventTimestamps whether to add a wallclock timestamp to all events
    ///   * experimentationId An experimentation identifier derived by the application
    ///   to be sent with all pings.
    ///   * enableInternalPings Whether to enable internal pings.
    ///   * pingLifetimeThreshold Write count threshold when to auto-flush. `0` disables it.
    ///   * pingLifetimeMaxTime After what time to auto-flush (in milliseconds). 0 disables it.
    ///   * pingSchedule A ping schedule map.
    ///   Maps a ping name to a list of pings to schedule along with it.
    ///   Only used if the ping's own ping schedule list is empty.
    ///   * httpClient An http uploader that supports the `PingUploader` protocol
    public init(
        maxEvents: Int32? = nil,
        channel: String? = nil,
        serverEndpoint: String? = nil,
        dataPath: String? = nil,
        logLevel: LevelFilter? = nil,
        enableEventTimestamps: Bool = true,
        experimentationId: String? = nil,
        enableInternalPings: Bool = true,
        pingLifetimeThreshold: Int = 0,
        pingLifetimeMaxTime: Int = 0,
        pingSchedule: [String: [String]] = [:],
        httpClient: PingUploader? = nil
    ) {
        self.serverEndpoint =
            serverEndpoint ?? Constants.defaultTelemetryEndpoint
        self.maxEvents = maxEvents
        self.channel = channel
        self.dataPath = dataPath
        self.logLevel = logLevel
        self.enableEventTimestamps = enableEventTimestamps
        self.experimentationId = experimentationId
        self.enableInternalPings = enableInternalPings
        self.pingLifetimeThreshold = pingLifetimeThreshold
        self.pingLifetimeMaxTime = pingLifetimeMaxTime
        self.pingSchedule = pingSchedule
        self.httpClient = httpClient
    }
}
