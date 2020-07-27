// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using Mozilla.Glean.Testing;
using System;
using System.Collections.Generic;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class LabeledMetricTypeTest
    {
        // Get a random test directory just for this single test.
        string TempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

        public LabeledMetricTypeTest()
        {
            // In xUnit, the constructor will be called before each test. This
            // feels like a natural place to initialize / reset Glean.
            GleanInstance.Reset(
	            applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: TempDataDir
                );
        }

        [Fact]
        public void TestLabeledCounterType()
        {
            CounterMetricType counterMetric = new CounterMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledCounterMetric = new LabeledMetricType<CounterMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" },
                submetric: counterMetric
            );

            labeledCounterMetric["label1"].Add(1);
            labeledCounterMetric["label2"].Add(2);

            // Record a regular non-labeled counter. This isn't normally
            // possible with the generated code because the subMetric is private,
            // but it's useful to test here that it works.
            counterMetric.Add(3);

            Assert.True(labeledCounterMetric["label1"].TestHasValue());
            Assert.Equal(1, labeledCounterMetric["label1"].TestGetValue());

            Assert.True(labeledCounterMetric["label2"].TestHasValue());
            Assert.Equal(2, labeledCounterMetric["label2"].TestGetValue());

            Assert.True(counterMetric.TestHasValue());
            Assert.Equal(3, counterMetric.TestGetValue());
        }

        [Fact]
        public void TestOtherLabelWithPredefinedLabels()
        {
            CounterMetricType counterMetric = new CounterMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledCounterMetric = new LabeledMetricType<CounterMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" },
                submetric: counterMetric,
                labels: new HashSet<string>() { "foo", "bar", "baz" }
            );

            labeledCounterMetric["foo"].Add(1);
            labeledCounterMetric["foo"].Add(2);
            labeledCounterMetric["bar"].Add(1);
            labeledCounterMetric["not_there"].Add(1);
            labeledCounterMetric["also_not_there"].Add(1);
            labeledCounterMetric["not_me"].Add(1);

            Assert.Equal(3, labeledCounterMetric["foo"].TestGetValue());
            Assert.Equal(1, labeledCounterMetric["bar"].TestGetValue());
            Assert.False(labeledCounterMetric["baz"].TestHasValue());
            // The rest all lands in the __other__ bucket
            Assert.Equal(3, labeledCounterMetric["not_there"].TestGetValue());
        }

        [Fact]
        public void TestOtherLabelWithoutPredefinedLabels()
        {
            CounterMetricType counterMetric = new CounterMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledCounterMetric = new LabeledMetricType<CounterMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" },
                submetric: counterMetric
            );

            for (int i = 0; i <= 20; i++)
            {
                labeledCounterMetric[$"label_{i}"].Add(1);
            }
            // Go back and record in one of the real labels again
            labeledCounterMetric["label_0"].Add(1);

            Assert.Equal(2, labeledCounterMetric["label_0"].TestGetValue());
            for (int i = 1; i <= 15; i++)
            {
                Assert.Equal(1, labeledCounterMetric[$"label_{i}"].TestGetValue());
            }
            Assert.Equal(5, labeledCounterMetric["__other__"].TestGetValue());
        }

        [Fact]
        public void TestOtherLabelWithoutPredefinedLabelsBeforeGleanInits()
        {
            CounterMetricType counterMetric = new CounterMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledCounterMetric = new LabeledMetricType<CounterMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" },
                submetric: counterMetric
            );

            // Make sure Glean isn't initialized, and turn task queueing on
            GleanInstance.TestDestroyGleanHandle();
            Dispatchers.QueueInitialTasks = true;

            for (int i = 0; i <= 20; i++)
            {
                labeledCounterMetric[$"label_{i}"].Add(1);
            }
            // Go back and record in one of the real labels again
            labeledCounterMetric["label_0"].Add(1);

            // Initialize glean
            GleanInstance.Initialize(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: TempDataDir
                );

            Assert.Equal(2, labeledCounterMetric["label_0"].TestGetValue());
            for (int i = 1; i <= 15; i++)
            {
                Assert.Equal(1, labeledCounterMetric[$"label_{i}"].TestGetValue());
            }
            Assert.Equal(5, labeledCounterMetric["__other__"].TestGetValue());
        }

        [Fact]
        public void EnsureInvalidLabelsOnLabeledCounterGoToOther()
        {
            CounterMetricType counterMetric = new CounterMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledCounterMetric = new LabeledMetricType<CounterMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_counter_metric",
                sendInPings: new string[] { "metrics" },
                submetric: counterMetric
            );

            labeledCounterMetric["notSnakeCase"].Add(1);
            labeledCounterMetric[""].Add(1);
            labeledCounterMetric["with/slash"].Add(1);
            labeledCounterMetric["this_string_has_more_than_thirty_characters"].Add(1);

            Assert.Equal(
                4,
                labeledCounterMetric.TestGetNumRecordedErrors(
                    ErrorType.InvalidLabel
                )
            );
            Assert.Equal(
                4,
                labeledCounterMetric["__other__"].TestGetValue()
            );
        }

        [Fact]
        public void EnsureInvalidLabelsOnLabeledBooleanGoToOther()
        {
            BooleanMetricType booleanMetric = new BooleanMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_boolean_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledBooleanMetric = new LabeledMetricType<BooleanMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_boolean_metric",
                sendInPings: new string[] { "metrics" },
                submetric: booleanMetric
            );

            labeledBooleanMetric["notSnakeCase"].Set(true);
            labeledBooleanMetric[""].Set(true);
            labeledBooleanMetric["with/slash"].Set(true);
            labeledBooleanMetric["this_string_has_more_than_thirty_characters"].Set(true);

            Assert.Equal(
                4,
                labeledBooleanMetric.TestGetNumRecordedErrors(
                    ErrorType.InvalidLabel
                )
            );
            Assert.True(
                labeledBooleanMetric["__other__"].TestGetValue()
            );
        }

        [Fact]
        public void EnsureInvalidLabelsOnLabeledStringGoToOther()
        {
            StringMetricType stringMetric = new StringMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_string_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledStringMetric = new LabeledMetricType<StringMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_string_metric",
                sendInPings: new string[] { "metrics" },
                submetric: stringMetric
            );

            labeledStringMetric["notSnakeCase"].Set("foo");
            labeledStringMetric[""].Set("foo");
            labeledStringMetric["with/slash"].Set("foo");
            labeledStringMetric["this_string_has_more_than_thirty_characters"].Set("foo");

            Assert.Equal(
                4,
                labeledStringMetric.TestGetNumRecordedErrors(
                    ErrorType.InvalidLabel
                )
            );
            Assert.Equal(
                "foo",
                labeledStringMetric["__other__"].TestGetValue()
            );
        }

        [Fact]
        public void TestLabeledStringMetricType()
        {
            StringMetricType stringMetric = new StringMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_string_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledStringMetric = new LabeledMetricType<StringMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_string_metric",
                sendInPings: new string[] { "metrics" },
                submetric: stringMetric
            );

            labeledStringMetric["label1"].Set("foo");
            labeledStringMetric["label2"].Set("bar");

            Assert.Equal("foo", labeledStringMetric["label1"].TestGetValue());
            Assert.Equal("bar", labeledStringMetric["label2"].TestGetValue());
        }

        [Fact]
        public void TestLabeledBooleanMetricType()
        {
            BooleanMetricType booleanMetric = new BooleanMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_boolean_metric",
                sendInPings: new string[] { "metrics" }
            );

            var labeledBooleanMetric = new LabeledMetricType<BooleanMetricType>(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Application,
                name: "labeled_boolean_metric",
                sendInPings: new string[] { "metrics" },
                submetric: booleanMetric
            );

            labeledBooleanMetric["label1"].Set(false);
            labeledBooleanMetric["label2"].Set(true);

            Assert.False(labeledBooleanMetric["label1"].TestGetValue());
            Assert.True(labeledBooleanMetric["label2"].TestGetValue());
        }

        [Fact]
        public void TestLabeledEventsAreAnException()
        {
            // TODO: Placeholder. Implement in bug 1648422 by converting the related Kotlin test.
        }

        // SKIPPED `test recording to static labels by label index` (Kotlin) because labels by label
        // index are not supported in C# (only useful on Android for project EXTRACT).
    }
}
