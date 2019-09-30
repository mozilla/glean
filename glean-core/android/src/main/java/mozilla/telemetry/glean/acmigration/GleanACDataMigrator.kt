/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration

import android.content.Context
import android.content.SharedPreferences
import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import mozilla.telemetry.glean.acmigration.engines.DatetimesStorageEngine
import mozilla.telemetry.glean.acmigration.engines.UuidsStorageEngine
import mozilla.telemetry.glean.private.Lifetime
import java.lang.NullPointerException
import java.util.UUID

/**
 * A class encapsulating the code used for migrating data from glean-ac
 * to glean-core. This class, along with all the migration code, should be removed
 * from this codebase 6 months after the last application was migrated to
 * glean-core.
 */
internal class GleanACDataMigrator(
    private val applicationContext: Context
) {
    companion object {
        // The name of the Glean AC package, used to build the name of the files
        // of the preferences files that contain data to be migrated.
        @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
        internal const val GLEAN_AC_PACKAGE_NAME = "mozilla.components.service.glean"

        // A known client id clients with disabled telemetry are set to.
        @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
        internal val KNOWN_CLIENT_ID = UUID.fromString(
            "c0ffeec0-ffee-c0ff-eec0-ffeec0ffeec0"
        )

        internal const val METRICS_SCHEDULER_PREFS_FILE =
            "$GLEAN_AC_PACKAGE_NAME.scheduler.MetricsPingScheduler"
        internal const val MIGRATION_PREFS_FILE = "$GLEAN_AC_PACKAGE_NAME.GleanACDataMigrator"
        internal const val SEQUENCE_NUMBERS_FILENAME = "$GLEAN_AC_PACKAGE_NAME.ping.PingMaker"
    }

    internal val migrationPrefs: SharedPreferences? by lazy {
        applicationContext.getSharedPreferences(
            MIGRATION_PREFS_FILE,
            Context.MODE_PRIVATE
        )
    }

    // The storage engines data is migrated from.
    private val uuidStorageEngine by lazy { UuidsStorageEngine(applicationContext) }
    private val dateTimeStorageEngine by lazy { DatetimesStorageEngine(applicationContext) }

    /**
     * A data class representing the metadata from glean-ac.
     *
     * @param alreadyMigrated whether or not migration was already performed; if true
     *        the remaining metadata is not loaded and will be `null`.
     * @param sequenceNumbers the mapping between AC storage names and their sequence
     *        numbers.
     * @param metricsPingLastSentDate the last time the metrics ping was sent (if ever).
     */
    internal data class ACMetadata(
        val alreadyMigrated: Boolean,
        val sequenceNumbers: Map<String, Int>,
        val metricsPingLastSentDate: String?
    ) {
        /**
         * Get a [Triple] containing data to be passed to the FFI layer.
         *
         * @return a [Triple] containing the keys and the values representing the sequence
         *         numbers map, with the addition of the number of elements in the map.
         */
        fun toFfi(): Triple<StringArray?, IntArray?, Int> {
            // The Map is sent over FFI as a pair of arrays, one containing the
            // keys, and the other containing the values.
            // In Kotlin, Map.keys and Map.values are not guaranteed to return the entries
            // in any particular order. Therefore, we iterate over the pairs together and
            // create the keys and values arrays step-by-step.
            val len = sequenceNumbers.size

            if (len == 0) {
                return Triple(null, null, 0)
            }

            val seqList = sequenceNumbers.toList()
            val keys = StringArray(Array<String>(sequenceNumbers.size) { seqList[it].first }, "utf-8")
            val values = IntArray(sequenceNumbers.size) { seqList[it].second }

            return Triple(keys, values, len)
        }
    }

    /**
     * Get the metadata from glean-ac.
     *
     * @return an instance of [ACMetadata] with, at least, [ACMetadata.alreadyMigrated].
     */
    fun getACMetadata(): ACMetadata {
        if (wasMigrated()) {
            return ACMetadata(true, emptyMap(), null)
        }

        return ACMetadata(
            alreadyMigrated = false,
            sequenceNumbers = getSequenceNumbers(),
            metricsPingLastSentDate = getMetricsPingLastSentDate()
        )
    }

    /**
     * Load the last time 'metrics' ping was sent in glean-ac [SharedPreferences].
     *
     * @return a `String?` that contains the date or `null` if there was a problem.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun getMetricsPingLastSentDate(): String? {
        val metricsPingPrefs = applicationContext.getSharedPreferences(
            METRICS_SCHEDULER_PREFS_FILE,
            Context.MODE_PRIVATE
        )

        return try {
            metricsPingPrefs?.getString("last_metrics_ping_iso_datetime", null)
        } catch (e: ClassCastException) {
            // If another pref in this file exists with a non string value,
            // something probably went wrong in the migration. Forget about this
            // and do not migrate.
            null
        }
    }

    /**
     * Load the sequence numbers that were stored in glean-ac [SharedPreferences].
     *
     * @return a `Map<String, Int>` in which entries are (`<ping_name>_seq`, nextSequenceNumber).
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun getSequenceNumbers(): Map<String, Int> {
        val seqPrefs = applicationContext.getSharedPreferences(
            SEQUENCE_NUMBERS_FILENAME,
            Context.MODE_PRIVATE
        )

        @Suppress("TooGenericExceptionCaught")
        return try {
            // The code in this block might throw. It probably means the file
            // does not exist or the data is corrupted. We try our best to recover
            // most of the data, though.
            seqPrefs
                .all
                .entries
                .filter {
                    it.key.endsWith("_seq") &&
                    it.value is Int &&
                    it.value as Int >= 0
                }
                .map {
                    val seq = it.value as Int
                    it.key to seq
                }.toMap()
        } catch (e: NullPointerException) {
            emptyMap()
        }
    }

    /**
     * Mark the current client as migrated.
     */
    fun markAsMigrated() {
        migrationPrefs?.edit()?.putBoolean("wasMigrated", true)?.apply()
    }

    /**
     * Migrate all the metrics that have User lifetime.
     */
    fun migrateUserLifetimeMetrics() {
        uuidStorageEngine.migrateToGleanCore(Lifetime.User)
        dateTimeStorageEngine.migrateToGleanCore(Lifetime.User)
    }

    /**
     * Migrate all the metrics that have Ping lifetime.
     */
    fun migratePingLifetimeMetrics() {
        uuidStorageEngine.migrateToGleanCore(Lifetime.Ping)
        dateTimeStorageEngine.migrateToGleanCore(Lifetime.Ping)
    }

    /**
     * Return any previously stored AC client id.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun getStoredClientId(): UUID? {
        val clientId = uuidStorageEngine
            .getSnapshot("glean_client_info", false)
            ?.get("client_id")

        return if (clientId == KNOWN_CLIENT_ID) {
            // Having a KNOWN_CLIENT_ID is like having no client id for
            // migration purposes.
            null
        } else {
            clientId
        }
    }

    /**
     * Get if the current client was migrated.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun wasMigrated(): Boolean {
        val defaultValue = false
        return try {
            migrationPrefs?.getBoolean("wasMigrated", defaultValue) ?: defaultValue
        } catch (e: ClassCastException) {
            // If another pref in this file exists with a non boolean value,
            // something probably went wrong in the migration. Let's try again?
            defaultValue
        }
    }

    /**
     * Check if data migration should be performed.
     *
     * @return true if glean-ac data was found and needs to be moved over,
     *         false otherwise.
     */
    internal fun shouldMigrateData(): Boolean {
        if (wasMigrated()) {
            return false
        }

        // Only migrate if we have a previously generated client id.
        return getStoredClientId() != null
    }

    /**
     * Clears any previously saved migration status.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun testResetMigrationStatus() {
        migrationPrefs?.edit()?.clear()?.apply()
    }
}
