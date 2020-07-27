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
    public class CounterMetricTypeTest
    {
        public CounterMetricTypeTest()
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
            Private.CounterMetricType counterMetric = new Private.CounterMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "counter_metric",
                sendInPings: new string[] { "store1" }
            );

            Assert.False(counterMetric.TestHasValue());

            // Add to the counter a couple of times with a little delay.  The first call will check
            // calling add() without parameters to test increment by 1.
            counterMetric.Add();

            // Check that the count was incremented and properly recorded.
            Assert.True(counterMetric.TestHasValue());
            Assert.Equal(1, counterMetric.TestGetValue());

            counterMetric.Add(10);
            // Check that count was incremented and properly recorded.  This second call will check
            // calling add() with 10 to test increment by other amount
            Assert.True(counterMetric.TestHasValue());
            Assert.Equal(11, counterMetric.TestGetValue());
        }

        [Fact]
        public void DisabledCountersMustNotRecordData()
        {
            Private.CounterMetricType counterMetric = new Private.CounterMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "counter_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the counter.
            counterMetric.Add();
            // Check that nothing was recorded.
            Assert.False(counterMetric.TestHasValue(), "Counters must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.CounterMetricType counterMetric = new Private.CounterMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "counter_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => counterMetric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            Private.CounterMetricType counterMetric = new Private.CounterMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "counter_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Add to the counter a couple of times with a little delay.  The first call will check
            // calling add() without parameters to test increment by 1.
            counterMetric.Add();

            // Check that the count was incremented and properly recorded for the second ping.
            Assert.True(counterMetric.TestHasValue("store2"));
            Assert.Equal(1, counterMetric.TestGetValue("store2"));

            counterMetric.Add(10);
            // Check that count was incremented and properly recorded for the second ping.
            // This second call will check calling add() with 10 to test increment by other amount
            Assert.True(counterMetric.TestHasValue("store2"));
            Assert.Equal(11, counterMetric.TestGetValue("store2"));
        }

        [Fact]
        public void NegativeValuesAreNotCounted()
        {
            Private.CounterMetricType counterMetric = new Private.CounterMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "counter_metric",
                sendInPings: new string[] { "store1" }
            );

            // Increment to 1 (initial value)
            counterMetric.Add();

            // Check that the count was incremented
            Assert.True(counterMetric.TestHasValue("store1"));
            Assert.Equal(1, counterMetric.TestGetValue("store1"));

            counterMetric.Add(-10);
            // Check that count was NOT incremented.
            Assert.True(counterMetric.TestHasValue("store1"));
            Assert.Equal(1, counterMetric.TestGetValue("store1"));
            Assert.Equal(1, counterMetric.TestGetNumRecordedErrors(ErrorType.InvalidValue));
        }
}
}
