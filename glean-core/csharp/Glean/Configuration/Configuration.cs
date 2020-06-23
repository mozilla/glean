// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Net;

namespace Mozilla.Glean
{
    public sealed class Configuration
    {
        /// <summary>
        /// The default server pings are sent to.
        /// </summary>
        public const string DefaultTelemetryEndpoint = "https://incoming.telemetry.mozilla.org";

        public string serverEndpoint;
        public string channel;
        public int? maxEvents;
        public bool logPings;
        public string pingTag;

        public IPingUploader httpClient;

        /// <summary>
        /// Configuration for Glean.
        /// </summary>
        /// <param name="serverEndpoint"> the server pings are sent to. Please note that this
        /// is only meant to be changed for tests.</param>
        /// <param name="channel">the release channel the application is on, if known. This will be
        /// sent along with all the pings, in the `client_info` section.</param>
        /// <param name="maxEvents">the number of events to store before the events ping is sent.</param>
        /// <param name="httpClient">The HTTP client implementation to use for uploading pings.</param>
        /// <param name="logPings">Whether to log ping contents to the console</param>
        /// <param name="pingTag">String tag to be applied to headers when uploading pings for debug view.</param>
        public Configuration(
            string serverEndpoint = DefaultTelemetryEndpoint,
            string channel = null,
            int? maxEvents = null,
            IPingUploader httpClient = null,
            bool logPings = false,
            string pingTag = null)
        {
            this.serverEndpoint = serverEndpoint;
            this.channel = channel;
            this.maxEvents = maxEvents;
            this.httpClient = httpClient ?? new HttpClientUploader();
            this.logPings = logPings;
            this.pingTag = pingTag;
        }
    }
}
