// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class BooleanMetricTypeTest
    {
        public BooleanMetricTypeTest()
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
            Private.BooleanMetricType booleanMetric = new Private.BooleanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "boolean_metric",
                sendInPings: new string[] { "store1" }
            );

            // Record two strings of the same type, with a little delay.
            booleanMetric.Set(true);

            // Check that data was properly recorded.
            Assert.True(booleanMetric.TestHasValue());
            Assert.True(booleanMetric.TestGetValue());

            booleanMetric.Set(true);

            // Check that data was properly recorded.
            Assert.True(booleanMetric.TestHasValue());
            Assert.True(booleanMetric.TestGetValue());
        }

        [Fact]
        public void DisabledstringsMustNotRecordData()
        {
            Private.BooleanMetricType booleanMetric = new Private.BooleanMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "boolean_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the string.
            booleanMetric.Set(true);
            // Check that nothing was recorded.
            Assert.False(booleanMetric.TestHasValue(), "Booleans must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.BooleanMetricType booleanMetric = new Private.BooleanMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "boolean_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => booleanMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            Private.BooleanMetricType booleanMetric = new Private.BooleanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "boolean_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Record two strings of the same type, with a little delay.
            booleanMetric.Set(true);

            // Check that data was properly recorded in the second ping.
            Assert.True(booleanMetric.TestHasValue("store2"));
            Assert.True(booleanMetric.TestGetValue("store2"));

            booleanMetric.Set(false);
            // Check that data was properly recorded in the second ping.
            Assert.True(booleanMetric.TestHasValue("store2"));
            Assert.False(booleanMetric.TestGetValue("store2"));
        }
    }
}
