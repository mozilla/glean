// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Serilog;

namespace Mozilla.Glean.Utils
{
    /// <summary>
    /// This is a utility class to wrap the Serilog logger and simplify setup
    /// by returning a correctly configured logger while only passing in the
    /// log tag.
    /// </summary>
    internal static class GleanLogger
    {
        public static ILogger GetLogger(string logTag)
        {
            return new LoggerConfiguration()
                .WriteTo.Console(outputTemplate:
                    "[{Timestamp:HH:mm:ss} {Level:u3}] {LogTag}: {Message:lj}{NewLine}{Exception}")
                .CreateLogger().ForContext("LogTag", logTag);
        }
    }
}
