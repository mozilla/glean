/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import Gzip
import OHHTTPStubs
import OHHTTPStubsSwift
import XCTest

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

        return HTTPStubsResponse(
            jsonObject: [],
            statusCode: 200,
            headers: ["Content-Type": "application/json"]
        )
    }
}

/// Resets Glean, and discards any pings sent during `initialize()`
/// that might interfere with what is being tested in the specific unit test
///
/// This also prevents outgoing network requests during unit tests while
/// still allowing us to use the default telemetry endpoint.
func resetGleanDiscardingInitialPings(testCase: XCTestCase, tag: String, clearStores: Bool = true) {
    let expectation = testCase.expectation(description: "\(tag): Ping Received")

    // We are using OHHTTPStubs combined with an XCTestExpectation in order to capture
    // outgoing network requests and prevent actual requests being made from tests.
    // We wait for 2 pings, in this order:
    //
    // 1. baseline ping with reason "dirty-startup"
    // 2. baseline ping with reason "active"
    //
    // Once we received the "active" baseline ping we're good to go.
    // All subsequent pings should be from the test itself.
    stubServerReceive { pingType, json in
        if pingType != "baseline" {
            return
        }
        let pingInfo = json?["ping_info"] as? [String: Any]
        let reason = pingInfo?["reason"] as? String
        if reason != "active" {
            return
        }

        // Fulfill test's expectation once we parsed the incoming data.
        DispatchQueue.main.async {
            // Let the response get processed before we mark the expectation fulfilled
            expectation.fulfill()
        }
    }

    // We may recieve more than one ping, using this function means we don't care about any of them
    expectation.assertForOverFulfill = false

    // Force NO overdue metrics ping. We only expect baseline pings.
    let mps = MetricsPingScheduler(true)
    mps.updateSentDate(Date())

    Glean.shared.resetGlean(clearStores: clearStores)

    testCase.waitForExpectations(timeout: 5.0) { error in
        XCTAssertNil(error, "Test timed out waiting for upload: \(error!)")
    }

    // In case this isn't the first test, there might be pending operations.
    // We wait for them to finish, so they don't clutter the next tests.
    Dispatchers.shared.serialOperationQueue.waitUntilAllOperationsAreFinished()
}

func tearDownStubs() {
    HTTPStubs.removeAllStubs()
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

func stubBuildInfo(_ dateString: String? = nil) -> BuildInfo {
    let buildDate: DateComponents

    if let dateString = dateString {
        let date = Date.fromISO8601String(dateString: dateString, precision: .second)!
        buildDate = Calendar.current.dateComponents(in: TimeZone(abbreviation: "UTC")!, from: date)
    } else {
        buildDate = DateComponents(
            calendar: Calendar.current,
            timeZone: TimeZone(abbreviation: "UTC"),
            year: 2019,
            month: 10,
            day: 23,
            hour: 12,
            minute: 52,
            second: 8
        )
    }
    return BuildInfo(buildDate: buildDate)
}
