// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System.IO;
using static Mozilla.Glean.Glean;
using Xunit;

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

        // TODO: The rest of tests need a server to verify data are submitted successfully.
        // We should take `PingTypeTest.kt` as references to implement. 
        // https://searchfox.org/glean/source/glean-core/android/src/test/java/mozilla/telemetry/glean/private/PingTypeTest.kt
        //[Fact]
        //public void SendCustomPings()
        //{

        //}

        //public void SendCustomPingsWithSnakeCase()
        //{
        //
        //}

        //[Fact]
        //public void SendCustomPingsWithKebabCase()
        //{
        //
        //}

        //[Fact]
        //public void SendCustomPingsWithoutClientId()
        //{
        //
        //}

        //[Fact]
        //public void SendCustomPingsWithAnUnknownNameNoOp()
        //{
        //
        //}

        [Fact]
        public void RegistryShouldContainBuildInPings()
        {
            Assert.True(GleanInstance.TestHasPingType("metrics"));
            Assert.True(GleanInstance.TestHasPingType("events"));
            Assert.True(GleanInstance.TestHasPingType("baseline"));
        }
    }
}
