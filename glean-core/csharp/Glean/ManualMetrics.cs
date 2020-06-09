// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using System;

namespace Mozilla.Glean.GleanMetrics
{
    internal sealed class GleanInternalMetricsOuter
    {
        // Initialize the singleton using the `Lazy` facilities.
        private static readonly Lazy<GleanInternalMetricsOuter>
          lazy = new Lazy<GleanInternalMetricsOuter>(() => new GleanInternalMetricsOuter());
        public static GleanInternalMetricsOuter GleanInternalMetrics => lazy.Value;

        // Private constructor to disallow instantiation from external callers.
        private GleanInternalMetricsOuter() { }

        // This disable the linting warning about having a method
        // starting with a lowercase member.
#pragma warning disable IDE1006 // Naming Styles
        private readonly Lazy<StringMetricType> architectureLazy = new Lazy<StringMetricType>(() => new StringMetricType(
            category: "",
            disabled: false,
            lifetime: Lifetime.Application,
            name: "architecture",
            sendInPings: new string[] { "glean_client_info" }
        ));

        // This makes it possible to access `architecture`
        // operations as `architecture.<operation>()` instead
        // of `architecture.Value.<operation>()`. 
        public StringMetricType architecture => architectureLazy.Value;
#pragma warning restore IDE1006 // Naming Styles
    }
}

