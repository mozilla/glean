// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Net;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using Xunit;

namespace Mozilla.Glean.Tests.Net
{
    public class HttpClientUploaderTest
    {
        private const string TEST_URL = "http://example.com/some/random/path/not/important";
        private const string TEST_PING = "{ 'ping': 'test' }";

        private class FakeHttpMessageHandler : HttpMessageHandler
        {
            private readonly Action<HttpRequestMessage> requestAction;
            private readonly HttpStatusCode returnCode;

            public FakeHttpMessageHandler(
                Action<HttpRequestMessage> requestAction,
                HttpStatusCode returnCode
                )
            {
                this.requestAction = requestAction;
                this.returnCode = returnCode;
            }

            protected override Task<HttpResponseMessage> SendAsync(HttpRequestMessage request, CancellationToken cancellationToken)
            {
                var taskCompletionSource = new TaskCompletionSource<HttpResponseMessage>();

                // Invoke the handler.
                requestAction.Invoke(request);

                // Generate a result to return.
                taskCompletionSource.SetResult(new HttpResponseMessage
                {
                    StatusCode = returnCode,
                    Content = new StringContent("Response-Test-Data")
                });

                return taskCompletionSource.Task;
            }
        }

        [Fact]
        public void TestTimeoutsProperlySet()
        {
            HttpClientUploader uploader = new HttpClientUploader();
            Assert.Equal(HttpClientUploader.DEFAULT_CONNECTION_TIMEOUT_MS, uploader.httpClient.Timeout.TotalMilliseconds);
        }

        [Fact]
        public void TestGleanHeadersAreCorrectlyDispatched()
        {
            (string, string)[] expectedHeaders = {
                ("Content-Type", "application/json; charset=utf-8"),
                ("Test-header", "SomeValue"),
                ("OtherHeader", "Glean/Test 25.0.2")
            };

            List<(string, string)> receivedHeaders = new List<(string, string)>();

            HttpClientUploader uploader = new HttpClientUploader(new FakeHttpMessageHandler((request) =>
            {
                // Get both content and request headers.
                foreach (var h in request.Headers)
                {
                    receivedHeaders.Add((h.Key, string.Join(",", h.Value.ToArray())));
                }

                foreach (var h in request.Content.Headers)
                {
                    receivedHeaders.Add((h.Key, string.Join(",", h.Value.ToArray())));
                }
            }, HttpStatusCode.OK));
            uploader.Upload(TEST_URL, Encoding.UTF8.GetBytes(TEST_PING), expectedHeaders);

            Assert.Equal(expectedHeaders.Length, receivedHeaders.Count);
            // For some reason checking `expectedHeaders.SequenceEqual(receivedHeaders.ToArray())`
            // does not work: it always fails. Computing the intersection of the two lists work, though.
            // As long as the size of the intersection equals that of the expected headers, we should be
            // fine.
            Assert.Equal(expectedHeaders.Length, receivedHeaders.Intersect(expectedHeaders).ToArray().Length);
        }

        [Fact]
        public void TestUploadReturnsTheStatusCodeForSuccessfulRequests()
        {
            // Create a fake uploader that always reports 200.
            HttpClientUploader uploader =
                new HttpClientUploader(new FakeHttpMessageHandler((_) => { }, HttpStatusCode.OK));
            var result = uploader.Upload(TEST_URL, Encoding.UTF8.GetBytes(TEST_PING), new (string, string)[]{ });
            // Check if we received 200.
            Assert.IsType<HttpResponse>(result);
            Assert.Equal((int)HttpStatusCode.OK, ((HttpResponse)result).statusCode);
        }

        [Fact]
        public void TestUploadCorrectlyUploadsThePingData()
        {
            // Create a fake uploader that always reports 200.
            HttpClientUploader uploader = new HttpClientUploader(new FakeHttpMessageHandler((request) =>
            {
                Assert.Equal(TEST_PING, Encoding.UTF8.GetString(request.Content.ReadAsByteArrayAsync().Result));
            }, HttpStatusCode.OK));
            var result = uploader.Upload(TEST_URL, Encoding.UTF8.GetBytes(TEST_PING), new (string, string)[] { });

            // We received a result, the content  will be checked in the fake handler above.
            Assert.IsType<HttpResponse>(result);
        }

        // TODO: Add cookie related tests with bug 1646778
    }
}
