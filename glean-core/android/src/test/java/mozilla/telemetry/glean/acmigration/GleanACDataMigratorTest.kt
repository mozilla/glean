/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.acmigration

import android.content.Context
import android.content.SharedPreferences
import androidx.test.core.app.ApplicationProvider
import androidx.test.ext.junit.runners.AndroidJUnit4
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertTrue
import org.junit.Test
import org.junit.runner.RunWith
import org.mockito.ArgumentMatchers

import org.mockito.Mockito.`when`
import org.mockito.Mockito.mock
import org.mockito.Mockito.spy
import java.lang.NullPointerException

@RunWith(AndroidJUnit4::class)
class GleanACDataMigratorTest {
    @Test
    fun `sequence numbers are correctly migrated`() {
        val persistedSeq = mapOf(
            "without_trailing_seq_but_valid" to 1,
            "valid_ping_seq" to 3785,
            "negative_ping_seq" to -3785,
            "null_seq" to null,
            "string_seq" to "test",
            "bool_seq" to "test"
        )

        // Create a fake application context that will be used to load our data.
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenAnswer { persistedSeq }
        `when`(context.getSharedPreferences(
            ArgumentMatchers.eq(GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME),
            ArgumentMatchers.eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)

        // Attempt a metadata migration.
        val migrator = GleanACDataMigrator(context)
        val migratedData = migrator.getSequenceNumbers()

        assertNotNull(migratedData)
        assertEquals(1, migratedData.size)
        assertEquals(3785, migratedData["valid_ping_seq"])
    }

    @Test
    fun `do not throw if the sequence number file does not exist`() {
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenThrow(NullPointerException())
        `when`(context.getSharedPreferences(
            ArgumentMatchers.eq(GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME),
            ArgumentMatchers.eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)

        // Attempt a metadata migration.
        val migrator = GleanACDataMigrator(context)
        val migratedData = migrator.getSequenceNumbers()

        // If SharedPreferences throws when we try to load, just return null.
        assertEquals(0, migratedData.size)
    }

    @Test
    fun `markAsMigrated correctly marks as migrated`() {
        // Mark the client as migrated.
        val writer = GleanACDataMigrator(ApplicationProvider.getApplicationContext())
        assertFalse(writer.wasMigrated())
        writer.markAsMigrated()

        val reader = GleanACDataMigrator(ApplicationProvider.getApplicationContext())
        assertTrue(reader.wasMigrated())
    }

    @Test
    fun `migration status is false when data is of the wrong type`() {
        val persistedStatus = mapOf(
            "wasMigrated" to "true"
        )

        // Create a fake application context that will be used to load our data.
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenAnswer { persistedStatus }
        `when`(context.getSharedPreferences(
            ArgumentMatchers.eq(GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME),
            ArgumentMatchers.eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)

        // Attempt a metadata migration.
        val migrator = GleanACDataMigrator(context)
        assertFalse(migrator.wasMigrated())
    }

    @Test
    fun `migration status is false if the data file is empty`() {
        val persistedStatus = emptyMap<String, Any>()

        // Create a fake application context that will be used to load our data.
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenAnswer { persistedStatus }
        `when`(context.getSharedPreferences(
            ArgumentMatchers.eq(GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME),
            ArgumentMatchers.eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)

        // Attempt a metadata migration.
        val migrator = GleanACDataMigrator(context)
        assertFalse(migrator.wasMigrated())
    }

    @Test
    fun `getACMetadata does not load AC data if client was migrated`() {
        val migrator =
            spy<GleanACDataMigrator>(GleanACDataMigrator(ApplicationProvider.getApplicationContext()))
        `when`(migrator.wasMigrated()).thenReturn(true)
        val meta = migrator.getACMetadata()
        assertTrue(meta.alreadyMigrated)
        assertEquals(
            "We must not reload the ac data if the client was migrated already",
            0,
            meta.sequenceNumbers.size
        )
    }

    @Test
    fun `getACMetadata loads AC data if client was not migrated`() {
        val persistedSeq = mapOf(
            "ping_one_seq" to 1,
            "ping_two_seq" to 2,
            "ping_three_seq" to 3
        )

        // Create a fake application context that will be used to load our data.
        val context = mock(Context::class.java)
        val sharedPreferences = mock(SharedPreferences::class.java)
        `when`(sharedPreferences.all).thenAnswer { persistedSeq }
        `when`(context.getSharedPreferences(
            ArgumentMatchers.eq(GleanACDataMigrator.SEQUENCE_NUMBERS_FILENAME),
            ArgumentMatchers.eq(Context.MODE_PRIVATE)
        )).thenReturn(sharedPreferences)

        val migrator = GleanACDataMigrator(context)
        // Mark the client as migrated.
        migrator.markAsMigrated()

        // Verify that the persisted sequence numbers are returned as part of
        // the metadata.
        val meta = migrator.getACMetadata()
        assertFalse(meta.alreadyMigrated)
        assertNotNull(meta.sequenceNumbers)
        assertEquals(3, meta.sequenceNumbers.size)
        assertEquals(1, meta.sequenceNumbers["ping_one_seq"])
        assertEquals(2, meta.sequenceNumbers["ping_two_seq"])
        assertEquals(3, meta.sequenceNumbers["ping_three_seq"])
    }
}
