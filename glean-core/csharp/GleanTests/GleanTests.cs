// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System.IO;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests
{
    public class GleanTests
    {
        public GleanTests()
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
        public void GettingUploadEnabledBeforeInitShouldNotCrash()
        {
            // Can't use resetGlean directly
            GleanInstance.TestDestroyGleanHandle();

            GleanInstance.SetUploadEnabled(true);
            Assert.True(GleanInstance.GetUploadEnabled());

            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
            GleanInstance.Initialize(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: tempDataDir
                );
        }
    }
}
