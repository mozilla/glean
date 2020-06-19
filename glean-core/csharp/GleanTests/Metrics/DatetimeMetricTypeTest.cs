// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using System;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class DatetimeMetricTypeTest
    {
        public DatetimeMetricTypeTest()
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
            var datetimeMetric = new Private.DatetimeMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "datetime_metric",
                sendInPings: new string[] { "store1" }
            );

            string displayName = "(UTC-08:00) Pacific Time (US & Canada)";
            string standardName = "America/Los_Angeles";
            var offset = new TimeSpan(-08, 00, 00);
            var timeZone = TimeZoneInfo.CreateCustomTimeZone(standardName, offset, displayName, standardName);
            var value = new DateTimeOffset(2004, 11, 9, 8, 3, 29, timeZone.BaseUtcOffset);
            datetimeMetric.Set(value);
            Assert.True(datetimeMetric.TestHasValue());
            Assert.Equal("2004-11-09T08:03-08:00", datetimeMetric.TestGetValueAsString());

            displayName = "(UTC+00:00) Dublin, Edinburgh, Lisbon, London";
            standardName = "GMT+0";
            offset = new TimeSpan(00, 00, 00);
            timeZone = TimeZoneInfo.CreateCustomTimeZone(standardName, offset, displayName, standardName);
            var value2 = new DateTimeOffset(1993, 1, 23, 9, 5, 43, timeZone.BaseUtcOffset);
            datetimeMetric.Set(value2);
            // Check that data was properly recorded.
            Assert.True(datetimeMetric.TestHasValue());
            Assert.Equal("1993-01-23T09:05+00:00", datetimeMetric.TestGetValueAsString());

            // A date prior to the UNIX epoch
            displayName = "(UTC-12:00) International Date Line West";
            standardName = "GMT-12";
            offset = new TimeSpan(-12, 00, 00);
            timeZone = TimeZoneInfo.CreateCustomTimeZone(standardName, offset, displayName, standardName);
            var value3 = new DateTimeOffset(1969, 7, 20, 20, 17, 3, timeZone.BaseUtcOffset);
            datetimeMetric.Set(value3);
            // Check that data was properly recorded.
            Assert.True(datetimeMetric.TestHasValue());
            Assert.Equal("1969-07-20T20:17-12:00", datetimeMetric.TestGetValueAsString());

            // A date following 2038 (the extent of signed 32-bits after UNIX epoch)
            // This fails on some workers on Taskcluster.  32-bit platforms, perhaps?

            // val value4 = Calendar.getInstance()
            // value4.set(2039, 7, 20, 20, 17, 3)
            // datetimeMetric.set(value4)
            // // Check that data was properly recorded.
            // assertTrue(datetimeMetric.testHasValue())
            // assertEquals("2039-08-20T20:17:03-04:00", datetimeMetric.testGetValueAsString())
        }

        [Fact]
        public void DisabledDatetimesMustNotRecordData()
        {
            var datetimeMetric = new Private.DatetimeMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "datetime_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the datetime.
            datetimeMetric.Set();
            Assert.False(datetimeMetric.TestHasValue());
        }

        [Fact]
        public void SettingDateAndReadingResultsInTheSame()
        {
            var datetimeMetric = new Private.DatetimeMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "datetime_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            var unixStart = new DateTime(1970, 1, 1, 0, 0, 0, 0);
            var now = new DateTime(DateTime.Now.Ticks);
            datetimeMetric.Set(now);
            Assert.Equal(Math.Floor((now.ToUniversalTime() - unixStart).TotalSeconds),
                datetimeMetric.TestGetValue().ToUnixTimeSeconds());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            // Define a 'datetimeMetric' datetime metric, which will be stored in "store1" and "store2"
            var datetimeMetric = new Private.DatetimeMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "datetime_metric",
                sendInPings: new string[] { "store1", "store2" },
                timeUnit: TimeUnit.Second
            );

            const string displayName = "(UTC-06:00) Central Time (US & Canada)";
            const string standardName = "Central Standard Time";
            var offset = new TimeSpan(-06, 00, 00);
            var timeZone = TimeZoneInfo.CreateCustomTimeZone(standardName, offset, displayName, standardName);
            var value = new DateTimeOffset(2010, 11, 29, 18, 3, 35, timeZone.BaseUtcOffset);
            datetimeMetric.Set(value);

            // Check that data was properly recorded.
            Assert.True(datetimeMetric.TestHasValue("store1"));
            Assert.Equal(value, datetimeMetric.TestGetValue("store1"));
            Assert.True(datetimeMetric.TestHasValue("store2"));
            Assert.Equal(value, datetimeMetric.TestGetValue("store2"));
        }
    }
}
