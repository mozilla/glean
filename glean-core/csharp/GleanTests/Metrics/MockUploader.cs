// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System.IO;
using Mozilla.Glean.Net;
using System.Collections.Concurrent;
using System.Text;
using System.IO.Compression;

namespace Mozilla.Glean.Tests.Metrics
{
    /// <summary>
    /// A mock uploader class to support testing uploaded pings.
    /// </summary>
    internal class MockUploader : IPingUploader
    {
        public struct UploadRequest
        {
            public string url;
            public string docType;
            public string payload;
            public (string, string)[] headers;
        };

        private readonly BlockingCollection<UploadRequest> requestQueue = new BlockingCollection<UploadRequest>();

        UploadResult IPingUploader.Upload(string url, byte[] data, (string, string)[] headers)
        {
            requestQueue.Add(new UploadRequest {
                url = url,
                docType = url.Split("/")[5],
                payload = GetPlainBody(data, headers),
                headers = headers,
            });

            return new HttpResponse(200);
        }

        public UploadRequest GetPendingUpload()
        {
            return requestQueue.Take();
        }

        public void Clear()
        {
            while (requestQueue.TryTake(out _)) { };
        }

        public int GetUploadRequestCount()
        {
            return requestQueue.Count;
        }

        private string DecompressGZIP(byte[] data)
        {
            // Note: we need two memory streams because we don't know the size
            // of the uncompressed buffer
            using MemoryStream outputMemoryStream = new MemoryStream();
            using MemoryStream inputMemoryStream = new MemoryStream(data);
            using (GZipStream gzipStream = new GZipStream(inputMemoryStream, CompressionMode.Decompress))
            {
                gzipStream.CopyTo(outputMemoryStream);
            }

            return Encoding.UTF8.GetString(outputMemoryStream.ToArray());
        }

        /// <summary>
        /// Convenience method to get the body of a request as a String.
        /// The UTF8 representation of the request body will be returned.
        /// If the request body is gzipped, it will be decompressed first.
        /// </summary>
        /// <param name="data">The input byte payload.</param>
        /// <param name="headers">The headers that come with the payload.</param>
        /// <returns>a `string` containing the body of the request.</returns>
        private string GetPlainBody(byte[] data, (string, string)[] headers)
        {
            bool isGzip = false;

            foreach ((string, string) h in headers)
            {
                if (h.Item1 == "Content-Encoding" && h.Item2 == "gzip")
                {
                    isGzip = true;
                    break;
                }
            }

            // We don't have gzip, so just decode the UTF8.
            if (!isGzip)
            {
                return Encoding.UTF8.GetString(data);
            }

            // We have GZIP, decompress and then decode.
            return DecompressGZIP(data);
        }
    }
}
