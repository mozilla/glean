/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package mozilla.telemetry.glean.utils

import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.private.TimeUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith
import java.util.Calendar
import java.util.TimeZone

@RunWith(AndroidJUnit4::class)
class DateUtilsTest {
    private val originalTimeZone: TimeZone = TimeZone.getDefault()

    @After
    fun tearDown() {
        TimeZone.setDefault(originalTimeZone)
    }

    private fun calendarAt(
        zone: String,
        month: Int,
        day: Int,
        hour: Int,
    ): Calendar {
        TimeZone.setDefault(TimeZone.getTimeZone(zone))
        return Calendar.getInstance().apply {
            clear()
            set(2027, month, day, hour, 0)
        }
    }

    // An ordinary day, then the 2027 days on which DST starts and ends in each zone below. Egypt
    // and Chile change it at midnight, and the Azores start it at midnight.
    private val days = listOf(
        Calendar.JUNE to 10,
        Calendar.MARCH to 14,
        Calendar.NOVEMBER to 7,
        Calendar.MARCH to 28,
        Calendar.OCTOBER to 31,
        Calendar.APRIL to 30,
        Calendar.APRIL to 4,
        Calendar.SEPTEMBER to 5,
        Calendar.OCTOBER to 29,
    )

    @Test
    fun `test roundtripping ISO date formats`() {
        for (zone in listOf(
            "UTC",
            "America/New_York",
            "Europe/Berlin",
            "Africa/Cairo",
            "America/Santiago",
            "Atlantic/Azores",
        )) {
            for ((month, day) in days) {
                for (hour in listOf(0, 10, 23)) {
                    val calendar = calendarAt(zone, month, day, hour)
                    for (timeUnit in listOf(
                        TimeUnit.NANOSECOND,
                        TimeUnit.MICROSECOND,
                        TimeUnit.MILLISECOND,
                        TimeUnit.SECOND,
                        TimeUnit.MINUTE,
                        TimeUnit.HOUR,
                        TimeUnit.DAY,
                    )) {
                        val dateString = getISOTimeString(calendar, truncateTo = timeUnit)
                        val parsedDate = parseISOTimeString(dateString)!!
                        val regenDateString = getISOTimeString(parsedDate, truncateTo = timeUnit)
                        assertEquals("$zone $month/$day $hour:00 $timeUnit", dateString, regenDateString)
                    }
                }
            }
        }
    }
}
