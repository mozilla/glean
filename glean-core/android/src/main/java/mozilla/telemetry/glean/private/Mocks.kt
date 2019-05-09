/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * An enum with no values for convenient use as the default set of extra keys
 * that an [EventMetricType] can accept.
 */
@Suppress("EmptyClassBlock")
enum class NoExtraKeys(val value: Int) {
    // deliberately empty
}
