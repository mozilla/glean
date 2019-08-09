/* This Source Code Form is subject to the terms of the Mozilla Public
* License, v. 2.0. If a copy of the MPL was not distributed with this
* file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean.private

import androidx.annotation.VisibleForTesting

/**
 * Deserialized experiment data.
 */
@VisibleForTesting(otherwise = VisibleForTesting.PRIVATE)
data class RecordedExperimentData(
    /**
     * The experiment's branch as set through `setExperimentActive`.
     */
    val branch: String,
    /**
     * Any extra data associated with this experiment through `setExperimentActive`.
     */
    val extra: Map<String, String>? = null
)
