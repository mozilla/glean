/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class UrlMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testUrlSavesToStorage() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(urlMetric.testHasValue())

        // Record two URLs of the same type, with a little delay.
        urlMetric.set("glean://test")

        // Check that the count was incremented and properly recorded.
        XCTAssert(urlMetric.testHasValue())
        XCTAssertEqual("glean://test", try urlMetric.testGetValue())

        urlMetric.set("glean://other")
        // Check that data was properly recorded.
        XCTAssert(urlMetric.testHasValue())
        XCTAssertEqual("glean://other", try urlMetric.testGetValue())
    }

    func testUrlMustNotRecordIfDisabled() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        )

        XCTAssertFalse(urlMetric.testHasValue())

        urlMetric.set("glean://notrecorded")

        XCTAssertFalse(urlMetric.testHasValue(), "Urls must not be recorded if they are disabled")
    }

    func testUrlGetValueThrowsExceptionIfNothingIsStored() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try urlMetric.testGetValue()) { error in
            XCTAssertEqual(error as! String, "Missing value")
        }
    }

    func testUrlSavesToSecondaryPings() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        urlMetric.set("glean://value")

        XCTAssert(urlMetric.testHasValue("store2"))
        XCTAssertEqual("glean://value", try urlMetric.testGetValue("store2"))

        urlMetric.set("glean://overridenValue")

        XCTAssert(urlMetric.testHasValue("store2"))
        XCTAssertEqual("glean://overridenValue", try urlMetric.testGetValue("store2"))
    }

    func testSettingLongURLsRecordsAnError() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1", "store2"],
            lifetime: .application,
            disabled: false
        )

        let host = String(repeating: "testing", count: 2000)
        urlMetric.set("glean://" + host)

        XCTAssertEqual(1, urlMetric.testGetNumRecordedErrors(.invalidOverflow))
    }

    func testSettingURLType() {
        let urlMetric = UrlMetricType(
            category: "telemetry",
            name: "url_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertFalse(urlMetric.testHasValue())

        // Record two URLs of the same type, with a little delay.
        let url = URL(string: "glean://test")!
        urlMetric.set(url: url)

        // Check that the count was incremented and properly recorded.
        XCTAssert(urlMetric.testHasValue())
        XCTAssertEqual("glean://test", try urlMetric.testGetValue())
    }
}
