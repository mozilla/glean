/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Text.Json;

namespace Mozilla.Glean.Private
{
    /// <summary>
    /// This class represents the structure of a distribution according to the pipeline schema. It
    /// is meant to help serialize and deserialize data to the correct format for transport and storage,
    /// as well as including a helper function to calculate the bucket sizes.
    /// </summary>
    public sealed class DistributionData
    {
        /// <summary>
        /// A map containing the bucket index mapped to the accumulated count.
        /// </summary>
        public readonly Dictionary<Int64, Int64> Values;
        /// <summary>
        /// The accumulated sum of all the samples in the distribution.
        /// </summary>
        public readonly Int64 Sum;

        // This constructor is only useful for tests.
        internal DistributionData() { }

        DistributionData(Dictionary<Int64, Int64> values, Int64 sum)
        {
            this.Values = values;
            this.Sum = sum;
        }

        /// <summary>
        /// Factory function that takes stringified JSON and converts it back into a
        /// `DistributionData`.  This tries to read all values and attempts to
        /// use a default where no value exists.
        /// </summary>
        /// 
        /// <param name="json">Stringified JSON value representing a `DistributionData` object</param>
        /// <returns>A `DistributionData` or null if unable to rebuild from the string.</returns>
        public static DistributionData FromJsonString(string json)
        {
            try
            {
                JsonDocument data = JsonDocument.Parse(json);
                JsonElement root = data.RootElement;
                
                Int64 sum = root.GetProperty("sum").GetInt64();
                Dictionary<Int64, Int64> processedValues = new Dictionary<Int64, Int64>();
                JsonElement rawValuesMap = root.GetProperty("values");
                foreach (var entry in rawValuesMap.EnumerateObject())
                {
                    processedValues.Add(Convert.ToInt64(entry.Name), entry.Value.GetInt64());
                }
                return new DistributionData(processedValues, sum);
            }
            catch (Exception)
            {
                // We're really interesting in catching anything that could have go wrong
                // in the try block, and return null. There's nothing we could do anyway.
                return null;
            }
        }

        /// <summary>
        /// The total count of accumulated values.
        /// 
        /// This is calculated from all recorded values.
        /// </summary>
        /// <returns>The count of accumulated values</returns>
        Int64 Count()
        {
            return Values.Values.Sum();
        }
    }
}
