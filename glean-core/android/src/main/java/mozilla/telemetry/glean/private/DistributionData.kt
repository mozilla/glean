/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting
import mozilla.telemetry.glean.utils.tryGetLong
import org.json.JSONObject

/**
 * This class represents the structure of a distribution according to the pipeline schema. It
 * is meant to help serialize and deserialize data to the correct format for transport and storage,
 * as well as including a helper function to calculate the bucket sizes.
 *
 * @param values a map containing the bucket index mapped to the accumulated count
 * @param sum the accumulated sum of all the samples in the distribution
 */
@VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
data class DistributionData(
    val values: MutableMap<Long, Long>,
    var sum: Long
) {
    companion object {
        /**
         * Factory function that takes stringified JSON and converts it back into a
         * [DistributionData].  This tries to read all values and attempts to
         * use a default where no value exists.
         *
         * @param json Stringified JSON value representing a [DistributionData] object
         * @return A [DistributionData] or null if unable to rebuild from the string.
         */
        @Suppress("ReturnCount", "ComplexMethod")
        internal fun fromJsonString(json: String): DistributionData? {
            val jsonObject: JSONObject
            try {
                jsonObject = JSONObject(json)
            } catch (e: org.json.JSONException) {
                return null
            }

            // Attempt to parse the values map, if it fails then something is wrong and we need to
            // return null.
            val values = try {
                val mapData = jsonObject.getJSONObject("values")
                val valueMap: MutableMap<Long, Long> = mutableMapOf()
                mapData.keys().forEach { key ->
                    valueMap[key.toLong()] = mapData.tryGetLong(key) ?: 0L
                }
                valueMap
            } catch (e: org.json.JSONException) {
                // This should only occur if there isn't a key/value pair stored for "values"
                return null
            }
            val sum = jsonObject.tryGetLong("sum") ?: return null

            return DistributionData(
                values = values,
                sum = sum
            )
        }
    }

    /**
     * The total count of accumulated values.
     *
     * This is calculated from all recorded values.
     */
    val count: Long
        get() = values.map { it.value }.sum()
}
