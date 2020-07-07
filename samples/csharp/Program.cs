// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean;
using System;
using System.IO;
using static Mozilla.Glean.Glean;

namespace csharp
{
    class Program
    {
        static void Main(string[] args)
        {
            string gleanDataDir = Path.Combine(Directory.GetCurrentDirectory(), "glean_data");
            Console.WriteLine("Adding Glean data to {0}", gleanDataDir);

            GleanInstance.Initialize(
                applicationId: "org.mozilla.glean.csharp.sample",
                applicationVersion: "1.0",
                uploadEnabled: true,
                configuration: new Configuration(),
                dataDir: gleanDataDir
                );

            // Create a sample ping and metric. Note that, once we'll have C# code
            // generation in place (bug 1647214), we would be able to remove the
            // manual definitions below and move them to the appropriate registry files.
            Mozilla.Glean.Private.PingType<Mozilla.Glean.Private.NoReasonCodes> samplePing =
                new Mozilla.Glean.Private.PingType<Mozilla.Glean.Private.NoReasonCodes>(
                    includeClientId: true,
                    sendIfEmpty: false,
                    name: "sample",
                    reasonCodes: null
                    );

            Mozilla.Glean.Private.StringMetricType sampleString = new Mozilla.Glean.Private.StringMetricType(
                category: "csharp.test",
                disabled: false,
                lifetime: Mozilla.Glean.Private.Lifetime.Application,
                name: "mystring",
                sendInPings: new string[] { "sample" }
            );

            sampleString.Set("test-string");

            samplePing.Submit();

            Console.WriteLine("Press any key to exit the sample...");
            Console.ReadKey();
        }
    }
}
