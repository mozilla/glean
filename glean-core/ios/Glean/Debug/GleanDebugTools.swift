/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class is meant to encapsulate the Glean debug functionality.
class GleanDebugUtility {
    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/GleanDebugUtility"
    }

    private static let logger = Logger(tag: Constants.logTag)

    /// When applications are launched using the custom URL scheme, this helper function will process
    /// the URL and parse the debug commands
    ///
    /// - parameters:
    ///     * url: A `URL` object containing the Glean debug commands as query parameters
    ///
    /// There are 4 available commands that you can use with the Glean SDK debug tools
    ///
    /// - `logPings`: If "true", will cause pings that are submitted successfully to also be echoed to the device's log.
    /// - `debugViewTag`:  This command expects a string to tag the pings with and redirects them to the Debug View.
    ///     The string must match the pattern `[a-zA-Z0-9-]{1,20}`.
    /// - `sourceTags`: This command tags all outgoing pings to make them available for real-time validation.
    ///     Tags are represented by a comma separated list. Each tag must match the pattern `[a-zA-Z0-9-]{1,20}`.
    ///     Up to 5 tags are allowed.
    /// - `sendPing`: This command expects a string name of a ping to force immediate collection and submission of.
    ///
    /// The structure of the custom URL uses the following format:
    ///
    /// `<protocol>://glean?<command 1>=<paramter 1>&<command 2>=<parameter 2> ...`
    ///
    /// Where:
    ///
    /// - `<protocol>://` is the "Url Scheme" that has been added for the app and doesn't matter to Glean.
    /// - `glean` is required for the Glean SDK to recognize the command is meant for it to process.
    /// - `?` indicating the beginning of the query.
    /// - `<command>=<parameter>` are instances of the commands listed above  separated by `&`.
    ///
    /// There are a few things to consider when creating the custom URL:
    ///
    /// - Invalid commands will trigger an error and be ignored.
    /// - Not all commands are requred to be encoded in the URL, you can mix and match the commands that you need.
    /// - Special characters should be properly URL encoded and escaped since this needs to represent a valid URL.
    static func handleCustomUrl(url: URL) {
        guard url.host?.removingPercentEncoding == "glean" else {
            return
        }

        guard let components = URLComponents(url: url, resolvingAgainstBaseURL: true),
            let params = components.queryItems else {
            logger.error("Error parsing query parameters, aborting Glean debugging tools")
            return
        }

        if let parsedCommands = processCustomUrlQuery(urlQueryItems: params) {
            if let debugTag = parsedCommands.debugViewTag {
                if Glean.shared.setDebugViewTag(debugTag) {
                    logger.debug("Pings tagged with debug tag: \(debugTag)")
                } else {
                    logger.error("Invalid ping debug tag name, aborting Glean debug tools")
                    return
                }
            }

            if let sourceTags = parsedCommands.sourceTags {
                if Glean.shared.setSourceTags(sourceTags) {
                    logger.debug("Pings tagged with source tags: \(sourceTags)")
                } else {
                    logger.error("Invalid ping source tags value, aborting Glean debug tools")
                    return
                }
            }

            if let logPings = parsedCommands.logPings {
                Glean.shared.setLogPings(logPings)
                logger.debug("Log pings set to: \(logPings)")
            }

            if let pingName = parsedCommands.pingNameToSend {
                Glean.shared.submitPingByName(pingName: pingName)
                logger.debug("Glean debug tools triggered ping: \(pingName)")
            }
        }
    }

    /// A simple struct to represent the commands parsed from the custom URL
    private struct ParsedQueryCommands {
        let debugViewTag: String?
        let sourceTags: [String]?
        let logPings: Bool?
        let pingNameToSend: String?
    }

    // swiftlint:disable cyclomatic_complexity
    /// Helper function to process parameters passed as a query to the custom URL processed by `handleCustomUrl`
    ///
    /// - parameters:
    ///     * urlQueryItems: A `URLQueryItem` representing the commands passed to Glean
    ///
    /// - returns: A tuple containing the values of the parameters `pingTag`, `logPings`, `pingNamesToSend` or nil
    ///     when invalid or duplicated commands are detected.
    private static func processCustomUrlQuery(urlQueryItems: [URLQueryItem]) -> ParsedQueryCommands? {
        var debugViewTag: String?
        var sourceTags: [String]?
        var willLogPings: Bool?
        var pingToSend: String?

        for param in urlQueryItems {
            if param.value == nil {
                logger.error("Empty values are unsupported, aborting Glean debug tools")
                return nil
            }

            switch param.name {
            case "debugViewTag":
                if debugViewTag != nil {
                    logger.error(
                        "Multiple `debugViewTag` commands not allowed, aborting Glean debug tools")
                    return nil
                }

                debugViewTag = param.value
            case "sourceTags":
                if sourceTags != nil {
                    logger.error(
                        "Multiple `sourceTags` commands not allowed, aborting Glean debug tools")
                    return nil
                }

                sourceTags = param.value?.components(separatedBy: ",")
            case "logPings":
                if willLogPings != nil {
                    logger.error("Multiple `logPings` commands not allowed, aborting Glean debug tools")
                    return nil
                }

                // If param.value is any string other than "true" or "false", the result is nil.
                // This initializer is case sensitive. See Apple docs for more info at:
                // https://developer.apple.com/documentation/swift/bool
                willLogPings = Bool(param.value!)
            case "sendPing":
                if pingToSend != nil {
                    logger.error("Multiple `sendPing` commands not allowed, aborting Glean debug tools")
                    return nil
                }

                pingToSend = param.value
            default:
                logger.error("Unknown parameter passed to Glean.handleCustomUrl, aborting Glean debug tools")
                return nil
            }
        }

        return ParsedQueryCommands(
            debugViewTag: debugViewTag,
            sourceTags: sourceTags,
            logPings: willLogPings,
            pingNameToSend: pingToSend
        )
    }

    // swiftlint:enable cyclomatic_complexity
}
