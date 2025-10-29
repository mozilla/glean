/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

struct BalloonsObjectItem: Codable, Equatable {
    var colour: String?
    var diameter: Int64?
    var anotherValue: Bool?

    enum CodingKeys: String, CodingKey {
        case colour = "colour"
        case diameter = "diameter"
        case anotherValue = "another_value"
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        if let colour = self.colour {
            try container.encode(colour, forKey: .colour)
        }
        if let diameter = self.diameter {
            try container.encode(diameter, forKey: .diameter)
        }
        if let anotherValue = self.anotherValue {
            try container.encode(anotherValue, forKey: .anotherValue)
        }
    }
}
typealias BalloonsObject = [BalloonsObjectItem]

// generated from
//
// ```
// structure:
//   type: object
//   properties:
//     key1:
//       type: string
//     another_value:
//       type: number
//     sub_array:
//       type: array
//       items:
//          type: number
// ```
struct ToplevelObjectObject: Codable, Equatable, ObjectSerialize {
    var key1: String?
    var anotherValue: Int64?
    var subArray: ToplevelObjectObjectSubArray = []

    enum CodingKeys: String, CodingKey {
        case key1 = "key1"
        case anotherValue = "another_value"
        case subArray = "sub_array"
    }

    func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: CodingKeys.self)
        if let key1 = self.key1 {
            try container.encode(key1, forKey: .key1)
        }
        if let anotherValue = self.anotherValue {
            try container.encode(anotherValue, forKey: .anotherValue)
        }
        if subArray.count > 0 {
            let subArray = self.subArray
            try container.encode(subArray, forKey: .subArray)
        }
    }

    func intoSerializedObject() -> String {
        let jsonEncoder = JSONEncoder()
        let jsonData = try! jsonEncoder.encode(self)
        let json = String(data: jsonData, encoding: String.Encoding.utf8)!
        return json
    }
}
typealias ToplevelObjectObjectSubArray = [ToplevelObjectObjectSubArrayItem]
typealias ToplevelObjectObjectSubArrayItem = Int64

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

     func testObjectDecodesFromSnakeCase() {
         let metric = ObjectMetricType<BalloonsObject>(CommonMetricData(
             category: "test",
             name: "balloon",
             sendInPings: ["store1"],
             lifetime: .ping,
             disabled: false
         ))

         XCTAssertNil(metric.testGetValue())

         var balloons: BalloonsObject = []
         balloons.append(BalloonsObjectItem(colour: "red", diameter: 5, anotherValue: true))
         balloons.append(BalloonsObjectItem(colour: "green", anotherValue: false))
         metric.set(balloons)

         let snapshot = metric.testGetValue()!
         XCTAssertNotNil(snapshot)
         XCTAssertEqual(2, snapshot.count)

         XCTAssertEqual(snapshot[0].colour, "red")
         XCTAssertEqual(snapshot[0].diameter, 5)
         XCTAssertEqual(snapshot[0].anotherValue, true)
         XCTAssertEqual(snapshot[1].colour, "green")
         XCTAssertNil(snapshot[1].diameter)
         XCTAssertEqual(snapshot[1].anotherValue, false)
     }

    func testObjectWithStructureOnToplevel() {
        let metric = ObjectMetricType<ToplevelObjectObject>(CommonMetricData(
            category: "test",
            name: "toplevel_object",
            sendInPings: ["store1"],
            lifetime: .ping,
            disabled: false
        ))

        XCTAssertNil(metric.testGetValue())

        let obj = ToplevelObjectObject(key1: "test", anotherValue: 3, subArray: [1, 2, 3])
        metric.set(obj)

        let snapshot = metric.testGetValue()!
        XCTAssertNotNil(snapshot)

        XCTAssertEqual("test", snapshot.key1)
        XCTAssertEqual(3, snapshot.anotherValue)
        XCTAssertEqual([1, 2, 3], snapshot.subArray)
    }
}
