/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// The Configuration struct describes how to configure Glean as well as providing convenience
/// property for calculating the `FfiConfiguration`
public struct Configuration {
    let serverEndpoint: String
    let maxEvents: Int32?
    let channel: String?

    struct Constants {
        static let defaultTelemetryEndpoint = "https://incoming.telemetry.mozilla.org"
    }

    /// Create a new Glean `Configuration` object
    ///
    /// - parameters:
    ///   * maxEvents the number of events to store before the events ping is sent.
    ///   * channel the release channel the application is on, if known.
    ///     This will be sent along with all the pings, in the `client_info` section.
    public init(
        maxEvents: Int32? = nil,
        channel: String? = nil,
        serverEndpoint: String? = nil
    ) {
        self.serverEndpoint = serverEndpoint ?? Constants.defaultTelemetryEndpoint
        self.maxEvents = maxEvents
        self.channel = channel
    }
}
