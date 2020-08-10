// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System.Collections.Generic;
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
        public void SendAPing()
        {
            // TODO: This test needs a server to verify data are submitted successfully.
            // We should take `GleanTest.kt` as a reference to implement. 
            GleanInstance.HandleBackgroundEvent();
        }

        [Fact]
        public void TestExperimentsRecording()
        {
            GleanInstance.SetExperimentActive(
                "experiment_test", "branch_a"
            );
            GleanInstance.SetExperimentActive(
            "experiment_api", "branch_b",
            new Dictionary<string, string>() { { "test_key", "value" } }
            );
            Assert.True(GleanInstance.TestIsExperimentActive("experiment_api"));
            Assert.True(GleanInstance.TestIsExperimentActive("experiment_test"));

            GleanInstance.SetExperimentInactive("experiment_test");

            Assert.True(GleanInstance.TestIsExperimentActive("experiment_api"));
            Assert.False(GleanInstance.TestIsExperimentActive("experiment_test"));

            var storedData = GleanInstance.TestGetExperimentData("experiment_api");
            Assert.Equal("branch_b", storedData.Branch);
            Assert.Single(storedData.Extra);
            Assert.Equal("value", storedData.Extra["test_key"]);
        }

        [Fact]
        public void TestExperimentsRecordingBeforeGleanInits()
        {
            // This test relies on Glean not being initialized and task queuing to be on.
            GleanInstance.TestDestroyGleanHandle();
            Dispatchers.QueueInitialTasks = true;

            GleanInstance.SetExperimentActive(
                "experiment_set_preinit", "branch_a"
            );

            GleanInstance.SetExperimentActive(
                "experiment_preinit_disabled", "branch_a"
            );

            GleanInstance.SetExperimentInactive("experiment_preinit_disabled");

            // This will init glean and flush the dispatcher's queue.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
            GleanInstance.Reset(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: tempDataDir
                );

            Assert.True(GleanInstance.TestIsExperimentActive("experiment_set_preinit"));
            Assert.False(GleanInstance.TestIsExperimentActive("experiment_preinit_disabled"));
        }

        [Fact]
        public void SettingMaxEventsDoesNotCrash()
        {
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            GleanInstance.Reset(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(maxEvents: 500),
                dataDir: tempDataDir
                );
        }
    }
}
