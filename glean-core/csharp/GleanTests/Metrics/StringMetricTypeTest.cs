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
    public class StringMetricTypeTest
    {
        public StringMetricTypeTest()
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
            Private.StringMetricType stringMetric = new Private.StringMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_metric",
                sendInPings: new string[] { "store1" }
            );

            // Record two strings of the same type, with a little delay.
            stringMetric.Set("value");

            // Check that data was properly recorded.
            Assert.True(stringMetric.TestHasValue());
            Assert.Equal("value", stringMetric.TestGetValue());

            stringMetric.Set("overriddenValue");

            // Check that data was properly recorded.
            Assert.True(stringMetric.TestHasValue());
            Assert.Equal("overriddenValue", stringMetric.TestGetValue());
        }

        [Fact]
        public void DisabledstringsMustNotRecordData()
        {
            Private.StringMetricType stringMetric = new Private.StringMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "string_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the string.
            stringMetric.Set("value");
            // Check that nothing was recorded.
            Assert.False(stringMetric.TestHasValue(), "Strings must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.StringMetricType stringMetric = new Private.StringMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "string_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => stringMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            Private.StringMetricType stringMetric = new Private.StringMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Record two strings of the same type, with a little delay.
            stringMetric.Set("value");

            // Check that data was properly recorded in the second ping.
            Assert.True(stringMetric.TestHasValue("store2"));
            Assert.Equal("value", stringMetric.TestGetValue("store2"));

            stringMetric.Set("overriddenValue");
            // Check that data was properly recorded in the second ping.
            Assert.True(stringMetric.TestHasValue("store2"));
            Assert.Equal("overriddenValue", stringMetric.TestGetValue("store2"));
        }

        [Fact]
        public void SettingALongStringRecordsAnError()
        {
            Private.StringMetricType stringMetric = new Private.StringMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            stringMetric.Set(new string('3', 110));

            Assert.Equal(1, stringMetric.TestGetNumRecordedErrors(ErrorType.InvalidOverflow));
        }
    }
}
