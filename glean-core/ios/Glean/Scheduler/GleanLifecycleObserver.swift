/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

class GleanLifecycleObserver {
    init() {
        NotificationCenter.default.addObserver(
            self,
            selector: #selector(GleanLifecycleObserver.appWillEnterForeground(notification:)),
            name: UIApplication.willEnterForegroundNotification,
            object: nil
        )

        NotificationCenter.default.addObserver(
            self,
            selector: #selector(GleanLifecycleObserver.appDidEnterBackground(notification:)),
            name: UIApplication.didEnterBackgroundNotification,
            object: nil
        )
    }

    @objc func appWillEnterForeground(notification _: NSNotification) {
        // TODO: start duration
    }

    @objc func appDidEnterBackground(notification _: NSNotification) {
        // TODO: stop duration

        Glean.shared.handleBackgroundEvent()
    }
}
