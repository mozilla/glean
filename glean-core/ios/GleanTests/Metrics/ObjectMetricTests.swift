/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

struct BalloonsObjectItem: Decodable, Equatable, ObjectSerialize {
    var colour: String?
    var diameter: Int64?

    func intoSerializedObject() -> String {
        var data: [String] = []
        if let val = self.colour {
            var elem = "\"colour\":"
            elem.append(val.intoSerializedObject())
            data.append(elem)
        }
        if let val = self.diameter {
            var elem = "\"diameter\":"
            elem.append(val.intoSerializedObject())
            data.append(elem)
        }
        let obj = data.joined(separator: ",")
        let json = "{" + obj + "}"
        return json
    }
}

typealias BalloonsObject = [BalloonsObjectItem]

class ObjectMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "ObjectMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testObjectSavesToStorage() {
        let metric = ObjectMetricType<BalloonsObject>(CommonMetricData(
            category: "test",
            name: "balloon",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
        ))

        XCTAssertNil(metric.testGetValue())

        var balloons: BalloonsObject = []
        balloons.append(BalloonsObjectItem(colour: "red", diameter: 5))
        balloons.append(BalloonsObjectItem(colour: "green"))
        metric.set(balloons)

        let snapshot = metric.testGetValue()!
        XCTAssertEqual(2, snapshot.count)

        let expectedJson = """
        [
            { "colour": "red", "diameter": 5 },
            { "colour": "green" }
        ]
        """
        let jsonDecoder = JSONDecoder()
        let expected = try! jsonDecoder.decode(BalloonsObject.self, from: Data(expectedJson.utf8))

        XCTAssertEqual(expected, snapshot)
    }

    func testObjectMustNotRecordIfDisabled() {
        let metric = ObjectMetricType<BalloonsObject>(CommonMetricData(
            category: "test",
            name: "balloon",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true
        ))

        var balloons: BalloonsObject = []
        balloons.append(BalloonsObjectItem(colour: "yellow", diameter: 10))
        metric.set(balloons)

        XCTAssertNil(metric.testGetValue())

    }

    func testObjectGetValueReturnsNilIfNothingIsStored() {
        let metric = ObjectMetricType<BalloonsObject>(CommonMetricData(
            category: "test",
            name: "balloon",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: true
        ))

        XCTAssertNil(metric.testGetValue())
    }

    func testObjectSavesToSecondaryPings() {
        let metric = ObjectMetricType<BalloonsObject>(CommonMetricData(
            category: "test",
            name: "balloon",
            sendInPings: ["store1", "store2"],
            lifetime: .ping,
            disabled: false
        )
        )

        XCTAssertNil(metric.testGetValue())

        var balloons: BalloonsObject = []
        balloons.append(BalloonsObjectItem(colour: "red", diameter: 5))
        balloons.append(BalloonsObjectItem(colour: "green"))
        metric.set(balloons)

        let expectedJson = """
        [
            { "colour": "red", "diameter": 5 },
            { "colour": "green" }
        ]
        """
        let jsonDecoder = JSONDecoder()
        let expected = try! jsonDecoder.decode(BalloonsObject.self, from: Data(expectedJson.utf8))

        var snapshot = metric.testGetValue("store1")!
        XCTAssertEqual(2, snapshot.count)
        XCTAssertEqual(expected, snapshot)

        snapshot = metric.testGetValue("store2")!
        XCTAssertEqual(2, snapshot.count)
        XCTAssertEqual(expected, snapshot)
    }
}
