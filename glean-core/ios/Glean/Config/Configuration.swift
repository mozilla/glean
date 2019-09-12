/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// The Configuration struct describes how to configure Glean as well as providing convenience
/// property for calculating the `FfiConfiguration`
public struct Configuration {
    let serverEndpoint: String
    let userAgent: String
    let connectionTimeout: Int
    let logPings: Bool
    let httpClient: HttpPingUploader
    let maxEvents: Int32?
    let pingTag: String?
    let channel: String?

    // swiftlint:disable identifier_name
    static let DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
    static let DEFAULT_USER_AGENT = "Glean/\(getGleanVersion()) (iOS)"
    public static let DEFAULT_CONNECTION_TIMEOUT = 10000
    static let DEFAULT_LOG_PINGS = false
    // swiftlint:enable identifier_name

    /// This init is for internal testing setup only.
    ///
    /// - parameters:
    ///   * serverEndpoint: A `String` representing the server the pings are sent to.
    ///     This should only be changed for tests.
    ///   * userAgent: the user agent used when sending pings, only to be used internally.
    ///   * connectionTimeout: the timeout, in milliseconds, to use when connecting to the `serverEndpoint`.
    ///   * logPings: whether to log ping contents to the console.
    ///     This is only meant to be used internally by the `GleanDebugActivity`.
    ///   * httpClient: The HTTP client implementation to use for uploading pings.
    ///   * maxEvents: the number of events to store before the events ping is sent.
    ///   * pingTag: String tag to be applied to headers when uploading pings for debug view.
    ///     Used internally by the `GleanDebugActivity`.
    ///   * channel: the release channel the application is on, if known.
    ///     This will be sent along with all the pings, in the `client_info` section.
    internal init(
        serverEndpoint: String = DEFAULT_TELEMETRY_ENDPOINT,
        userAgent: String = DEFAULT_USER_AGENT,
        connectionTimeout: Int = DEFAULT_CONNECTION_TIMEOUT,
        logPings: Bool = DEFAULT_LOG_PINGS,
        httpClient: HttpPingUploader = HttpPingUploader(),
        maxEvents: Int32? = nil,
        pingTag: String? = nil,
        channel: String? = nil
    ) {
        self.serverEndpoint = serverEndpoint
        self.userAgent = userAgent
        self.connectionTimeout = connectionTimeout
        self.logPings = logPings
        self.httpClient = httpClient
        self.maxEvents = maxEvents
        self.pingTag = pingTag
        self.channel = channel
    }

    /// Create a new Glean `Configuration` object
    ///
    /// - parameters:
    ///   * connectionTimeout the timeout, in milliseconds, to use when connecting to the `serverEndpoint`.
    ///   * maxEvents the number of events to store before the events ping is sent.
    ///   * httpClient The HTTP client implementation to use for uploading pings.
    ///   * channel the release channel the application is on, if known.
    ///     This will be sent along with all the pings, in the `client_info` section.
    public init(
        connectionTimeout: Int = DEFAULT_CONNECTION_TIMEOUT,
        maxEvents: Int32? = nil,
        httpClient: HttpPingUploader = HttpPingUploader(),
        channel: String? = nil
    ) {
        self.serverEndpoint = Configuration.DEFAULT_TELEMETRY_ENDPOINT
        self.userAgent = Configuration.DEFAULT_USER_AGENT
        self.connectionTimeout = connectionTimeout
        self.logPings = Configuration.DEFAULT_LOG_PINGS
        self.httpClient = httpClient
        self.maxEvents = maxEvents
        self.pingTag = nil
        self.channel = channel
    }

    /// Returns the current Glean version as a `String`
    ///
    /// - returns: The `String` representation of the version
    static func getGleanVersion() -> String {
        return "\(GleanVersionNumber)"
    }
}
