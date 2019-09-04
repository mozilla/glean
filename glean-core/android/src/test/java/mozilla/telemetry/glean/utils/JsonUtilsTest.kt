/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import androidx.test.ext.junit.runners.AndroidJUnit4
import org.json.JSONArray
import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class JsonUtilsTest {
    private lateinit var testData2Elements: JSONArray

    @Before
    fun setUp() {
        testData2Elements = JSONArray().apply {
            put(JSONObject("""{"a": 1}"""))
            put(JSONObject("""{"b": 2}"""))
        }
    }

    @Test
    fun tryGetLongNull() {
        val jsonObject = JSONObject("""{"key":null}""")
        assertNull(jsonObject.tryGetLong("key"))
        assertNull(jsonObject.tryGetLong("another-key"))
    }

    @Test
    fun tryGetLongNotNull() {
        val jsonObject = JSONObject("""{"key":218728173837192717}""")
        assertEquals(218728173837192717, jsonObject.tryGetLong("key"))
    }

    @Test
    fun itCanBeIterated() {
        val array = JSONArray("[1, 2, 3]")

        val sum = array.asSequence()
            .map { it as Int }
            .sum()

        assertEquals(6, sum)
    }

    @Test
    fun toListNull() {
        val jsonArray: JSONArray? = null
        val list = jsonArray.toList<Any>()
        assertEquals(0, list.size)
    }

    @Test
    fun toListEmpty() {
        val jsonArray = JSONArray()
        val list = jsonArray.toList<Any>()
        assertEquals(0, list.size)
    }

    @Test
    fun toListNotEmpty() {
        val jsonArray = JSONArray()
        jsonArray.put("value")
        jsonArray.put("another-value")
        val list = jsonArray.toList<String>()
        assertEquals(2, list.size)
        assertTrue(list.contains("value"))
        assertTrue(list.contains("another-value"))
    }
}
