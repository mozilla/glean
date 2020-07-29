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
    public class StringListMetricTypeTest
    {
        public StringListMetricTypeTest()
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
        public void APISavesToStorageByAddingThenSetting()
        {
            // Define a 'stringListMetric' string list metric, which will be stored in "store1".
            var stringListMetric = new Private.StringListMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1" }
            );

            // Record two lists using add and set.
            stringListMetric.Add("value1");
            stringListMetric.Add("value2");
            stringListMetric.Add("value3");

            // Check that data was properly recorded.
            var snapshot = stringListMetric.TestGetValue();
            Assert.Equal(3, snapshot.Length);
            Assert.True(stringListMetric.TestHasValue());
            Assert.Equal("value1", snapshot[0]);
            Assert.Equal("value2", snapshot[1]);
            Assert.Equal("value3", snapshot[2]);

            // Use Set() to see that the first list is replaced by the new list.
            stringListMetric.Set(new string[]{ "other1", "other2", "other3"});
            // Check that data was properly recorded.
            var snapshot2 = stringListMetric.TestGetValue();
            Assert.Equal(3, snapshot2.Length);
            Assert.True(stringListMetric.TestHasValue());
            Assert.Equal("other1", snapshot2[0]);
            Assert.Equal("other2", snapshot2[1]);
            Assert.Equal("other3", snapshot2[2]);
        }

        [Fact]
        public void APISavesToStorageBySettingThenAdding()
        {
            // Define a 'stringListMetric' string list metric, which will be stored in "store1".
            var stringListMetric = new Private.StringListMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1" }
            );

            // Record two lists using set and add.
            stringListMetric.Set(new string[] { "value1", "value2", "value3" });

            // Check that data was properly recorded.
            var snapshot = stringListMetric.TestGetValue();
            Assert.Equal(3, snapshot.Length);
            Assert.True(stringListMetric.TestHasValue());
            Assert.Equal("value1", snapshot[0]);
            Assert.Equal("value2", snapshot[1]);
            Assert.Equal("value3", snapshot[2]);

            // Use Add() to see that the list is appended to.
            stringListMetric.Add("added1");
            // Check that data was properly recorded.
            var snapshot2 = stringListMetric.TestGetValue();
            Assert.Equal(4, snapshot2.Length);
            Assert.True(stringListMetric.TestHasValue());
            Assert.Equal("value1", snapshot2[0]);
            Assert.Equal("value2", snapshot2[1]);
            Assert.Equal("value3", snapshot2[2]);
            Assert.Equal("added1", snapshot2[3]);
        }

        [Fact]
        public void DisabledListMustNotRecordData()
        {
            // Define a 'stringListMetric' string list metric, which will be stored in "store1".
            // It's disabled so it should not record anything.
            var stringListMetric = new Private.StringListMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the string list using set.
            stringListMetric.Set(new string[] { "value1", "value2", "value3" });
            // Check that nothing was recorded.
            // StringLists must not be recorded if they are disabled.
            Assert.False(stringListMetric.TestHasValue());

            // Attempt to store the string list using add.
            stringListMetric.Add("value4");
            // Check that nothing was recorded.
            // StringLists must not be recorded if they are disabled.
            Assert.False(stringListMetric.TestHasValue());
        }

        [Fact]
        public void TestGetValueThrows()
        {
            var stringListMetric = new Private.StringListMetricType(
                disabled: true,
                category: "telemetry",
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => stringListMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            // Define a 'stringListMetric' string list metric, which will be stored in "store1".
            var stringListMetric = new Private.StringListMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Record two lists using Add() and Set().
            stringListMetric.Add("value1");
            stringListMetric.Add("value2");
            stringListMetric.Add("value3");

            // Check that data was properly recorded in the second ping.
            Assert.True(stringListMetric.TestHasValue("store2"));
            var snapshot = stringListMetric.TestGetValue("store2");
            Assert.Equal(3, snapshot.Length);
            Assert.Equal("value1", snapshot[0]);
            Assert.Equal("value2", snapshot[1]);
            Assert.Equal("value3", snapshot[2]);

            // Use Set() to see that the first list is replaced by the new list.
            stringListMetric.Set(new string[] { "other1", "other2", "other3" });
            // Check that data was properly recorded in the second ping.
            Assert.True(stringListMetric.TestHasValue("store2"));
            var snapshot2 = stringListMetric.TestGetValue("store2");
            Assert.Equal(3, snapshot2.Length);
            Assert.Equal("other1", snapshot2[0]);
            Assert.Equal("other2", snapshot2[1]);
            Assert.Equal("other3", snapshot2[2]);
        }

        [Fact]
        public void LongStringListsAreTruncated()
        {
            // Define a 'stringListMetric' string list metric, which will be stored in "store1".
            var stringListMetric = new Private.StringListMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Private.Lifetime.Application,
                name: "string_list_metric",
                sendInPings: new string[] { "store1" }
            );

            for (int x = 0; x <= 20; x++)
            {
                stringListMetric.Add("value" + x);
            }

            var snapshot = stringListMetric.TestGetValue("store1");
            Assert.Equal(20, snapshot.Length);

            Assert.Equal(1, stringListMetric.TestGetNumRecordedErrors(ErrorType.InvalidValue));
        }
    }
}
