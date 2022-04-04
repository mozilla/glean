/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

package mozilla.telemetry.glean.utils

import androidx.test.ext.junit.runners.AndroidJUnit4
import mozilla.telemetry.glean.private.TimeUnit
import org.junit.Assert.assertEquals
import org.junit.Test
import org.junit.runner.RunWith

@RunWith(AndroidJUnit4::class)
class DateUtilsTest {
    @Test
    fun `test roundtripping ISO date formats`() {
        for (
            timeUnit in listOf(
                TimeUnit.NANOSECOND,
                TimeUnit.MICROSECOND,
                TimeUnit.MILLISECOND,
                TimeUnit.SECOND,
                TimeUnit.MINUTE,
                TimeUnit.HOUR,
                TimeUnit.DAY
            )
        ) {
            val dateString = getISOTimeString(truncateTo = timeUnit)
            val parsedDate = parseISOTimeString(dateString)!!
            val regenDateString = getISOTimeString(parsedDate, truncateTo = timeUnit)
            assertEquals(dateString, regenDateString)
        }
    }
}
