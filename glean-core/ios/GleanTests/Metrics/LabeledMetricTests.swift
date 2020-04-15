/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable force_cast
// REASON: Used in a test
class LabeledMetricTypeTests: XCTestCase {
    override func setUp() {
        Glean.shared.resetGlean(clearStores: true)
    }

    func testLabeledCounterType() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

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

        XCTAssert(labeledCounterMetric["label1"].testHasValue())
        XCTAssertEqual(1, try labeledCounterMetric["label1"].testGetValue())

        XCTAssert(labeledCounterMetric["label2"].testHasValue())
        XCTAssertEqual(2, try labeledCounterMetric["label2"].testGetValue())

        XCTAssert(counterMetric.testHasValue())
        XCTAssertEqual(3, try counterMetric.testGetValue())
    }

    func testOtherLabelWithPredefinedLabels() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

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

        XCTAssertEqual(3, try labeledCounterMetric["foo"].testGetValue())
        XCTAssertEqual(1, try labeledCounterMetric["bar"].testGetValue())
        XCTAssertFalse(labeledCounterMetric["baz"].testHasValue())
        // The rest all lands in the __other__ bucket
        XCTAssertEqual(3, try labeledCounterMetric["not_there"].testGetValue())
    }

    func testOtherLabelWithoutPredefinedLabels() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

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

        XCTAssertEqual(2, try labeledCounterMetric["label_0"].testGetValue())
        for i in 1 ... 15 {
            XCTAssertEqual(1, try labeledCounterMetric["label_\(i)"].testGetValue())
        }
        XCTAssertEqual(5, try labeledCounterMetric["__other__"].testGetValue())
    }

    func testEnsureInvalidLabelsGoToOther() {
        let counterMetric = CounterMetricType(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false,
            subMetric: counterMetric
        )

        labeledCounterMetric["notSnakeCase"].add(1)
        labeledCounterMetric[""].add(1)
        labeledCounterMetric["with/slash"].add(1)
        labeledCounterMetric["this_string_has_more_than_thirty_characters"].add(1)

        XCTAssertEqual(
            4,
            labeledCounterMetric.testGetNumRecordedErrors(.invalidLabel)
        )

        XCTAssertEqual(4, try labeledCounterMetric["__other__"].testGetValue())
    }

    func testLabeledStringType() {
        let counterMetric = StringMetricType(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

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

        XCTAssertEqual("foo", try labeledStringMetric["label1"].testGetValue())
        XCTAssertEqual("bar", try labeledStringMetric["label2"].testGetValue())
    }

    func testLabeledBooleanType() {
        let booleanMetric = BooleanMetricType(
            category: "telemetry",
            name: "labeled_boolean_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

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

        XCTAssertEqual(false, try labeledBooleanMetric["label1"].testGetValue())
        XCTAssertEqual(true, try labeledBooleanMetric["label2"].testGetValue())
    }

    func testLabeledEventsThrowAnException() {
        let eventMetric = EventMetricType<NoExtraKeys>(
            category: "telemetry",
            name: "labeled_event",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        )

        XCTAssertThrowsError(try LabeledMetricType<EventMetricType<NoExtraKeys>>(
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
