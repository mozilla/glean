/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Glean
import UIKit

typealias Basic = GleanMetrics.Basic
typealias Custom = GleanMetrics.Custom
typealias Test = GleanMetrics.Test
typealias Pings = GleanMetrics.Pings

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?
    let glean = Glean.shared

    // swiftlint:disable line_length
    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        glean.registerPings(Pings.shared)

        // Set a "fake" legacy client id for the purpose of testing the deletion-request ping payload
        if let fakeLegacyId = UUID(uuidString: "01234567-89ab-cdef-0123-456789abcdef") {
            GleanMetrics.LegacyIds.clientId.set(fakeLegacyId)
        }

        let mockServerIndex = ProcessInfo.processInfo.arguments.firstIndex(of: "USE_MOCK_SERVER")
        if let idx = mockServerIndex {
            let portIdx = idx + 1
            var address = "http://localhost:9080"
            if portIdx < ProcessInfo.processInfo.arguments.count {
                let port = ProcessInfo.processInfo.arguments[portIdx]
                address = "http://localhost:\(port)"
            }

            print("using a mock server, setting address: \(address)")
            let cfg = Configuration(serverEndpoint: address)
            glean.initialize(uploadEnabled: true, configuration: cfg)
        } else {
            print("using default config for Glean")
            glean.initialize(uploadEnabled: true)
        }

        Test.timespan.start()

        // Set a sample value for a metric.
        Basic.os.set("iOS")

        return true
    }

    func application(_: UIApplication,
                     open url: URL,
                     options _: [UIApplication.OpenURLOptionsKey: Any] = [:]) -> Bool {
        // This does nothing if the url isn't meant for Glean.
        glean.handleCustomUrl(url: url)

        return true
    }

    // swiftlint:enable line_length

    func applicationWillResignActive(_: UIApplication) {
        // Sent when the application is about to move from active to inactive state.
        // This can occur for certain types of temporary interruptions
        // (such as an incoming phone call or SMS message) or when the user quits the application
        // and it begins the transition to the background state.
        // Use this method to pause ongoing tasks, disable timers,
        // and invalidate graphics rendering callbacks.
        // Games should use this method to pause the game.
    }

    func applicationDidEnterBackground(_: UIApplication) {
        // Use this method to release shared resources, save user data, invalidate timers,
        // and store enough application state information to restore your application
        // to its current state in case it is terminated later.
        // If your application supports background execution,
        // this method is called instead of applicationWillTerminate: when the user quits.
    }

    func applicationWillEnterForeground(_: UIApplication) {
        // Called as part of the transition from the background to the active state;
        // here you can undo many of the changes made on entering the background.
    }

    func applicationDidBecomeActive(_: UIApplication) {
        // Restart any tasks that were paused (or not yet started) while the application was inactive.
        // If the application was previously in the background, optionally refresh the user interface.
    }

    func applicationWillTerminate(_: UIApplication) {
        // Called when the application is about to terminate. Save data if appropriate.
        // See also applicationDidEnterBackground:.
    }
}
