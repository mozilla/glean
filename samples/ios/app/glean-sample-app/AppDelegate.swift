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

    // swiftlint:disable line_length
    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        let glean = Glean.shared

        glean.registerPings(Pings.shared)
        glean.setUploadEnabled(true)

        if ProcessInfo.processInfo.arguments.contains("USE_MOCK_SERVER") {
            let address = "http://localhost:9080"
            print("using a mock server, setting address: \(address)")
            let cfg = Configuration(serverEndpoint: address)
            glean.initialize(configuration: cfg)
        } else {
            print("using default config for Glean")
            glean.initialize()
        }

        Test.timespan.start()

        Custom.counter.add()

        // Set a sample value for a metric.
        Basic.os.set("iOS")

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
