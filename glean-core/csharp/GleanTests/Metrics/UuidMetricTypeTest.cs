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
    public sealed class UuidMetricTypeTest
    {
        public UuidMetricTypeTest()
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
            // Define a 'uuidMetric' uuid metric, which will be stored in "store1"
            var uuidMetric = new Private.UuidMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "uuid_metric",
                sendInPings: new string[] { "store1" }
            );

            // Check that there is no UUID recorded
            Assert.False(uuidMetric.TestHasValue());

            // Record two uuids of the same type, with a little delay.
            var uuid = uuidMetric.GenerateAndSet();

            // Check that data was properly recorded.
            Assert.True(uuidMetric.TestHasValue());
            Assert.Equal(uuid, uuidMetric.TestGetValue());

            var uuid2 = System.Guid.NewGuid();
            uuidMetric.Set(uuid2);

            // Check that data was properly recorded.
            Assert.True(uuidMetric.TestHasValue());
            Assert.Equal(uuid2, uuidMetric.TestGetValue());
        }

        [Fact]
        public void DisabledstringsMustNotRecordData()
        {
            // Define a 'uuidMetric' uuid metric, which will be stored in "store1". It's disabled
            // so it should not record anything.
            var uuidMetric = new Private.UuidMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "uuid_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the uuid.
            uuidMetric.GenerateAndSet();

            // Check that nothing was recorded.
            Assert.False(uuidMetric.TestHasValue(),
                "Uuids must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            var uuidMetric = new Private.UuidMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "uuid_metric",
                sendInPings: new string[] { "store1" }
            );

            Assert.Throws<NullReferenceException>(() => uuidMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            // Define a 'uuidMetric' uuid metric, which will be stored in "store1" and "store2"
            var uuidMetric = new Private.UuidMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "uuid_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Record two uuids of the same type, with a little delay.
            var uuid = uuidMetric.GenerateAndSet();

            // Check that data was properly recorded.
            Assert.True(uuidMetric.TestHasValue("store2"));
            Assert.Equal(uuid, uuidMetric.TestGetValue("store2"));

            var uuid2 = System.Guid.NewGuid();
            uuidMetric.Set(uuid2);

            // Check that data was properly recorded.
            Assert.True(uuidMetric.TestHasValue("store2"));
            Assert.Equal(uuid2, uuidMetric.TestGetValue("store2"));
        }
    }
}
