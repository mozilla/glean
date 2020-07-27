// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using Mozilla.Glean.Testing;
using System;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class LabeledMetricTypeTest
    {
        public LabeledMetricTypeTest()
        {
            // Get a random test directory just for this single test.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // In xUnit, the constructor will be called before each test. This
            // feels like a natural place to initialize / reset Glean.
            GleanInstance.Reset(
	            applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: tempDataDir
                );
        }

        [Fact]
        public void TestLabeledCounterType()
        {
            // TODO: Placeholder. Implement in bug 1648437 by converting the Kotlin test.
        }

        [Fact]
        public void TestOtherLabelWithPredefinedLabels()
        {
            // TODO: Placeholder. Implement in bug 1648437 by converting the Kotlin test.
        }

        [Fact]
        public void TestOtherLabelWithoutPredefinedLabels()
        {
            // TODO: Placeholder. Implement in bug 1648437 by converting the Kotlin test.
        }

        [Fact]
        public void TestOtherLabelWithoutPredefinedLabelsBeforeGleanInits()
        {
            // TODO: Placeholder. Implement in bug 1648437 by converting the Kotlin test.
        }

        [Fact]
        public void EnsureInvalidLabelsOnLabeledCounterGoToOther()
        {
            // TODO: Placeholder. Implement in bug 1648437 by converting the Kotlin test.
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
