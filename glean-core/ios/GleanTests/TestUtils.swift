/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import Gzip
import OHHTTPStubs

/// Stub out receiving a request on Glean's default Telemetry endpoint.
///
/// When receiving a request, it extracts the ping type from the URL
/// according to the endpoint URL format.
///
/// It assumes the request body is a JSON document and tries to decode it.
/// If necessary, it decompresses the request body first.
///
/// - parameters
///       * callback: A callback to validate the incoming request.
///                   It receives a `pingType` and the ping's JSON-decoded `payload`.
func stubServerReceive(callback: @escaping (String, [String: Any]?) -> Void) {
    let host = URL(string: Configuration.Constants.defaultTelemetryEndpoint)!.host!
    stub(condition: isHost(host)) { data in
        let request = data as NSURLRequest
        let url = request.url!
        let parts = url.absoluteString.split(separator: "/")
        let pingType = String(parts[4])

        var body = request.ohhttpStubs_HTTPBody()!
        if request.value(forHTTPHeaderField: "Content-Encoding") == "gzip" {
            body = try! body.gunzipped()
        }

        let payload = try! JSONSerialization.jsonObject(with: body, options: []) as? [String: Any]

        callback(pingType, payload)

        return OHHTTPStubsResponse(
            jsonObject: [],
            statusCode: 200,
            headers: ["Content-Type": "application/json"]
        )
    }
}

/// Stringify a JSON object and if unable to, just return an empty string.
func JSONStringify(_ json: Any) -> String {
    do {
        let data = try JSONSerialization.data(withJSONObject: json, options: .prettyPrinted)
        if let string = String(data: data, encoding: String.Encoding.utf8) {
            return string
        }
    } catch {
        print(error)
    }

    return ""
}
