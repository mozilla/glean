/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration.engines

import android.content.Context
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.StringListMetricType
import mozilla.telemetry.glean.utils.toList
import org.json.JSONArray

internal class StringListsStorageEngine(
    applicationContext: Context
) : GenericStorageEngine<List<String>>() {

    init {
        this.applicationContext = applicationContext
    }

    override fun deserializeSingleMetric(metricName: String, value: Any?): List<String>? {
        /*
        Since SharedPreferences doesn't directly support storing of List<> types, we must use
        an intermediate JSONArray which can be deserialized and converted back to List<String>.
        Using JSONArray introduces a possible issue in that it's constructor will still properly
        convert a stringified JSONArray into an array of Strings regardless of whether the values
        have been properly quoted or not.  For example, [1,2,3] is as valid just like
        ["a","b","c"] is valid.
        The try/catch is necessary as JSONArray can throw a JSONException if it cannot parse the
        string into an array.
        */
        return (value as? String)?.let {
            try {
                return@let JSONArray(it).toList()
            } catch (e: org.json.JSONException) {
                return@let null
            }
        }
    }

    /**
     * Perform the data migration.
     */
    override fun migrateToGleanCore(lifetime: Lifetime) {
        super.migrateToGleanCore(lifetime)

        // Get the stored data.
        val storedData = dataStores[lifetime.ordinal]
        for ((storeName, data) in storedData) {
            // Get each storage for the specified lifetime
            for ((metricId, metricData) in data) {
                // HACK HACK HACK HACK! Hic sunt leones!
                // It would be tricky to break apart the category and the name of each metric,
                // given that categories might contain dots themselves. Just leave the category
                // blank and provide the full metric identifier through the "name".
                val metric = StringListMetricType(
                    name = metricId,
                    category = "",
                    sendInPings = listOf(storeName),
                    lifetime = lifetime,
                    disabled = false
                )

                if (lifetime == Lifetime.User) {
                    // User lifetime metrics are migrated very early: we don't want them
                    // to be batched but, rather, set immediately.
                    metric.setSync(metricData)
                } else {
                    metric.set(metricData)
                }
            }
        }
    }
}
