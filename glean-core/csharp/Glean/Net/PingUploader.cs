// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

namespace Mozilla.Glean.Net
{
    /// <summary>
    /// The result of the ping upload.
    ///
    /// See below for the different possible cases.
    /// </summary>
    public class UploadResult
    {
        public virtual int ToFfi()
        {
            return (int)FFI.Constants.UPLOAD_RESULT_UNRECOVERABLE;
        }
    }

    /// <summary>
    /// A HTTP response code.
    /// 
    /// This can still indicate an error, depending on the status code.
    /// </summary>
    public class HttpResponse : UploadResult
    {
        internal readonly int statusCode;

        public HttpResponse(int statusCode)
        {
            this.statusCode = statusCode;
        }

        public override int ToFfi()
        {
            return (int)FFI.Constants.UPLOAD_RESULT_HTTP_STATUS | statusCode;
        }
    }

    /// <summary>
    /// An unrecoverable upload failure.
    /// 
    /// A possible cause might be a malformed URL.
    /// The ping data is removed afterwards.
    /// </summary>
    public class UnrecoverableFailure : UploadResult
    {
        public override int ToFfi()
        {
            return (int)FFI.Constants.UPLOAD_RESULT_UNRECOVERABLE;
        }
    }

    /// <summary>
    /// An unrecoverable upload failure.
    /// 
    /// During upload something went wrong,
    /// e.g. the network connection failed.
    /// The upload should be retried at a later time.
    /// </summary>
    public class RecoverableFailure : UploadResult
    {
        public override int ToFfi()
        {
            return (int)FFI.Constants.UPLOAD_RESULT_RECOVERABLE;
        }
    }

    /// <summary>
    /// The interface defining how to send pings.
    /// </summary>
    public interface IPingUploader
    {
        /// <summary>
        /// Synchronously upload a ping to a server.
        /// </summary>
        /// <param name="url">the URL path to upload the data to</param>
        /// <param name="data">the serialized text data to send</param>
        /// <param name="headers">a vector of tuples containing the headers to add</param>
        /// <returns>An `UploadResult` representing the response that came from
        /// the upload attempt.</returns>
        UploadResult Upload(string url, byte[] data, (string, string)[] headers);
    }
}
