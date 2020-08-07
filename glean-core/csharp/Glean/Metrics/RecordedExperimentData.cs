/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

using System;
using System.Collections.Generic;
using System.Text.Json;

namespace Mozilla.Glean.Private
{
    /// <summary>
    /// Deserialized experiment data.
    /// </summary>
    public sealed class RecordedExperimentData
    {
        /// <summary>
        /// The experiment's branch as set through `SetExperimentActive`.
        /// </summary>
        public readonly string Branch;

        /// <summary>
        /// Any extra data associated with this experiment through `SetExperimentActive`.
        /// </summary>
        public readonly Dictionary<string, string> Extra;

        // This constructor is only useful for tests.
        internal RecordedExperimentData() { }

        RecordedExperimentData(string branch, Dictionary<string, string> extra)
        {
            Branch = branch;
            Extra = extra;
        }

        public static RecordedExperimentData FromJsonString(string json)
        {
            try
            {
                JsonDocument data = JsonDocument.Parse(json);
                JsonElement root = data.RootElement;

                string branch = root.GetProperty("branch").GetString();
                Dictionary<string, string> processedExtra = new Dictionary<string, string>();
                JsonElement rawExtraMap = root.GetProperty("extra");
                foreach (var entry in rawExtraMap.EnumerateObject())
                {
                    processedExtra.Add(entry.Name, entry.Value.GetString());
                }
                return new RecordedExperimentData(branch, processedExtra);
            }
            catch (Exception)
            {
                return null;
            }
        }
    }
}
