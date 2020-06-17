// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Private;
using System;

namespace Mozilla.Glean.GleanPings
{
    internal sealed class GleanInternalPingsOuter
    {
        // Initialize the singleton using the `Lazy` facilities.
        private static readonly Lazy<GleanInternalPingsOuter>
          lazy = new Lazy<GleanInternalPingsOuter>(() => new GleanInternalPingsOuter());
        public static GleanInternalPingsOuter GleanInternalPings => lazy.Value;

        // Private constructor to disallow instantiation from external callers.
        private GleanInternalPingsOuter() { }

        internal enum BaselineReasonCodes : int
        {
            background,
            dirtyStartup,
            foreground
        }

        internal enum MetricsReasonCodes : int
        {
            overdue,
            reschedule,
            today,
            tomorrow,
            upgrade
        }

        internal enum EventsReasonCodes : int
        {
            background,
            maxCapacity,
            startup
        }

        internal PingType<BaselineReasonCodes> baseline =
            new PingType<BaselineReasonCodes>(
                    includeClientId: true,
                    sendIfEmpty: false,
                    name: "baseline",
                    reasonCodes: new string[] { "background", "dirty_startup", "foreground" });

        internal PingType<MetricsReasonCodes> metrics =
            new PingType<MetricsReasonCodes>(
                    includeClientId: true,
                    sendIfEmpty: false,
                    name: "metrics",
                    reasonCodes: new string[] { "overdue", "reschedule", "today", "tomorrow", "upgrade" });

        internal PingType<EventsReasonCodes> events =
            new PingType<EventsReasonCodes>(
                    includeClientId: true,
                    sendIfEmpty: false,
                    name: "events",
                    reasonCodes: new string[] { "background", "max_capacity", "startup" });
    }
}

