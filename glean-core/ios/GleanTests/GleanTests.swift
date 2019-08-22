//
//  GleanTests.swift
//  GleanTests
//
//  Created by Jan-Erik Rediger on 21.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

import XCTest
@testable import Glean

class GleanTests: XCTestCase {

    override func setUp() {
        Glean.shared.initialize(uploadEnabled: true)
        Glean.shared.testClearAllStores()
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDown() {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
    }

    func testCounterIncrement() {
        let testCounter = CounterMetricType(category: "telemetry.test", name: "count", sendInPings: ["metrics"], lifetime: .ping, disabled: false)
        testCounter.add(amount: 42)
        XCTAssertEqual(42, testCounter.testGetValue())
    }

    func testCounterWhenUploadDisabled() {
        Glean.shared.setUploadEnabled(false)

        let testCounter = CounterMetricType(category: "telemetry.test", name: "count", sendInPings: ["metrics"], lifetime: .ping, disabled: false)
        testCounter.add(amount: 42)
        XCTAssertFalse(testCounter.testHasValue())
    }

    func testSendPings() {
        let testCounter = CounterMetricType(category: "telemetry.test", name: "count", sendInPings: ["metrics"], lifetime: .ping, disabled: false)
        testCounter.add(amount: 42)
        XCTAssertTrue(Glean.shared.sendPings(["metrics"]))
    }

}
