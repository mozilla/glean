/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.testing

import android.content.SharedPreferences

internal class MockSharedPreferences : SharedPreferences {
    override fun contains(key: String?): Boolean {
        TODO("Not yet implemented")
    }

    override fun getBoolean(key: String?, defValue: Boolean): Boolean {
        TODO("Not yet implemented")
    }

    override fun unregisterOnSharedPreferenceChangeListener(listener: SharedPreferences.OnSharedPreferenceChangeListener?) {
        TODO("Not yet implemented")
    }

    override fun getInt(key: String?, defValue: Int): Int {
        TODO("Not yet implemented")
    }

    override fun getAll(): MutableMap<String, *> {
        TODO("Not yet implemented")
    }

    override fun edit(): SharedPreferences.Editor = MockEditor()

    override fun getLong(key: String?, defValue: Long): Long {
        TODO("Not yet implemented")
    }

    override fun getFloat(key: String?, defValue: Float): Float {
        TODO("Not yet implemented")
    }

    override fun getStringSet(key: String?, defValues: MutableSet<String>?): MutableSet<String> {
        TODO("Not yet implemented")
    }

    override fun registerOnSharedPreferenceChangeListener(listener: SharedPreferences.OnSharedPreferenceChangeListener?) {
        TODO("Not yet implemented")
    }

    override fun getString(key: String?, defValue: String?): String? = null

    class MockEditor : SharedPreferences.Editor {
        override fun clear(): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun putLong(key: String?, value: Long): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun putInt(key: String?, value: Int): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun remove(key: String?): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun putBoolean(key: String?, value: Boolean): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun putStringSet(key: String?, values: MutableSet<String>?): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun commit(): Boolean {
            TODO("Not yet implemented")
        }

        override fun putFloat(key: String?, value: Float): SharedPreferences.Editor {
            TODO("Not yet implemented")
        }

        override fun apply() {
            // TODO("Not yet implemented")
            // Currently no-ops
        }

        override fun putString(key: String?, value: String?): SharedPreferences.Editor {
            // TODO("Not yet implemented")
            // Currently no-ops
            return this
        }

    }
}