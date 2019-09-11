/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// The Configuration struct describes how to configure Glean
///
/// - parameters:
///   * serverEndpoint: A `String` representing the server the pings are sent to.  This should only be changed
///     for tests
///   * userAgent: the user agent used when sending pings, only to be used internally.
///   * connectionTimeout: the timeout, in milliseconds, to use when connecting to the `serverEndpoint`.
///   * readTimeout: the timeout, in milliseconds, to use when connecting to the `serverEndpoint`.
///   * maxEvents: the number of events to store before the events ping is sent.
///   * logPings: whether to log ping contents to the console. This is only meant to be used internally by the
///     `GleanDebugActivity`.
///   * httpClient: The HTTP client implementation to use for uploading pings.
///   * pingTag: String tag to be applied to headers when uploading pings for debug view. This is only meant to
///     be used internally by the `GleanDebugActivity`.
///   * channel: the release channel the application is on, if known. This will be sent along with all the pings,
///     in the `client_info` section.
struct Configuration {
    let serverEndpoint: String = Configuration.DEFAULT_TELEMETRY_ENDPOINT
    let userAgent: String = Configuration.DEFAULT_USER_AGENT
    let connectionTimeout: Int = Configuration.DEFAULT_CONNECTION_TIMEOUT
    let readTimeout: Int = Configuration.DEFAULT_READ_TIMEOUT
    let logPings: Bool = Configuration.DEFAULT_LOG_PINGS
    // let httpClient: HttpPingUploader = HttpPingUploader()
    let maxEvents: Int? = nil
    let pingTag: String? = nil
    let channel: String? = nil

    // swiftlint:disable identifier_name
    static let DEFAULT_TELEMETRY_ENDPOINT = "https://incoming.telemetry.mozilla.org"
    static let DEFAULT_USER_AGENT = "Glean/\(getGleanVersion()) (iOS)"
    static let DEFAULT_CONNECTION_TIMEOUT = 10000
    static let DEFAULT_READ_TIMEOUT = 30000
    static let DEFAULT_LOG_PINGS = false
    // swiftlint:enable identifier_name

    /// Returns the current Glean version from the Bundle as a `String`
    ///
    /// - returns: The `String` representation of the version
    static func getGleanVersion() -> String {
        return (Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String) ?? ""
    }
}
