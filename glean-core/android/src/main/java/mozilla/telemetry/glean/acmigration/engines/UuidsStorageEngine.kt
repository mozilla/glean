/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration.engines

import android.content.Context
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.UuidMetricType
import java.util.UUID

internal class UuidsStorageEngine(
    applicationContext: Context
) : GenericStorageEngine<UUID>() {

    init {
        this.applicationContext = applicationContext
    }

    override fun deserializeSingleMetric(metricName: String, value: Any?): UUID? {
        return try {
            if (value is String) UUID.fromString(value) else null
        } catch (e: IllegalArgumentException) {
            null
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
                val metric = UuidMetricType(
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
