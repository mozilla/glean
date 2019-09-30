/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration.engines

import android.content.Context
import android.content.SharedPreferences
import android.util.Log
import mozilla.telemetry.glean.private.Lifetime

/**
 * Defines an alias for a generic data storage to be used by
 * [GenericStorageEngine]. This maps a metric name to
 * its data.
 */
internal typealias GenericDataStorage<T> = MutableMap<String, T>

/**
 * Defines an alias for a generic storage map to be used by
 * [GenericStorageEngine]. This maps a store name to
 * the [GenericDataStorage] it holds.
 */
internal typealias GenericStorageMap<T> = MutableMap<String, GenericDataStorage<T>>

/**
 * A base class for common metric storage functionality. This allows sharing the common
 * store managing and lifetime behaviours.
 */
internal abstract class GenericStorageEngine<MetricType> {
    companion object {
        const val LOG_TAG = "GenericStorageEngine"
    }

    lateinit var applicationContext: Context

    protected val userLifetimeStorage: SharedPreferences by lazy {
        deserializeLifetime(Lifetime.User)
    }
    protected val pingLifetimeStorage: SharedPreferences by lazy {
        deserializeLifetime(Lifetime.Ping)
    }

    // Store a map for each lifetime as an array element:
    // Array[Lifetime] = Map[StorageName, MetricType].
    protected val dataStores: Array<GenericStorageMap<MetricType>> =
        Array(Lifetime.values().size) { mutableMapOf<String, GenericDataStorage<MetricType>>() }

    /**
     * Implementor's provided function to convert deserialized 'user' lifetime
     * data to the destination [MetricType].
     *
     * @param metricName the name of the metric being deserialized
     * @param value loaded from the storage as [Any]
     *
     * @return data as [MetricType] or null if deserialization failed
     */
    protected abstract fun deserializeSingleMetric(metricName: String, value: Any?): MetricType?

    /**
     * Deserialize the metrics with a particular lifetime that are on disk.
     * This will be called the first time a metric is used or before a snapshot is
     * taken.
     *
     * @param lifetime a [Lifetime] to deserialize
     * @return A [SharedPreferences] reference that will be used to initialize [pingLifetimeStorage]
     *         or [userLifetimeStorage] or null if an invalid lifetime is used.
     */
    @Suppress("TooGenericExceptionCaught", "ComplexMethod")
    open fun deserializeLifetime(lifetime: Lifetime): SharedPreferences {
        require(lifetime == Lifetime.Ping || lifetime == Lifetime.User) {
            "deserializeLifetime does not support Lifetime.Application"
        }

        val prefsName = if (lifetime == Lifetime.Ping) {
            "${this.javaClass.canonicalName}.PingLifetime"
        } else {
            this.javaClass.canonicalName
        }
        val prefs = applicationContext.getSharedPreferences(prefsName, Context.MODE_PRIVATE)

        val metrics = try {
            prefs.all.entries
        } catch (e: NullPointerException) {
            // If we fail to deserialize, we can log the problem but keep on going.
            Log.e(
                LOG_TAG,
                "Failed to deserialize metric with ${lifetime.name} lifetime"
            )
            return prefs
        }

        for ((metricStoragePath, metricValue) in metrics) {
            if (!metricStoragePath.contains('#')) {
                continue
            }

            // Split the stored name in 2: we expect it to be in the format
            // store#metric.name
            val (storeName, metricName) =
                metricStoragePath.split('#', limit = 2)
            if (storeName.isEmpty()) {
                continue
            }

            val storeData = dataStores[lifetime.ordinal].getOrPut(storeName) { mutableMapOf() }
            // Only set the stored value if we're able to deserialize the persisted data.
            deserializeSingleMetric(metricName, metricValue)?.let { value ->
                storeData[metricName] = value
            } ?: Log.w(LOG_TAG, "Failed to deserialize $metricStoragePath")
        }

        return prefs
    }

    /**
     * Ensures that the lifetime metrics in [pingLifetimeStorage] and [userLifetimeStorage] is
     * loaded.  This is a no-op if they are already loaded.
     */
    private fun ensureAllLifetimesLoaded() {
        // Make sure data with the provided lifetime is loaded.
        // We still need to catch exceptions here, as `getAll()` might throw.
        @Suppress("TooGenericExceptionCaught")
        try {
            pingLifetimeStorage.all
            userLifetimeStorage.all
        } catch (e: NullPointerException) {
            // Intentionally left blank. We just want to fall through.
        }
    }

    /**
     * Retrieves the [recorded metric data][MetricType] for the provided
     * store name.
     *
     * Please note that the [Lifetime.Application] lifetime is handled implicitly
     * by never clearing its data. It will naturally clear out when restarting the
     * application.
     *
     * @param storeName the name of the desired store
     * @param clearStore whether or not to clear the requested store. Not that only
     *        metrics stored with a lifetime of [Lifetime.Ping] will be cleared.
     *
     * @return the [MetricType] recorded in the requested store
     */
    @Synchronized
    fun getSnapshot(storeName: String, clearStore: Boolean): GenericDataStorage<MetricType>? {
        val allLifetimes: GenericDataStorage<MetricType> = mutableMapOf()

        ensureAllLifetimesLoaded()

        // Get the metrics for all the supported lifetimes.
        for (store in dataStores) {
            store[storeName]?.let {
                allLifetimes.putAll(it)
            }
        }

        if (clearStore) {
            // We only allow clearing metrics with the "ping" lifetime.
            val editor = pingLifetimeStorage.edit()
            dataStores[Lifetime.Ping.ordinal][storeName]?.keys?.forEach { key ->
                editor.remove("$storeName#$key")
            }
            editor.apply()
            dataStores[Lifetime.Ping.ordinal].remove(storeName)
        }

        return if (allLifetimes.isNotEmpty()) allLifetimes else null
    }

    /**
     * Perform the data migration for the given Lifetime.
     *
     * @param lifetime the lifetime to migrate. Only metrics with this lifetime will
     *        be migrated. Note that `Application` lifetime is not supported.
     */
    open fun migrateToGleanCore(lifetime: Lifetime) {
        ensureAllLifetimesLoaded()

        // No need to attempt to migrate metrics with Application lifetime.
        check(lifetime != Lifetime.Application)
    }
}
