/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

private typealias GleanBaseline = GleanMetrics.GleanBaseline

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

        // On init we start the duration, as we won't get the enter-foreground notification.
        GleanBaseline.duration.start()
    }

    @objc func appWillEnterForeground(notification _: NSNotification) {
        // Note that this is sending the length of the last foreground session
        // because it belongs to the baseline ping and that ping is sent every
        // time the app goes to background.
        Glean.shared.handleForegroundEvent()
        GleanBaseline.duration.start()

        // Set the "dirty flag" to `true`.
        Dispatchers.shared.launchAPI {
            glean_set_dirty_flag(true.toByte())
        }
    }

    @objc func appDidEnterBackground(notification _: NSNotification) {
        // We're going to background, so store how much time we spent
        // on foreground.
        GleanBaseline.duration.stop()
        Glean.shared.handleBackgroundEvent()

        // Clear the "dirty flag" as the last thing when going to background.
        // If the application is not being force-closed, we should still be
        // alive and allowed to change this. If we're being force-closed and
        // don't get to this point, next time Glean runs it will be detected.
        Dispatchers.shared.launchAPI {
            glean_set_dirty_flag(false.toByte())
        }
    }
}
