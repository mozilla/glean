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

    struct Constants {
        static let defaultTelemetryEndpoint = "https://incoming.telemetry.mozilla.org"
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
    public init(
        maxEvents: Int32? = nil,
        channel: String? = nil,
        serverEndpoint: String? = nil,
        dataPath: String? = nil,
        logLevel: LevelFilter? = nil
    ) {
        self.serverEndpoint = serverEndpoint ?? Constants.defaultTelemetryEndpoint
        self.maxEvents = maxEvents
        self.channel = channel
        self.dataPath = dataPath
        self.logLevel = logLevel
    }
}
