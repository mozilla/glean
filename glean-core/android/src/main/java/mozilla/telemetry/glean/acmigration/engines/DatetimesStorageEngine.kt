/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration.engines

import android.content.Context
import mozilla.telemetry.glean.private.DatetimeMetricType
import mozilla.telemetry.glean.private.Lifetime
import mozilla.telemetry.glean.private.TimeUnit
import mozilla.telemetry.glean.utils.parseISOTimeStringAsCalendar

internal class DatetimesStorageEngine(
    applicationContext: Context
) : GenericStorageEngine<String>() {

    init {
        this.applicationContext = applicationContext
    }

    companion object {
        // Unfortunately we have no way to tell, at run time, what is the precision
        // with which metrics were recorded in glean-ac. To work around this problem
        // we generate the following map from all the metrics.yaml files listed in
        // https://github.com/mozilla/probe-scraper/blob/master/repositories.yaml
        val datePrecisionMap: Map<String, TimeUnit> = mapOf(
            // from glean's metrics.yaml
            "first_run_date" to TimeUnit.Day,
            // from support-sync-telemetry's metrics.yaml
            "history_sync.started_at" to TimeUnit.Millisecond,
            "history_sync.finished_at" to TimeUnit.Millisecond,
            "bookmarks_sync.started_at" to TimeUnit.Millisecond,
            "bookmarks_sync.finished_at" to TimeUnit.Millisecond
        )
    }

    override fun deserializeSingleMetric(metricName: String, value: Any?): String? {
        // This parses the date strings on ingestion as a sanity check, but we
        // don't actually need their results, and that would throw away the
        // timezone offset information.
        (value as? String)?.let {
            stringValue -> parseISOTimeStringAsCalendar(stringValue)?.let {
                return stringValue
            }
        }
        return null
    }

    /**
     * Perform the data migration.
     */
    @Suppress("NestedBlockDepth")
    override fun migrateToGleanCore(lifetime: Lifetime) {
        super.migrateToGleanCore(lifetime)

        // Get the stored data.
        val storedData = dataStores[lifetime.ordinal]
        for ((storeName, data) in storedData) {
            // Get each storage for the specified lifetime
            for ((metricId, metricData) in data) {
                // Hotfix the prevision of metrics we know about. Otherwise,
                // use the highest possible precision.
                val precision = datePrecisionMap[metricId]?.let {
                    it
                } ?: TimeUnit.Nanosecond

                // HACK HACK HACK HACK! Hic sunt leones!
                // It would be tricky to break apart the category and the name of each metric,
                // given that categories might contain dots themselves. Just leave the category
                // blank and provide the full metric identifier through the "name".
                val metric = DatetimeMetricType(
                    name = metricId,
                    category = "",
                    sendInPings = listOf(storeName),
                    lifetime = lifetime,
                    disabled = false,
                    timeUnit = precision
                )

                // `metricData` should always successfully parse as a `Date`: we
                // already discard all the invalid stuff in `deserializeSingleMetric`.
                parseISOTimeStringAsCalendar(metricData)?.let {
                    if (lifetime == Lifetime.User) {
                        // User lifetime metrics are migrated very early: we don't want them
                        // to be batched but, rather, set immediately.
                        metric.setSync(it)
                    } else {
                        metric.set(it)
                    }
                }
            }
        }
    }
}
