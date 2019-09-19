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
        // Note that this is sending the length of the last foreground session
        // because it belongs to the baseline ping and that ping is sent every
        // time the app goes to background.
        GleanBaseline.duration.start()
    }

    @objc func appDidEnterBackground(notification _: NSNotification) {
        // We're going to background, so store how much time we spent
        // on foreground.
        GleanBaseline.duration.stop()
        Glean.shared.handleBackgroundEvent()
    }
}
