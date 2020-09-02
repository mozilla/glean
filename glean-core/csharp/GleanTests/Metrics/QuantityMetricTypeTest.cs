// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Testing;
using System;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class QuantityMetricTypeTest
    {
        public QuantityMetricTypeTest()
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
        public void APISavesToStorage()
        {
            Private.QuantityMetricType quantityMetric = new Private.QuantityMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "quantity_metric",
                sendInPings: new string[] { "store1" }
            );

            Assert.False(quantityMetric.TestHasValue());

            quantityMetric.Set(1);

            // Check that the metric was properly recorded.
            Assert.True(quantityMetric.TestHasValue());
            Assert.Equal(1, quantityMetric.TestGetValue());

            quantityMetric.Set(10);
            // Check that the metric was properly overwritten.
            Assert.True(quantityMetric.TestHasValue());
            Assert.Equal(10, quantityMetric.TestGetValue());
        }

        [Fact]
        public void DisabledCountersMustNotRecordData()
        {
            Private.QuantityMetricType quantityMetric = new Private.QuantityMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "quantity_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the quantity.
            quantityMetric.Set(1);
            // Check that nothing was recorded.
            Assert.False(quantityMetric.TestHasValue(), "Quantities must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.QuantityMetricType quantityMetric = new Private.QuantityMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "quantity_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => quantityMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            Private.QuantityMetricType quantityMetric = new Private.QuantityMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "quantity_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            quantityMetric.Set(1);

            // Check that the metric was properly recorded for the secondary ping.
            Assert.True(quantityMetric.TestHasValue("store2"));
            Assert.Equal(1, quantityMetric.TestGetValue("store2"));

            quantityMetric.Set(10);
            // Check that the metric was properly overwritten for the secondary ping.
            Assert.True(quantityMetric.TestHasValue("store2"));
            Assert.Equal(10, quantityMetric.TestGetValue("store2"));
        }

        [Fact]
        public void NegativeValuesAreNotRecorded()
        {
            Private.QuantityMetricType quantityMetric = new Private.QuantityMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "quantity_metric",
                sendInPings: new string[] { "store1" }
            );

            quantityMetric.Set(-10);
            // Check that quantity was NOT recorded.
            Assert.False(quantityMetric.TestHasValue("store1"));
            Assert.Equal(1, quantityMetric.TestGetNumRecordedErrors(ErrorType.InvalidValue));
        }
}
}
