/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class manages a single background operation queue.
class Dispatchers {
    /// This is the shared singleton access to the Glean Dispatchers
    static let shared = Dispatchers()

    // Don't let other instances be created, we only want singleton access through the static `shared`
    // property
    private init() {}

    // This is a task queue for background operations that are required to be executed in order.
    // It is currently set to be a serial queue by setting the `maxConcurrentOperationsCount` to 1.
    // This queue is intended for API operations that are subject to the behavior and constraints of the
    // API.
    lazy var serialOperationQueue: OperationQueue = {
        var queue = OperationQueue()
        queue.name = "Glean serial dispatch queue"
        queue.maxConcurrentOperationCount = 1
        return queue
    }()

    func launchAsync(block: @escaping () -> Void) {
        serialOperationQueue.addOperation(BlockOperation(block: block))
    }
}
