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
    public class MemoryDistributionMetricTypeTest
    {
        public MemoryDistributionMetricTypeTest()
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
            MemoryDistributionMetricType metric = new MemoryDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "memory_distribution",
                sendInPings: new string[] { "store1" },
                memoryUnit: MemoryUnit.Kilobyte
            );

            // Accumulate a few values
            for (ulong i = 1; i <= 3; i++)
            {
                metric.Accumulate(i);
            }

            long kb = 1024;

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue());
            DistributionData snapshot = metric.TestGetValue();
            // Check the sum
            Assert.Equal<long>(1L * kb + 2L * kb + 3L * kb, snapshot.Sum);
            // Check that the 1L fell into the first value bucket
            Assert.Equal(1L, snapshot.Values[1023]);
            // Check that the 2L fell into the second value bucket
            Assert.Equal(1L, snapshot.Values[2047]);
            // Check that the 3L fell into the third value bucket
            Assert.Equal(1L, snapshot.Values[3024]);
        }

        [Fact]
        public void ValuesAreTruncatedTo1TB()
        {
            MemoryDistributionMetricType metric = new MemoryDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "memory_distribution",
                sendInPings: new string[] { "store1" },
                memoryUnit: MemoryUnit.Gigabyte
            );

            metric.Accumulate(2048L);

            // Check that data was properly recorded.
            Assert.True(metric.TestHasValue());
            var snapshot = metric.TestGetValue();
            // Check the sum
            Assert.Equal(1L << 40, snapshot.Sum);
            // Check that the 1L fell into 1TB bucket
            Assert.Equal(1L, snapshot.Values[(1L << 40) - 1]);
            // Check that an error was recorded
            Assert.Equal(1, metric.TestGetNumRecordedErrors(ErrorType.InvalidValue));
        }

        [Fact]
        public void DisabledMemoryDistributionsMustNotRecordData()
        {
            MemoryDistributionMetricType metric = new MemoryDistributionMetricType(
                disabled: true,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "memory_distribution",
                sendInPings: new string[] { "store1" },
                memoryUnit: MemoryUnit.Gigabyte
            );

            metric.Accumulate(1L);

            // Check that nothing was recorded.
            Assert.False(metric.TestHasValue(), "MemoryDistributions without a lifetime should not record data.");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            MemoryDistributionMetricType metric = new MemoryDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "memory_distribution",
                sendInPings: new string[] { "store1" },
                memoryUnit: MemoryUnit.Gigabyte
            );
            Assert.Throws<NullReferenceException>(() => metric.TestGetValue());
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            MemoryDistributionMetricType metric = new MemoryDistributionMetricType(
                disabled: false,
                category: "telemetry",
                lifetime: Lifetime.Ping,
                name: "memory_distribution",
                sendInPings: new string[] { "store1", "store2", "store3" },
                memoryUnit: MemoryUnit.Kilobyte
            );

            // Accumulate a few values
            for (ulong i = 1; i <= 3; i++)
            {
                metric.Accumulate(i);
            }

            // Check that data was properly recorded in the second ping.
            Assert.True(metric.TestHasValue("store2"));
            var snapshot = metric.TestGetValue("store2");
            // Check the sum
            Assert.Equal(6144L, snapshot.Sum);
            // Check that the 1L fell into the first bucket
            Assert.Equal(1L, snapshot.Values[1023]);
            // Check that the 2L fell into the second bucket
            Assert.Equal(1L, snapshot.Values[2047]);
            // Check that the 3L fell into the third bucket
            Assert.Equal(1L, snapshot.Values[3024]);

            // Check that data was properly recorded in the third ping.
            Assert.True(metric.TestHasValue("store3"));
            var snapshot2 = metric.TestGetValue("store3");
            // Check the sum
            Assert.Equal(6144L, snapshot2.Sum);
            // Check that the 1L fell into the first bucket
            Assert.Equal(1L, snapshot2.Values[1023]);
            // Check that the 2L fell into the second bucket
            Assert.Equal(1L, snapshot2.Values[2047]);
            // Check that the 3L fell into the third bucket
            Assert.Equal(1L, snapshot2.Values[3024]);
        }
    }
}
