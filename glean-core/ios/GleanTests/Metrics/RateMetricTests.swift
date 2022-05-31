/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class RateMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "RateMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testRateSavesToStorage() {
        let rateMetric = RateMetricType(CommonMetricData(
            category: "telemetry",
            name: "rate_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ))

        XCTAssertNil(rateMetric.testGetValue())

        rateMetric.addToNumerator(2)
        rateMetric.addToDenominator(5)
        XCTAssertEqual(Rate(numerator: 2, denominator: 5), rateMetric.testGetValue())

        rateMetric.addToNumerator(1)
        XCTAssertEqual(Rate(numerator: 3, denominator: 5), rateMetric.testGetValue())
    }

    func testRateMustNotRecordIfDisabled() {
        let rateMetric = RateMetricType(CommonMetricData(
            category: "telemetry",
            name: "rate_metric",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: true
        ))

        XCTAssertNil(rateMetric.testGetValue())

        rateMetric.addToNumerator(2)
        rateMetric.addToDenominator(5)

        XCTAssertNil(rateMetric.testGetValue())
    }

    func testRateWithExternalDenominator() {
        let meta1 = CommonMetricData(
            category: "telemetry",
            name: "rate1",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        let meta2 = CommonMetricData(
            category: "telemetry",
            name: "rate2",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        )

        let denom = DenominatorMetricType(CommonMetricData(
            category: "telemetry",
            name: "counter",
            sendInPings: ["store1"],
            lifetime: .application,
            disabled: false
        ), [meta1, meta2])

        let num1 = NumeratorMetricType(meta1)
        let num2 = NumeratorMetricType(meta2)

        num1.addToNumerator(3)
        num2.addToNumerator(5)

        denom.add(7)

        XCTAssertEqual(Rate(numerator: 3, denominator: 7), num1.testGetValue())
        XCTAssertEqual(Rate(numerator: 5, denominator: 7), num2.testGetValue())
    }
}
