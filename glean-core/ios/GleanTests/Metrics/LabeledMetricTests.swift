/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

@testable import Glean
import XCTest

// swiftlint:disable type_body_length
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
            subMetric: counterMetric
        )

        // These are fine, now.
        labeledCounterMetric["notSnakeCase"].add(1)
        labeledCounterMetric[""].add(1)
        labeledCounterMetric["with/slash"].add(1)
        labeledCounterMetric["this_string_has_more_than_thirty_characters"].add(1)
        labeledCounterMetric["Spøøn"].add(1)

        XCTAssertEqual(
            0,
            labeledCounterMetric.testGetNumRecordedErrors(.invalidLabel)
        )

        XCTAssertEqual(nil, labeledCounterMetric["__other__"].testGetValue())

        // More than 111 characters? Not okay.
        labeledCounterMetric[String(repeating: "1", count: 112)].add(1)
        XCTAssertEqual(
            1,
            labeledCounterMetric.testGetNumRecordedErrors(.invalidLabel)
        )

        XCTAssertEqual(1, labeledCounterMetric["__other__"].testGetValue())
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_boolean_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_quantity_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
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
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_event_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
            subMetric: eventMetric
        )) { error in
            XCTAssertEqual(error as! String, "Can not create a labeled version of this metric type")
        }
    }

    func testLabeledMetricTestGetLabeledValues() {
        let counterMetric = CounterMetricType(CommonMetricData(
            category: "telemetry",
            name: "labeled_counter_metric",
            sendInPings: ["metrics"],
            lifetime: .application,
            disabled: false
        ))

        let labeledCounterMetric = try! LabeledMetricType<CounterMetricType>(
            .common(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_counter_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                )
            ),
            subMetric: counterMetric
        )

        labeledCounterMetric["label1"].add(1)
        labeledCounterMetric["label2"].add(2)

        let labeledValues = labeledCounterMetric.testGetValue()
        XCTAssertEqual(2, labeledValues.count)
        XCTAssertEqual(1, labeledValues["label1"] as! Int32)
        XCTAssertEqual(2, labeledValues["label2"] as! Int32)
    }

    func testLabeledMemoryDistribution() {
        let metric = MemoryDistributionMetricType(
            CommonMetricData(
                category: "telemetry",
                name: "labeled_memory_distribution_metric",
                sendInPings: ["metrics"],
                lifetime: .application,
                disabled: false
            ),
            .kilobyte
        )

        let labeledMetric = try! LabeledMetricType<MemoryDistributionMetricType>(
            .memoryDistribution(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_memory_distribution_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                ),
                unit: .kilobyte
            ),
            subMetric: metric
        )

        labeledMetric["label1"].accumulate(1)
        labeledMetric["label2"].accumulate(2)

        let kb = Int64(1024)

        XCTAssertEqual(1 * kb, labeledMetric["label1"].testGetValue()!.sum)
        XCTAssertEqual(2 * kb, labeledMetric["label2"].testGetValue()!.sum)
    }

    func testLabeledTimingDistribution() {
        let metric = TimingDistributionMetricType(
            CommonMetricData(
                category: "telemetry",
                name: "labeled_timing_distribution_metric",
                sendInPings: ["metrics"],
                lifetime: .application,
                disabled: false
            ),
            .nanosecond
        )

        let labeledMetric = try! LabeledMetricType<TimingDistributionMetricType>(
            .timingDistribution(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_timing_distribution_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                ),
                unit: .nanosecond
            ),
            subMetric: metric
        )

        var id = labeledMetric["label1"].start()
        labeledMetric["label1"].stopAndAccumulate(id)

        id = labeledMetric["label2"].start()
        labeledMetric["label2"].stopAndAccumulate(id)

        id = labeledMetric["label2"].start()
        labeledMetric["label2"].stopAndAccumulate(id)

        XCTAssertEqual(1, labeledMetric["label1"].testGetValue()!.count)
        XCTAssertEqual(2, labeledMetric["label2"].testGetValue()!.count)
    }

    func testLabeledCustomDistribution() {
        let metric = CustomDistributionMetricType(
            CommonMetricData(
                category: "telemetry",
                name: "labeled_custom_distribution_metric",
                sendInPings: ["metrics"],
                lifetime: .application,
                disabled: false
            ),
            0,
            60000,
            100,
            .exponential,
        )

        let labeledMetric = try! LabeledMetricType<CustomDistributionMetricType>(
            .customDistribution(
                cmd: CommonMetricData(
                    category: "telemetry",
                    name: "labeled_custom_distribution_metric",
                    sendInPings: ["metrics"],
                    lifetime: .application,
                    disabled: false,
                ),
                rangeMin: 0,
                rangeMax: 60000,
                bucketCount: 100,
                histogramType: .exponential,
            ),
            subMetric: metric
        )

        labeledMetric["label1"].accumulateSamples([1])
        labeledMetric["label2"].accumulateSamples([2])
        labeledMetric["label2"].accumulateSamples([3])

        XCTAssertEqual(1, labeledMetric["label1"].testGetValue()!.count)
        XCTAssertEqual(2, labeledMetric["label2"].testGetValue()!.count)
    }
}
// swiftlint:enable type_body_length
