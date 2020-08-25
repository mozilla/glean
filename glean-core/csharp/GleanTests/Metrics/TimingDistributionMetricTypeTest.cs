// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using Mozilla.Glean.Testing;
using Mozilla.Glean.Utils;
using System;
using System.Collections.Generic;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class TimingDistributionMetricTypeTest
    {
        public TimingDistributionMetricTypeTest()
        {
            // Get a random test directory just for this single test.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // Make sure to clear the HighPrecisionTimestamp mocked value between tests.
            HighPrecisionTimestamp.MockedValue = null;

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
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Nanosecond
            );

            // Accumulate a few values
            for (ulong i = 1; i <= 3; i++)
            {
                HighPrecisionTimestamp.MockedValue = 0;
                GleanTimerId id = metric.Start();
                HighPrecisionTimestamp.MockedValue = i;
                metric.StopAndAccumulate(id);
            }

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue());
            var snapshot = metric.TestGetValue();
            // Check the sum
            Assert.Equal(6L, snapshot.Sum);
            // Check that the 1L fell into the first bucket (max 1)
            Assert.Equal(1L, snapshot.Values[1]);
            // Check that the 2L fell into the second bucket (max 2)
            Assert.Equal(1L, snapshot.Values[2]);
            // Check that the 3L fell into the third bucket (max 3)
            Assert.Equal(1L, snapshot.Values[3]);
        }

        [Fact]
        public void DisableTimingDistributionsMustNotRecordData()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: true,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Nanosecond
            );

            // Attempt to store the timing distribution using set
            GleanTimerId id = metric.Start();
            metric.StopAndAccumulate(id);

            // Check that nothing was recorded.
            Assert.False(metric.TestHasValue(), "Disabled TimingDistributions should not record data.");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Nanosecond
            );
            Assert.Throws<NullReferenceException>(() => metric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1", "store2", "store3" },
                timeUnit: TimeUnit.Nanosecond
            );

            // Accumulate a few values
            for (ulong i = 1; i <= 3; i++)
            {
                HighPrecisionTimestamp.MockedValue = 0;
                GleanTimerId id = metric.Start();
                HighPrecisionTimestamp.MockedValue = i;
                metric.StopAndAccumulate(id);
            }

            // Check that data was properly recorded in the second ping.
            Assert.True(metric.TestHasValue("store2"));
            var snapshot = metric.TestGetValue("store2");
            // Check the sum
            Assert.Equal(6L, snapshot.Sum);
            // Check that the 1L fell into the first bucket
            Assert.Equal(1L, snapshot.Values[1]);
            // Check that the 2L fell into the second bucket
            Assert.Equal(1L, snapshot.Values[2]);
            // Check that the 3L fell into the third bucket
            Assert.Equal(1L, snapshot.Values[3]);

            // Check that data was properly recorded in the third ping.
            Assert.True(metric.TestHasValue("store3"));
            var snapshot2 = metric.TestGetValue("store3");
            // Check the sum
            Assert.Equal(6L, snapshot2.Sum);
            // Check that the 1L fell into the first bucket
            Assert.Equal(1L, snapshot2.Values[1]);
            // Check that the 2L fell into the second bucket
            Assert.Equal(1L, snapshot2.Values[2]);
            // Check that the 3L fell into the third bucket
            Assert.Equal(1L, snapshot2.Values[3]);
        }

        [Fact]
        public void StartingATimerBeforeInitializationDoesNotCrash()
        {
            GleanInstance.TestDestroyGleanHandle();
            Dispatchers.QueueInitialTasks = true;

            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            GleanTimerId timerId = metric.Start();

            // Start Glean again.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
            GleanInstance.Initialize(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: tempDataDir
            );

            metric.StopAndAccumulate(timerId);
            Assert.True(metric.TestGetValue().Sum >= 0);
        }

        [Fact]
        public void StartingAndStoppingATimerBeforeInitializationDoesNotCrash()
        {
            GleanInstance.TestDestroyGleanHandle();
            Dispatchers.QueueInitialTasks = true;

            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            GleanTimerId timerId = metric.Start();
            metric.StopAndAccumulate(timerId);

            // Start Glean again.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
            GleanInstance.Initialize(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: tempDataDir
            );
            Assert.True(metric.TestGetValue().Sum >= 0);
        }

        [Fact]
        public void StoppingANonExistentTimerRecordsAnError()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            // Hopefully ulong.MaxValue wasn't used already :)
            metric.StopAndAccumulate((GleanTimerId)(ulong.MaxValue));
            Assert.Equal(1, metric.TestGetNumRecordedErrors(ErrorType.InvalidState));
        }

        [Fact]
        public void MeasureFunctionCorrectlyMeasuresValues()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Nanosecond
            );

            // Create a test function to "measure". This works by mocking the timer return
            // value setting it to return a known value to make it easier to validate.
            static ulong TestFunc(ulong value) {
                HighPrecisionTimestamp.MockedValue = value;
                return value;
            }

            // Accumulate a few values
            for (ulong i = 1; i <= 3; i++)
            {
                // Measure the test function, capturing the value to verify we correctly return the
                // value of the underlying function.
                HighPrecisionTimestamp.MockedValue = 0;
                var testValue = metric.Measure(() => TestFunc(i));

                // Returned value must match
                Assert.Equal(i, testValue);
            }

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue());
            var snapshot = metric.TestGetValue();
            // Check the sum
            Assert.Equal(6L, snapshot.Sum);
            // Check that the 1L fell into the first bucket (max 1)
            Assert.Equal(1L, snapshot.Values[1]);
            // Check that the 2L fell into the second bucket (max 2)
            Assert.Equal(1L, snapshot.Values[2]);
            // Check that the 3L fell into the third bucket (max 3)
            Assert.Equal(1L, snapshot.Values[3]);
        }

        [Fact]
        public void MeasureFunctionBubblesUpExceptionsAndTimingIsCanceled()
        {
            TimingDistributionMetricType metric = new TimingDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "timing_distribution",
                sendInPings: new string[] { "store1" },
                timeUnit: TimeUnit.Second
            );

            // Create a test function that throws a NPE
            static bool TestFunc()
            {
                throw new NullReferenceException();
            }

            // Attempt to measure the function that will throw an exception.  The `measure` function
            // should allow the exception to bubble up, the timing distribution measurement is canceled.
            Assert.Throws<NullReferenceException>(() => metric.Measure(() => TestFunc()));
        }

        [Fact]
        public void EnsureThatTimeUnitControlsTruncation()
        {
            ulong maxSampleTime = 1000L * 1000 * 1000 * 60 * 10;

            foreach (TimeUnit unit in new List<TimeUnit>() {
                    TimeUnit.Nanosecond,
                    TimeUnit.Microsecond,
                    TimeUnit.Millisecond
            })
            {
                TimingDistributionMetricType metric = new TimingDistributionMetricType(
                    disabled: false,
                    category: "telemetry",
                    lifetime: Lifetime.Ping,
                    name: $"test_{unit}",
                    sendInPings: new string[] { "store1" },
                    timeUnit: unit
                );

                foreach (ulong value in new List<ulong>() {
                    1L,
                    1000L,
                    100000L,
                    maxSampleTime,
                    maxSampleTime* 1000L,
                    maxSampleTime* 1000000L
                })
                {
                    HighPrecisionTimestamp.MockedValue = 0;
                    var timerId = metric.Start();
                    HighPrecisionTimestamp.MockedValue = value;
                    metric.StopAndAccumulate(timerId);
                }

                var snapshot = metric.TestGetValue();
                Assert.True(snapshot.Values.Count < 318);
            }
        }
    }
}
