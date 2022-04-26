/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import org.json.JSONArray
import org.json.JSONObject

/**
 * Returns the value mapped by {@code key} if it exists, and
 * if the value returned is not null. If it's null, it returns null
 */
fun JSONObject.tryGetLong(key: String): Long? = if (isNull(key)) null else getLong(key)

/**
 * Convenience method to convert a JSONArray into a sequence.
 *
 * @param getter callback to get the value for an index in the array.
 */
inline fun <V> JSONArray.asSequence(crossinline getter: JSONArray.(Int) -> V): Sequence<V> {
    val indexRange = 0 until length()
    return indexRange.asSequence().map { i -> getter(i) }
}

/**
 * Convenience method to convert a JSONArray into a sequence.
 */
fun JSONArray.asSequence(): Sequence<Any> = asSequence { i -> get(i) }

/**
 * Convenience method to convert a JSONArray into a List
 *
 * @return list with the JSONArray values, or an empty list if the JSONArray was null
 */
@Suppress("UNCHECKED_CAST")
fun <T> JSONArray?.toList(): List<T> {
    val array = this ?: return emptyList()
    return array.asSequence().map { it as T }.toList()
}
