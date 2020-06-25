/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// The Configuration struct describes how to configure Glean as well as providing convenience
/// property for calculating the `FfiConfiguration`
public struct Configuration {
    let serverEndpoint: String
    var logPings: Bool
    let maxEvents: Int32?
    let channel: String?

    struct Constants {
        static let defaultTelemetryEndpoint = "https://incoming.telemetry.mozilla.org"
        static let defaultLogPings = false
    }

    /// This init is for internal testing setup only.
    ///
    /// - parameters:
    ///   * serverEndpoint: A `String` representing the server the pings are sent to.
    ///     This should only be changed for tests.
    ///   * logPings: whether to log ping contents to the console.
    ///     This is only meant to be used internally by the `GleanDebugActivity`.
    ///   * maxEvents: the number of events to store before the events ping is sent.
    ///   * channel: the release channel the application is on, if known.
    ///     This will be sent along with all the pings, in the `client_info` section.
    internal init(
        serverEndpoint: String = Constants.defaultTelemetryEndpoint,
        logPings: Bool = Constants.defaultLogPings,
        maxEvents: Int32? = nil,
        channel: String? = nil
    ) {
        self.serverEndpoint = serverEndpoint
        self.logPings = logPings
        self.maxEvents = maxEvents
        self.channel = channel
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
        self.logPings = Constants.defaultLogPings
        self.maxEvents = maxEvents
        self.channel = channel
    }

    /// Returns the current Glean version as a `String`
    ///
    /// - returns: The `String` representation of the version
    static func getGleanVersion() -> String {
        return "\(GleanVersionNumber)"
    }
}
