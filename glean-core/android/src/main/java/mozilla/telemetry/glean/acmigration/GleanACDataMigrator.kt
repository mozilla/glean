/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration

import android.content.Context
import android.content.SharedPreferences
import androidx.annotation.VisibleForTesting
import com.sun.jna.StringArray
import java.lang.NullPointerException

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
        internal const val SEQUENCE_NUMBERS_FILENAME = "mozilla.components.service.glean.ping.PingMaker"
        internal const val MIGRATION_PREFS_FILE = "mozilla.components.service.glean.GleanACDataMigrator"
    }

    internal val migrationPrefs: SharedPreferences? by lazy {
        applicationContext.getSharedPreferences(
            MIGRATION_PREFS_FILE,
            Context.MODE_PRIVATE
        )
    }

    /**
     * A data class representing the metadata from glean-ac.
     *
     * @param alreadyMigrated whether or not migration was already performed; if true
     *        the remaining metadata is not loaded and will be `null`.
     * @param sequenceNumbers the mapping between AC storage names and their sequence
     *        numbers.
     * @return a [Triple] containing the keys and the values representing the sequence
     *         numbers map, with the addition of the number of elements in the map.
     */
    internal data class ACMetadata(
        val alreadyMigrated: Boolean,
        val sequenceNumbers: Map<String, Int>
    ) {
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
            return ACMetadata(true, emptyMap())
        }

        return ACMetadata(
            alreadyMigrated = false,
            sequenceNumbers = getSequenceNumbers()
        )
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
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun markAsMigrated() {
        migrationPrefs?.edit()?.putBoolean("wasMigrated", true)?.apply()
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

        // TODO: check if we have any previous a-c data (i.e. client_id). If so, migrate.

        return true
    }

    /**
     * Clears any previously saved migration status.
     */
    @VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
    internal fun testResetMigrationStatus() {
        migrationPrefs?.edit()?.clear()?.apply()
    }
}
