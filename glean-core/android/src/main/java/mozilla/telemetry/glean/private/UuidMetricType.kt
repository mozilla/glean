/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import android.util.Log
import androidx.annotation.VisibleForTesting
// import mozilla.components.service.glean.Dispatchers
import java.util.UUID

// import mozilla.components.service.glean.storages.UuidsStorageEngine
// import mozilla.components.support.base.log.logger.Logger

class UuidMetricType(
    disabled: Boolean,
    category: String,
    lifetime: Lifetime,
    name: String,
    private val sendInPings: List<String>
) {
    companion object {
        private val LOG_TAG: String = "glean/UuidMetricType"
    }

    /**
     * Generate a new UUID value and set it in the metric store.
     *
     * @return a [UUID] or [null] if we're not allowed to record.
     */
    fun generateAndSet(): UUID? {
        // Even if `set` is already checking if we're allowed to record,
        // we need to check here as well otherwise we'd return a `UUID`
        // that won't be stored anywhere.
        /*if (!shouldRecord(logger)) {
            return null
        }*/

        val uuid = UUID.randomUUID()
        set(uuid)
        return uuid
    }

    /**
     * Explicitly set an existing UUID value
     *
     * @param value a valid [UUID] to set the metric to
     */
    fun set(value: UUID) {
        /*if (!shouldRecord(logger)) {
            return
        }

        @Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.launch {
            // Delegate storing the event to the storage engine.
            UuidsStorageEngine.record(
                this@UuidMetricType,
                value = value
            )
        }*/
        Log.e(LOG_TAG, "UuidMetricType.set is a stub")
    }

    /**
     * Tests whether a value is stored for the metric for testing purposes only. This function will
     * attempt to await the last task (if any) writing to the the metric's storage engine before
     * returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return true if metric value exists, otherwise false
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testHasValue(pingName: String = sendInPings.first()): Boolean {
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return UuidsStorageEngine.getSnapshot(pingName, false)?.get(identifier) != null*/
        assert(false, { "Testing API not implemented for UuidMetricType" })
        return false
    }

    /**
     * Returns the stored value for testing purposes only. This function will attempt to await the
     * last task (if any) writing to the the metric's storage engine before returning a value.
     *
     * @param pingName represents the name of the ping to retrieve the metric for.  Defaults
     *                 to the either the first value in [defaultStorageDestinations] or the first
     *                 value in [sendInPings]
     * @return value of the stored metric
     * @throws [NullPointerException] if no value is stored
     */
    @VisibleForTesting(otherwise = VisibleForTesting.NONE)
    fun testGetValue(pingName: String = sendInPings.first()): UUID {
        /*@Suppress("EXPERIMENTAL_API_USAGE")
        Dispatchers.API.assertInTestingMode()

        return UuidsStorageEngine.getSnapshot(pingName, false)!![identifier]!!*/
        assert(false, { "Testing API not implemented for UuidMetricType" })
        return UUID.randomUUID()
    }
}
