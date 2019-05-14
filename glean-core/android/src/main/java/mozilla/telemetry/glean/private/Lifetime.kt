/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

/**
 * Enumeration of different metric lifetimes.
 */
enum class Lifetime {
    /**
     * The metric is reset with each sent ping
     */
    Ping,
    /**
     * The metric is reset on application restart
     */
    Application,
    /**
     * The metric is reset with each user profile
     */
    User
}
