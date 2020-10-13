// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.FFI;
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text.Json;
using System.Threading;
using Serilog;
using static Mozilla.Glean.Utils.GleanLogger;

namespace Mozilla.Glean.Net
{
    /// <summary>
    /// The logic for uploading pings: this leaves the actual upload implementation
    /// to the user-provided delegate.
    /// </summary>
    internal class BaseUploader
    {
        internal const int THROTTLED_BACKOFF_MS = 60_000;
        private readonly IPingUploader uploader;

        /// <summary>
        /// This is the tag used for logging from this class.
        /// </summary>
        private const string LogTag = "glean/BaseUploader";

        /// <summary>
        /// A logger configured for this class
        /// </summary>
        private static readonly ILogger Log = GetLogger(LogTag);

        internal BaseUploader(IPingUploader uploader)
        {
            this.uploader = uploader;
        }

        /// <summary>
        /// This function triggers the actual upload.
        /// 
        /// It calls the implementation-specific upload function.
        /// </summary>
        /// <param name="url">the URL path to upload the data to</param>
        /// <param name="data">the serialized text data to send</param>
        /// <param name="headers">a vector of tuples containing the headers to add</param>
        /// <param name="config">the Glean configuration</param>
        /// <returns>An `UploadResult` representing the response that came from
        /// the upload attempt.</returns>
        internal UploadResult Upload(string url, byte[] data, (string, string)[] headers, Configuration config)
        {
            // Bail out if we don't have a valid uploader. This should never happen outside of tests.
            if (uploader == null)
            {
                Log.Error("No HTTP uploader defined. Please set it in the Glean SDK configuration.");
                return new RecoverableFailure();
            }

            return uploader.Upload(config.serverEndpoint + url, data, headers);
        }

        internal static (string, string)[] GetHeadersFromJSONString(string documentId, string headers, Configuration config)
        {
            List<(string, string)> headerList = new List<(string, string)>();
            try
            {
                // Parse the headers from JSON.
                Dictionary<string, string> parsedHeaders = JsonSerializer.Deserialize<Dictionary<string, string>>(headers);

                foreach (KeyValuePair<string, string> h in parsedHeaders)
                {
                    headerList.Add((h.Key, h.Value));
                }
            }
            catch (JsonException e)
            {
                Log.Error(e, $"Error while parsing headers for ping {documentId}");
            }

            return headerList.ToArray();
        }

        /// <summary>
        /// Signals Glean to upload pings at the next best opportunity.
        /// </summary>
        internal void TriggerUpload(Configuration config)
        {
            // TODO: must not work like this, it should work off the main thread.
            // FOR TESTING Implement the upload worker here and call this from Glean.cs

            // Limits are enforced by glean-core to avoid an inifinite loop here.
            // Whenever a limit is reached, this binding will receive `UploadTaskTag.Done` and step out.
            while (true)
            {
                FfiUploadTask incomingTask = new FfiUploadTask();
                LibGleanFFI.glean_get_upload_task(ref incomingTask);

                UploadTaskTag tag = (UploadTaskTag)incomingTask.tag;
                switch (tag)
                {
                    case UploadTaskTag.Upload:
                        {
                            // Extract C#-friendly data from the FFI object
                            string documentId = LibGleanFFI.GetFromRustString(incomingTask.body.documentId);
                            string path = LibGleanFFI.GetFromRustString(incomingTask.body.path);
                            string headersString = LibGleanFFI.GetFromRustString(incomingTask.body.headers);
                            (string, string)[] headers = GetHeadersFromJSONString(documentId, headersString, config);
                            byte[] body = new byte[incomingTask.body.bodyLen];
                            Marshal.Copy(incomingTask.body.body, body, 0, body.Length);

                            // Delegate the actual upload and get its return value.
                            UploadResult result = Upload(path, body, headers, config);

                            // Copy the `FfiUploadTask` to unmanaged memory, because
                            // `glean_process_ping_upload` assumes it has to free the memory.
                            IntPtr ptrCopy = Marshal.AllocHGlobal(Marshal.SizeOf(incomingTask));
                            Marshal.StructureToPtr(incomingTask, ptrCopy, false);

                            // Process the upload response in the core.
                            LibGleanFFI.glean_process_ping_upload_response(ptrCopy, result.ToFfi());

                            // Free the allocated.
                            Marshal.FreeHGlobal(ptrCopy);
                        }
                        break;
                    case UploadTaskTag.Wait:
                        Thread.Sleep(THROTTLED_BACKOFF_MS);
                        break;
                    case UploadTaskTag.Done:
                        // Nothing to do here, break out of the loop.
                        return;
                }
            }
        }

        /// <summary>
        /// Cancel any outstanding upload.
        /// </summary>
        internal void CancelUploads()
        {
            // TODO: to be implemented once a real HTTP uploader is added.
        }
    }
}
