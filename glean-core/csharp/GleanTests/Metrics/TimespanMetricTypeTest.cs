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
    public class TimespanMetricTypeTest
    {
        public TimespanMetricTypeTest()
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
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Record a timespan.
            metric.Start();
            metric.Stop();

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue());
            Assert.True(metric.TestGetValue() >= 0);
        }

        [Fact]
        public void DisabledTimespansMustNotRecordData()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Record a timespan.
            metric.Start();
            metric.Stop();

            // Let's also call cancel() to make sure it's a no-op.
            metric.Cancel();

            // Check that data was not recorded.
            Assert.False(metric.TestHasValue(), "The API should not record a counter if metric is disabled");
        }

        [Fact]
        public void APIMustCorrectlyCancel()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Record a timespan.
            metric.Start();
            metric.Cancel();
            metric.Stop();

            // Check that data was not recorded.
            Assert.False(metric.TestHasValue(), "The API should not record an error if the metric is cancelled");
            Assert.Equal(1, metric.TestGetNumRecordedErrors(ErrorType.InvalidState));
        }

        [Fact]
        public void TestGetValueThrows()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            Assert.Throws<NullReferenceException>(() => metric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1", "store2" },
                timeUnit: TimeUnit.Millisecond
            );

            metric.Start();
            metric.Stop();

            // Check that data was properly recorded in the second ping.
            Assert.True(metric.TestHasValue("store2"));
            Assert.True(metric.TestGetValue("store2") >= 0);
        }

        [Fact]
        public void RecordsAnErrorIfStartedTwice()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1", "store2" },
                timeUnit: TimeUnit.Millisecond
            );

            // Record a timespan.
            metric.Start();
            metric.Start();
            metric.Stop();

            // Check that data was properly recorded in the second ping.
            Assert.True(metric.TestHasValue("store2"));
            Assert.True(metric.TestGetValue("store2") >= 0);
            Assert.Equal(1, metric.TestGetNumRecordedErrors(ErrorType.InvalidState));
        }

        [Fact]
        public void ValueUnchangedIfStoppedTwice()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Record a timespan.
            metric.Start();
            metric.Stop();
            Assert.True(metric.TestHasValue());
            ulong value = metric.TestGetValue();

            metric.Stop();

            Assert.Equal(value, metric.TestGetValue());
        }

        [Fact]
        public void TestSetRawNanos()
        {
            ulong timespanNanos = 6 * 1000000000L;

            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Ping,
                name: "explicit_timespan",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            metric.SetRawNanos(timespanNanos);
            Assert.Equal<ulong>(6, metric.TestGetValue());
        }

        [Fact]
        public void TestSetRawNanosFollowedByOtherAPI()
        {
            ulong timespanNanos = 6 * 1000000000L;

            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Ping,
                name: "explicit_timespan",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            metric.SetRawNanos(timespanNanos);
            Assert.Equal<ulong>(6, metric.TestGetValue());

            metric.Start();
            metric.Stop();
            ulong value = metric.TestGetValue();
            Assert.Equal<ulong>(6, value);
        }

        [Fact]
        public void SetRawNanosDoesNotOverwriteValue()
        {
            ulong timespanNanos = 6 * 1000000000L;
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Ping,
                name: "explicit_timespan_1",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            metric.Start();
            metric.Stop();
            ulong value = metric.TestGetValue();

            metric.SetRawNanos(timespanNanos);

            Assert.Equal<ulong>(value, metric.TestGetValue());
        }

        [Fact]
        public void SetRawNanosDoesNothingWhenTimerIsRunning()
        {
            ulong timespanNanos = 1000000000L;

            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Ping,
                name: "explicit_timespan",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            metric.Start();
            metric.SetRawNanos(timespanNanos);
            metric.Stop();

            // If setRawNanos worked, (which it's not supposed to in this case), it would
            // have recorded 1000000000 ns == 1s.  Make sure it's not that.
            Assert.NotEqual<ulong>(1, metric.TestGetValue());
        }

        [Fact]
        public void MeasureFunctionCorrectlyMeasuresValues()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Create a function to measure, which also returns a value to test that we properly pass
            // along the returned value from the measure function
            static bool TestFunc(bool value) => value;

            // Capture returned value to determine if the function return value matches what is expected
            // and measure the test function, which should record to the metric
            bool testValue = metric.Measure(() => TestFunc(true));

            // Make sure the returned valued matches the expected value of "true"
            Assert.True(testValue, "Test value must match");

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue(), "Metric must have a value");
            Assert.True(metric.TestGetValue() >= 0, "Metric value must be greater than zero");
        }

        [Fact]
        public void MeasureFunctionBubblesUpExceptionsAndTimingIsCanceled()
        {
            TimespanMetricType metric = new TimespanMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Lifetime.Application,
                name: "timespan_metric",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Millisecond
            );

            // Create a function that will throw a NPE
            static bool TestFunc()
            {
                throw new NullReferenceException();
            }

            // Attempt to measure the function that will throw an exception.  The `Measure` function
            // should allow the exception to bubble up, the timespan measurement is canceled.
            try {
                metric.Measure(() => TestFunc());
            } catch (Exception e) {
                // Make sure we caught the right kind of exception: NPE
                Assert.True(e is NullReferenceException, "Exception type must match");
            } finally {
                Assert.True(!metric.TestHasValue(), "Metric must not have a value");
            }
        }
    }
}
