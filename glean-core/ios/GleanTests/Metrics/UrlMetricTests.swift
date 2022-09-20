/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class UrlMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "UrlMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testUrlSavesToStorage() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(urlMetric.testGetValue())

        // Record two URLs of the same type, with a little delay.
        urlMetric.set("glean://test")

        // Check that the count was incremented and properly recorded.
        XCTAssertEqual("glean://test", urlMetric.testGetValue())

        urlMetric.set("glean://other")
        // Check that data was properly recorded.
        XCTAssertEqual("glean://other", urlMetric.testGetValue())
    }

    func testUrlMustNotRecordIfDisabled() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(urlMetric.testGetValue())

        urlMetric.set("glean://notrecorded")

        XCTAssertNil(urlMetric.testGetValue(), "Urls must not be recorded if they are disabled")
    }

    func testUrlGetValueReturnsNilIfNothingIsStored() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(urlMetric.testGetValue())
    }

    func testUrlSavesToSecondaryPings() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        urlMetric.set("glean://value")

        XCTAssertEqual("glean://value", urlMetric.testGetValue("store2"))

        urlMetric.set("glean://overridenValue")

        XCTAssertEqual("glean://overridenValue", urlMetric.testGetValue("store2"))
    }

    func testSettingLongURLsRecordsAnError() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        ))

        // Whenever the URL is longer than our MAX_URL_LENGTH, we truncate the URL to the
        // MAX_URL_LENGTH.
        //
        // This 8-character string was chosen so we could have an even number that is
        // a divisor of our MAX_URL_LENGTH.
        let longPathBase = "abcdefgh"

        // Using 2000 creates a string > 16000 characters, well over MAX_URL_LENGTH.
        let testUrl = "glean://" + String(repeating: longPathBase, count: 2000)
        urlMetric.set(testUrl)

        // "glean://" is 8 characters
        // "abcdefgh" (longPathBase) is 8 characters
        // `longPathBase` is repeated 1023 times (8184)
        // 8 + 8184 = 8192 (MAX_URL_LENGTH)
        let expected = "glean://" + String(repeating: longPathBase, count: 1023)

        XCTAssertEqual(expected, urlMetric.testGetValue())
        XCTAssertEqual(1, urlMetric.testGetNumRecordedErrors(.invalidOverflow))
    }

    func testSettingURLType() {
        let urlMetric = UrlMetricType(CommonMetricData(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(urlMetric.testGetValue())

        // Record two URLs of the same type, with a little delay.
        let url = URL(string: "glean://test")!
        urlMetric.set(url: url)

        // Check that the count was incremented and properly recorded.
        XCTAssertEqual("glean://test", urlMetric.testGetValue())
    }
}
