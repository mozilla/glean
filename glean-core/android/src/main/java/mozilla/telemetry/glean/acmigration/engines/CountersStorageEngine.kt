/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration.engines

import android.content.Context
import mozilla.telemetry.glean.private.CounterMetricType
import mozilla.telemetry.glean.private.Lifetime

internal class CountersStorageEngine(
    applicationContext: Context
) : GenericStorageEngine<Int>() {

    init {
        this.applicationContext = applicationContext
    }

    override fun deserializeSingleMetric(metricName: String, value: Any?): Int? {
        return (value as? Int)?.let {
            return@let if (it < 0) null else it
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
                val metric = CounterMetricType(
                    name = metricId,
                    category = "",
                    sendInPings = listOf(storeName),
                    lifetime = lifetime,
                    disabled = false
                )

                if (lifetime == Lifetime.User) {
                    // User lifetime metrics are migrated very early: we don't want them
                    // to be batched but, rather, set immediately.
                    metric.addSync(metricData)
                } else {
                    metric.add(metricData)
                }
            }
        }
    }
}
