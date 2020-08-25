// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Net;
using System;
using System.IO;
using System.Text;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Net
{
    public class BaseUploaderTest
    {
        public BaseUploaderTest()
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

        // Define a fake uploader to use in our tests.
        class FakeUploader : IPingUploader
        {
            private Action<string, byte[], (string, string)[]> uploadAction;

            public FakeUploader(Action<string, byte[], (string, string)[]> uploadAction)
            {
                this.uploadAction = uploadAction;
            }

            UploadResult IPingUploader.Upload(string url, byte[] data, (string, string)[] headers)
            {
                uploadAction.Invoke(url, data, headers);
                return new UnrecoverableFailure();
            }
        };

        [Fact]
        public void TestUploadMustgetCalledWithTheFullSubmissionPath()
        {
            const string testPath = "/some/random/path/not/important";
            const string testPing = "{ 'ping': 'test' }";
            (string, string)[] testHeaders = { ("X-Test-Glean", "nothing-to-see-here") };
            Configuration testConfig = new Configuration(serverEndpoint: "https://example.com");

            BaseUploader testBaseUploader =
                new BaseUploader(new FakeUploader(new Action<string, byte[], (string, string)[]>((url, data, headers) => {
                Assert.Equal(testConfig.serverEndpoint + testPath, url);
            })));

            // Manually trigger upload.
            testBaseUploader.Upload(testPath, Encoding.UTF8.GetBytes(testPing), testHeaders, testConfig);
        }

        [Fact]
        public void TestGetHeadersFromJSONString()
        {
            const string testDocumentId = "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0";

            // Empty headers string.
            (string, string)[] headers = BaseUploader.GetHeadersFromJSONString(testDocumentId, "", new Configuration());
            Assert.Empty(headers);

            // Corrupted headers.
            headers = BaseUploader.GetHeadersFromJSONString(testDocumentId, "[not-json", new Configuration());
            Assert.Empty(headers);

            // Good headers.
            string testHeaders = "{\"X-Test-Glean\": \"nothing-to-see-here\"}";
            headers = BaseUploader.GetHeadersFromJSONString(testDocumentId, testHeaders, new Configuration());
            Assert.Equal(new[] { ("X-Test-Glean", "nothing-to-see-here") }, headers);
        }
    }
}
