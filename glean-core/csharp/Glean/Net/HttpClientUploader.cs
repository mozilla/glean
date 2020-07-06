// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Net.Http;

namespace Mozilla.Glean.Net
{
    /// <summary>
    /// A simple ping uploader that uses the `HttpClient` C# object.
    /// </summary>
    public class HttpClientUploader : IPingUploader
    {
        // The timeout, in milliseconds, to use when connecting to the server.
        internal const int DEFAULT_CONNECTION_TIMEOUT_MS = 10000;

        internal readonly HttpClient httpClient;

        public HttpClientUploader() : this(null)
        {
        }

        // This constructor is only meant to be used for testing.
        internal HttpClientUploader(HttpMessageHandler customHandler)
        {
            if (customHandler != null)
            {
                httpClient = new HttpClient(customHandler);
            }
            else
            {
                httpClient = new HttpClient();
            }

            httpClient.Timeout = TimeSpan.FromMilliseconds(DEFAULT_CONNECTION_TIMEOUT_MS);
        }

        public UploadResult Upload(string url, byte[] data, (string, string)[] headers)
        {
            try
            {
                HttpRequestMessage request = new HttpRequestMessage(new HttpMethod("POST"), url);

                // TODO: Verify that cookies are not being sent in bug 1646778

                // Create `request.Content` first, as we might need to slap headers on it.
                request.Content = new ByteArrayContent(data);

                foreach ((string name, string value) header in headers)
                {
                    // The `HttpRequestMessage` separates the headers in categories: 'Content-*' headers
                    // have to be set in `request.Content.Headers.*` while other headers can be set
                    // through `request.Headers`. Let's try to deal with this by checking the header name.
                    // The following code will throw an exception if glean-core sends an header that's
                    // mistakenly routed to the wrong category, but that's fine: we'll hopefully catch that
                    // in integration tests.
                    if (header.name.StartsWith("Content-", StringComparison.InvariantCultureIgnoreCase))
                    {
                        request.Content.Headers.Add(header.name, header.value);
                        continue;
                    }

                    // Ok, that's not a content header. Try with a request header.
                    request.Headers.Add(header.name, header.value);
                }

                // While `SendAsync` is triggering an off-the-main thread request, we synchronously wait
                // for it to finish by adding `.Result`. We're fine with doing that as we want the non-blocking
                var httpResponseMessage = httpClient.SendAsync(request).Result;

                return new HttpResponse((int)httpResponseMessage.StatusCode);
            }
            catch (UriFormatException)
            {
                // There's nothing we can do to recover from this here. So let's just log an error and
                // notify the service that this job has been completed - even though we didn't upload
                // anything to the server.
                Console.WriteLine("Glean - Could not upload telemetry due to malformed URL");
                return new UnrecoverableFailure();
            }
            catch (Exception ex) when (ex is HttpRequestException || ex is System.Threading.Tasks.TaskCanceledException)
            {
                // The request failed due to an underlying issue such as network connectivity. We need to catch
                // both `HttpRequestException` and `TaskCanceledException`: the former for HTTP specific exception
                // and the latter for the task being cancelled due to hitting the provided connection timeout.
                Console.WriteLine("Glean - there was a problem while performing the network request.");
                return new RecoverableFailure();
            }
        }
    }
}
