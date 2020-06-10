using System;
using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    public class PingTypeTest
    {
        public PingTypeTest()
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
        public void SendCustomPings()
        {
            Private.PingType<Private.NoReasonCodes> customPing = new Private.PingType<Private.NoReasonCodes>(
                name: "custom",
                includeClientId: true,
                sendIfEmpty: false,
                reasonCodes: null
            );

            // 
            //Private.BooleanMetricType booleanMetric = new Private.BooleanMetricType(
            //    category: "telemetry",
            //    disabled: false,
            //    lifetime: Private.Lifetime.Application,
            //    name: "boolean_metric",
            //    sendInPings: new string[] { "custom" }
            //);

            //// Record two strings of the same type, with a little delay.
            //booleanMetric.Set(true);

            //// Check that data was properly recorded.
            //Assert.True(booleanMetric.TestHasValue());
            customPing.Submit();
        }
    }
}
