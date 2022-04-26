/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// Represents deserialized experiments data
public struct RecordedExperimentData {
    init?(json: [String: Any]) {
        guard let branch = json["branch"] as? String,
            let extra = json["extra"] as? [String: String]
        else {
            return nil
        }

        self.branch = branch
        self.extra = extra
    }

    // The experiment's branch as set through `setExperimentActive`
    let branch: String
    // Any extra data associated with this experiment through `setExperimentActive`
    let extra: [String: String]
}
