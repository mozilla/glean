/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class manages background execution of Glean tasks.
///
/// This class makes use of the higher level `Operation` and `OperationQueue` API's in order to allow
/// for observable background operation with the capabilities to pause, cancel, and resume tasks.
///
/// There are two main queues created and used as part of `Dispatchers`.  The `serialOperationQueue`
/// is a serially executed, single-threaded queue meant for API tasks and is serviced through `launchAPI`, the
/// `concurrentOperationsQueue` is a concurrently executed queue meant for other heavy tasks that
/// should not be subject to the same behavior and constraints as the serial API queue and should not block other
/// tasks.
class Dispatchers {
    /// This is the shared singleton access to the Glean Dispatchers
    static let shared = Dispatchers()

    // This is a task queue for all Glean background operations that are required to be executed in order.
    // It is currently set to be a serial queue by setting the `maxConcurrentOperationsCount` to 1.
    // This queue is intended for API operations that are subject to the behavior and constraints of the
    // API.
    lazy var serialOperationQueue: OperationQueue = {
        var queue = OperationQueue()
        queue.name = "Glean serial dispatch queue"
        queue.maxConcurrentOperationCount = 1
        return queue
    }()

    /// Cancel any pending background tasks
    func cancelBackgroundTasks() {
        // This will cancel operations in the serially executed queue. This includes most of the
        // metrics recording and things that need to execute in order. This would not stop the currently
        // executing operation, but would prevent all remaining operations from executing.
        serialOperationQueue.cancelAllOperations()
    }
}
