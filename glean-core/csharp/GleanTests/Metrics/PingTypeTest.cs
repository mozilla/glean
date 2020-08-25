// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System.IO;
using static Mozilla.Glean.Glean;
using Xunit;
using Mozilla.Glean.Private;
using System.Text.Json;

namespace Mozilla.Glean.Tests.Metrics
{
    public class PingTypeTest
    {
        private readonly MockUploader mockUploader = new MockUploader();

        public PingTypeTest()
        {
            // Get a random test directory just for this single test.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // Remove all the pending pings from the queue.
            mockUploader.Clear();

            // In xUnit, the constructor will be called before each test. This
            // feels like a natural place to initialize / reset Glean.
            GleanInstance.Reset(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(httpClient: mockUploader),
                dataDir: tempDataDir
                );
        }

        [Fact]
        public void SendCustomPings()
        {
            PingType<NoReasonCodes> customPing = new PingType<NoReasonCodes>(
                name: "custom",
                includeClientId: true,
                sendIfEmpty: false,
                reasonCodes: null
            );

            BooleanMetricType sampleMetric = new BooleanMetricType(
                disabled: false,
                category: "test",
                lifetime: Lifetime.Ping,
                name: "boolean_metric",
                sendInPings: new string[] { "custom" }
            );

            sampleMetric.Set(true);
            Assert.True(sampleMetric.TestHasValue());

            customPing.Submit();

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("custom", request.docType);

            // Check that we have a non-null client id.
            JsonDocument jsonPayload = JsonDocument.Parse(request.payload);
            JsonElement root = jsonPayload.RootElement;
            Assert.NotNull(root.GetProperty("client_info").GetProperty("client_id").GetString());
            
            // TODO: Check the ping schema.
            // checkPingSchema(pingJson)
        }

        [Fact]
        public void SendCustomPingsWithSnakeCase()
        {
            PingType<NoReasonCodes> customPing = new PingType<NoReasonCodes>(
                name: "custom_ping",
                includeClientId: true,
                sendIfEmpty: false,
                reasonCodes: null
            );

            BooleanMetricType sampleMetric = new BooleanMetricType(
                disabled: false,
                category: "test",
                lifetime: Lifetime.Ping,
                name: "boolean_metric",
                sendInPings: new string[] { "custom_ping" }
            );

            sampleMetric.Set(true);
            Assert.True(sampleMetric.TestHasValue());

            customPing.Submit();

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("custom_ping", request.docType);

            // Check that we have a non-null client id.
            JsonDocument jsonPayload = JsonDocument.Parse(request.payload);
            JsonElement root = jsonPayload.RootElement;
            Assert.NotNull(root.GetProperty("client_info").GetProperty("client_id").GetString());

            // TODO: Check the ping schema.
            // checkPingSchema(pingJson)
        }

        [Fact]
        public void SendCustomPingsWithKebabCase()
        {
            PingType<NoReasonCodes> customPing = new PingType<NoReasonCodes>(
                name: "custom-ping",
                includeClientId: true,
                sendIfEmpty: false,
                reasonCodes: null
            );

            BooleanMetricType sampleMetric = new BooleanMetricType(
                disabled: false,
                category: "test",
                lifetime: Lifetime.Ping,
                name: "boolean_metric",
                sendInPings: new string[] { "custom-ping" }
            );

            sampleMetric.Set(true);
            Assert.True(sampleMetric.TestHasValue());

            customPing.Submit();

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("custom-ping", request.docType);

            // Check that we have a non-null client id.
            JsonDocument jsonPayload = JsonDocument.Parse(request.payload);
            JsonElement root = jsonPayload.RootElement;
            Assert.NotNull(root.GetProperty("client_info").GetProperty("client_id").GetString());

            // TODO: Check the ping schema.
            // checkPingSchema(pingJson)
        }

        [Fact]
        public void SendCustomPingsWithoutClientId()
        {
            PingType<NoReasonCodes> customPing = new PingType<NoReasonCodes>(
                name: "custom",
                includeClientId: false,
                sendIfEmpty: false,
                reasonCodes: null
            );

            BooleanMetricType sampleMetric = new BooleanMetricType(
                disabled: false,
                category: "test",
                lifetime: Lifetime.Ping,
                name: "boolean_metric",
                sendInPings: new string[] { "custom" }
            );

            sampleMetric.Set(true);
            Assert.True(sampleMetric.TestHasValue());

            customPing.Submit();

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("custom", request.docType);

            // Check that we have a non-null client id.
            JsonDocument jsonPayload = JsonDocument.Parse(request.payload);
            JsonElement root = jsonPayload.RootElement;
            Assert.False(root.GetProperty("client_info").TryGetProperty("client_id", out _));

            // TODO: Check the ping schema.
            // checkPingSchema(pingJson)
        }

        [Fact]
        public void SendCustomPingsWithAnUnknownNameNoOp()
        {
            const string unknownPingName = "unknown";

            Assert.False(GleanInstance.TestHasPingType(unknownPingName));

            BooleanMetricType sampleMetric = new BooleanMetricType(
                disabled: false,
                category: "test",
                lifetime: Lifetime.Ping,
                name: "boolean_metric",
                sendInPings: new string[] { unknownPingName }
            );

            sampleMetric.Set(true);
            Assert.True(sampleMetric.TestHasValue());

            GleanInstance.SubmitPingByName(unknownPingName);

            // We don't expect any ping to be sent.
            Assert.Equal(0, mockUploader.GetUploadRequestCount());
        }

        [Fact]
        public void RegistryShouldContainBuildInPings()
        {
            Assert.True(GleanInstance.TestHasPingType("metrics"));
            Assert.True(GleanInstance.TestHasPingType("events"));
            Assert.True(GleanInstance.TestHasPingType("baseline"));
        }
    }
}
