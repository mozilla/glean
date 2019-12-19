/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.utils

import java.lang.StringBuilder
import java.text.SimpleDateFormat
import mozilla.telemetry.glean.private.TimeUnit
import java.util.Calendar
import java.util.Date
import java.util.Locale
import java.util.TimeZone

@Suppress("TopLevelPropertyNaming")
internal val DATE_FORMAT_PATTERNS = mapOf(
    TimeUnit.Nanosecond to "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
    TimeUnit.Microsecond to "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
    TimeUnit.Millisecond to "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
    TimeUnit.Second to "yyyy-MM-dd'T'HH:mm:ssZ",
    TimeUnit.Minute to "yyyy-MM-dd'T'HH:mmZ",
    TimeUnit.Hour to "yyyy-MM-dd'T'HHZ",
    TimeUnit.Day to "yyyy-MM-ddZ"
)

// A mapping from the length of the date string to the format that would parse
// it.
@Suppress("TopLevelPropertyNaming")
internal val DATE_FORMAT_PATTERN_BY_LENGTH = mapOf(
    28 to "yyyy-MM-dd'T'HH:mm:ss.SSSZ",
    24 to "yyyy-MM-dd'T'HH:mm:ssZ",
    21 to "yyyy-MM-dd'T'HH:mmZ",
    18 to "yyyy-MM-dd'T'HHZ",
    15 to "yyyy-MM-ddZ"
)

internal val DATE_FORMAT_PATTERN_VALUES = DATE_FORMAT_PATTERNS.values.toSet()

/**
 * Generate an ISO8601 compliant time string for the given time.
 *
 * @param date the [Date] object to convert to string
 * @param truncateTo The TimeUnit to truncate the value to
 * @return a string containing the date, time and timezone offset
 */
internal fun getISOTimeString(
    date: Date = Date(),
    truncateTo: TimeUnit = TimeUnit.Minute
): String {
    val cal = Calendar.getInstance()
    cal.setTime(date)
    return getISOTimeString(cal, truncateTo)
}

/**
 * Generate an ISO8601 compliant time string for the given time.
 *
 * @param calendar the [Calendar] object to convert to string
 * @param truncateTo The TimeUnit to truncate the value to
 * @return a string containing the date, time and timezone offset
 */
internal fun getISOTimeString(
    calendar: Calendar,
    truncateTo: TimeUnit = TimeUnit.Minute
): String {
    val dateFormat = SimpleDateFormat(DATE_FORMAT_PATTERNS[truncateTo], Locale.US)
    dateFormat.setTimeZone(calendar.getTimeZone())
    val timeString = StringBuilder(dateFormat.format(calendar.getTime()))

    // Due to limitations of SDK version 21, there isn't a way to properly output the time
    // offset with a ':' character:
    // 2018-12-19T12:36:00-0600    -- This is what we get
    // 2018-12-19T12:36:00-06:00   -- This is what GCP will expect
    //
    // In order to satisfy time offset requirements of GCP, we manually insert the ":"
    timeString.insert(timeString.length - 2, ":")

    return timeString.toString()
}

/**
 * Parses the subset of ISO8601 datetime strings generated by [getISOTimeString].
 *
 * Always returns the result in the device's current timezone offset, regardless of the
 * timezone offset specified in the string.
 *
 * @param date a [String] representing an ISO date string generated by [getISOTimeString]
 * @return a [Date] object representation of the provided string
 */
@Suppress("MagicNumber")
internal fun parseISOTimeString(date: String): Date? {
    // Due to limitations of SDK version 21, there isn't a way to properly parse the time
    // offset with a ':' character:
    // 2018-12-19T12:36:00-06:00  -- This is what we store
    // 2018-12-19T12:36:00-0600   -- This is what SimpleDateFormat will expect

    val correctedDate = if (date.get(date.length - 3) == ':') {
        date.substring(0, date.length - 3) + date.substring(date.length - 2)
    } else {
        date
    }

    DATE_FORMAT_PATTERN_BY_LENGTH.get(correctedDate.length)?.let {
        val dateFormat = SimpleDateFormat(it, Locale.US)
        try {
            return dateFormat.parse(correctedDate)
        } catch (e: java.text.ParseException) {
            // fall through
        }
    }

    // Fall back to trying all formats if the obvious choice by length doesn't
    // work
    for (format in DATE_FORMAT_PATTERN_VALUES) {
        val dateFormat = SimpleDateFormat(format, Locale.US)
        try {
            return dateFormat.parse(correctedDate)
        } catch (e: java.text.ParseException) {
            continue
        }
    }

    return null
}

/**
 * Parses the subset of ISO8601 datetime strings generated by [getISOTimeString].
 *
 * Always returns the result in the device's current timezone offset, regardless of the
 * timezone offset specified in the string.
 *
 * @param date a [String] representing an ISO date string generated by [getISOTimeString]
 * @return a [Date] object representation of the provided string
 */
@Suppress("MagicNumber")
internal fun parseISOTimeStringAsCalendar(date: String): Calendar? {
    // Due to limitations of SDK version 21, there isn't a way to properly parse the time
    // offset with a ':' character:
    // 2018-12-19T12:36:00-06:00  -- This is what we store
    // 2018-12-19T12:36:00-0600   -- This is what SimpleDateFormat will expect

    val correctedDate = if (date.get(date.length - 3) == ':') {
        date.substring(0, date.length - 3) + date.substring(date.length - 2)
    } else {
        date
    }

    // We want the timezone offset of the calendar value to be exactly what is
    // specified in the offset of the string. However, leaving SimpleDateFormat.parse
    // to parse the string will simply convert the offset to the local timezone, or
    // the timezone specified in the Calendar.getInstance constructor.  Here, we parse
    // the offset suffix of the string to get a TimeZone object and apply that to the
    // Calendar instance being created.
    val offset = "GMT" + correctedDate.substring(correctedDate.length - 5, correctedDate.length)
    val timeZone = TimeZone.getTimeZone(offset)

    for (format in DATE_FORMAT_PATTERN_VALUES) {
        val dateFormat = SimpleDateFormat(format, Locale.US)
        try {
            val cal = Calendar.getInstance(timeZone, Locale.US)
            cal.clear()
            dateFormat.calendar = cal
            dateFormat.parse(correctedDate)
            return cal
        } catch (e: java.text.ParseException) {
            continue
        }
    }

    return null
}
