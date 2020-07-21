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
    public sealed class JweMetricTypeTest
    {
        private string header = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ";
        private string key = "OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg";
        private string initVector = "48V1_ALb6US04U3b";
        private string cipherText = "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A";
        private string authTag = "XFBoMYUZodetZdvTiFvSkQ";
        private string jwe = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ.OKOawDo13gRp2ojaHV7LFpZcgV7T6DVZKTyKOMTYUmKoTCVJRgckCL9kiMT03JGeipsEdY3mx_etLbbWSrFr05kLzcSr4qKAq7YN7e9jwQRb23nfa6c9d-StnImGyFDbSv04uVuxIp5Zms1gNxKKK2Da14B8S4rzVRltdYwam_lDp5XnZAYpQdb76FdIKLaVmqgfwX7XWRxv2322i-vDxRfqNzo_tETKzpVLzfiwQyeyPGLBIO56YJ7eObdv0je81860ppamavo35UgoRdbYaBcoh9QcfylQr66oc6vFWXRcZ_ZT2LawVCWTIy3brGPi6UklfCpIMfIjf7iGdXKHzg.48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ";
        private string minimumJwe = "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ...5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.";

        public JweMetricTypeTest()
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
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1" }
            );

            // Record two JWEs of the same type, with a little delay.
            jweMetric.Set(this.header, this.key, this.initVector, this.cipherText, this.authTag);

            // Check that data was properly recorded.
            Assert.True(jweMetric.TestHasValue());
            Assert.Equal(this.jwe, jweMetric.testGetCompactRepresentation());

            jweMetric.Set(this.header, "", "", this.cipherText, "");

            // Check that data was properly recorded.
            Assert.True(jweMetric.TestHasValue());
            Assert.Equal(this.minimumJwe, jweMetric.testGetCompactRepresentation());
        }

        [Fact]
        public void DisabledJwesMustNotRecordData()
        {
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the string.
            jweMetric.Set(this.header, this.key, this.initVector, this.cipherText, this.authTag);

            // Check that nothing was recorded.
            Assert.False(jweMetric.TestHasValue(), "JWEs must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: true,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => jweMetric.TestGetValue());
        }

        [Fact]
        public void TestJweGetValueReturnsCorrectJweDataRepresentation()
        {
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1" }
            );

            jweMetric.Set(this.header, this.key, this.initVector, this.cipherText, this.authTag);

            Private.JweData data = jweMetric.TestGetValue();
            Assert.Equal(data.Header, this.header);
            Assert.Equal(data.Key, this.key);
            Assert.Equal(data.InitVector, this.initVector);
            Assert.Equal(data.CipherText, this.cipherText);
            Assert.Equal(data.AuthTag, this.authTag);
        }

        [Fact]
        public void APISavesToSecondaryPings()
        {
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Record two JWEs of the same type, with a little delay.
            jweMetric.Set(this.header, this.key, this.initVector, this.cipherText, this.authTag);

            // Check that data was properly recorded in the second ping.
            Assert.True(jweMetric.TestHasValue("store2"));
            Assert.Equal(this.jwe, jweMetric.testGetCompactRepresentation("store2"));

            jweMetric.Set(this.header, "", "", this.cipherText, "");
            // Check that data was properly recorded in the second ping.
            Assert.True(jweMetric.TestHasValue("store2"));
            Assert.Equal(this.minimumJwe, jweMetric.testGetCompactRepresentation("store2"));
        }

        [Fact]
        public void SettingALongStringRecordsAnError()
        {
            Private.JweMetricType jweMetric = new Private.JweMetricType(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Application,
                name: "jwe_metric",
                sendInPings: new string[] { "store1", "store2" }
            );

            // Too long elements should yield a InvalidOverflow error
            jweMetric.Set(new string('X', 1025), this.key, this.initVector, this.cipherText, this.authTag);
            Assert.Equal(1, jweMetric.TestGetNumRecordedErrors(ErrorType.InvalidOverflow));

            // Invalid compact string representation yield a InvalidValue error
            jweMetric.setWithCompactRepresentation("");
            Assert.Equal(1, jweMetric.TestGetNumRecordedErrors(ErrorType.InvalidValue));
        }
    }
}
