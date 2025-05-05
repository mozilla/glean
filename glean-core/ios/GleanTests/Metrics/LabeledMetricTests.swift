/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

class LabeledMetricTypeTests: XCTestCase {
    override func setUp() {
        resetGleanDiscardingInitialPings(testCase: self, tag: "LabeledMetricTypeTests")
    }

    override func tearDown() {
        tearDownStubs()
    }

    func testLabeledCounterType() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric
        )

        labeledCounterMetric["label1"].add(1)
        labeledCounterMetric["label2"].add(2)

        // Record a regular non-labeled counter. This isn't normally
        // possible with the generated code because the subMetric is private,
        // but it's useful to test here that it works.
        counterMetric.add(3)

        XCTAssertEqual(1, labeledCounterMetric["label1"].testGetValue())
        XCTAssertEqual(2, labeledCounterMetric["label2"].testGetValue())
        XCTAssertEqual(3, counterMetric.testGetValue())
    }

    func testOtherLabelWithPredefinedLabels() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric,
            labels: ["foo", "bar", "baz"]
        )

        labeledCounterMetric["foo"].add(1)
        labeledCounterMetric["foo"].add(2)
        labeledCounterMetric["bar"].add(1)
        labeledCounterMetric["not_there"].add(1)
        labeledCounterMetric["also_not_there"].add(1)
        labeledCounterMetric["not_me"].add(1)

        XCTAssertEqual(3, labeledCounterMetric["foo"].testGetValue())
        XCTAssertEqual(1, labeledCounterMetric["bar"].testGetValue())
        XCTAssertNil(labeledCounterMetric["baz"].testGetValue())
        // The rest all lands in the __other__ bucket
        XCTAssertEqual(3, labeledCounterMetric["not_there"].testGetValue())
    }

    func testOtherLabelWithoutPredefinedLabels() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric
        )

        for i in 0 ... 20 {
            labeledCounterMetric["label_\(i)"].add(1)
        }

        // Go back and record in one of the real labels again
        labeledCounterMetric["label_0"].add(1)

        XCTAssertEqual(2, labeledCounterMetric["label_0"].testGetValue())
        for i in 1 ... 15 {
            XCTAssertEqual(1, labeledCounterMetric["label_\(i)"].testGetValue())
        }
        XCTAssertEqual(5, labeledCounterMetric["__other__"].testGetValue())
    }

    func testEnsureInvalidLabelsGoToOther() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric
        )

        // These are fine, now.
        labeledCounterMetric["notSnakeCase"].add(1)
        labeledCounterMetric[""].add(1)
        labeledCounterMetric["with/slash"].add(1)
        labeledCounterMetric["this_string_has_more_than_thirty_characters"].add(1)

        XCTAssertEqual(
            0,
            labeledCounterMetric.testGetNumRecordedErrors(.invalidLabel)
        )

        XCTAssertEqual(nil, labeledCounterMetric["__other__"].testGetValue())

        // More than 111 characters? Not okay.
        labeledCounterMetric[String(repeating: "1", count: 112)].add(1)
        // Not ASCII? Not okay.
        labeledCounterMetric["Spøøn"].add(1)
        XCTAssertEqual(
            2,
            labeledCounterMetric.testGetNumRecordedErrors(.invalidLabel)
        )

        XCTAssertEqual(2, labeledCounterMetric["__other__"].testGetValue())
    }

    func testLabeledStringType() {
        let counterMetric = StringMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledStringMetric = try! LabeledMetricType<StringMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric
        )

        labeledStringMetric["label1"].set("foo")
        labeledStringMetric["label2"].set("bar")

        XCTAssertEqual("foo", labeledStringMetric["label1"].testGetValue())
        XCTAssertEqual("bar", labeledStringMetric["label2"].testGetValue())
    }

    func testLabeledBooleanType() {
        let booleanMetric = BooleanMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_boolean_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledBooleanMetric = try! LabeledMetricType<BooleanMetricType>(
            category: "telemetry",
            name: "labeled_boolean_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: booleanMetric
        )

        labeledBooleanMetric["label1"].set(false)
        labeledBooleanMetric["label2"].set(true)

        XCTAssertEqual(false, labeledBooleanMetric["label1"].testGetValue())
        XCTAssertEqual(true, labeledBooleanMetric["label2"].testGetValue())
    }

    func testLabeledQuantityType() {
        let quantityMetric = QuantityMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_quantity_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledQuantityMetric = try! LabeledMetricType<QuantityMetricType>(
            category: "telemetry",
            name: "labeled_quantity_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: quantityMetric
        )

        labeledQuantityMetric["label1"].set(42)
        labeledQuantityMetric["label2"].set(43)

        XCTAssertEqual(42, labeledQuantityMetric["label1"].testGetValue())
        XCTAssertEqual(43, labeledQuantityMetric["label2"].testGetValue())
    }

    func testLabeledEventsThrowAnException() {
        let eventMetric = EventMetricType<NoExtras>(CommonMetricData(
            category: "telemetry",
            name: "labeled_event",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ), nil)

        XCTAssertThrowsError(try LabeledMetricType<EventMetricType<NoExtras>>(
            category: "telemetry",
            name: "labeled_event_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: eventMetric
        )) { error in
            XCTAssertEqual(error as! String, "Can not create a labeled version of this metric type")
        }
    }
}
