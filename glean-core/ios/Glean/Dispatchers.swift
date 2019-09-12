/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class manages background execution of Glean tasks.
///
/// This class makes use of the higher level `Operation` and `OperationQueue` API's in order to allow
/// for observable background operation with the capabilities to pause, cancel, and resume tasks
class Dispatchers {
    /// This is the shared singleton access to the Glean Dispatchers
    static let shared = Dispatchers()

    // This struct is used for organizational purposes to keep the class constants in a single place
    struct Constants {
        static let logTag = "glean/Dispatchers"
        static let maxQueueSize = 100
    }

    // This is the task queue that all Glean background operations will be executed on.  It is currently
    // set to be a serial queue by setting the `maxConcurrentOperationsCount` to 1, but could allow for
    // concurrent operations by increasing the parameter.
    lazy var operationQueue: OperationQueue = {
        var queue = OperationQueue()
        queue.name = "Glean dispatch queue"
        queue.maxConcurrentOperationCount = 1
        return queue
    }()

    // When true, jobs will be queued and not ran until triggered by calling `flushQueuedInitialTasks()`
    private var queueInitialTasks = AtomicBoolean(true)

    // This array will hold the queued initial tasks that are launched before Glean is initialized
    lazy var preInitOperations: [Operation] = {
        [Operation]()
    }()

    // When true, jobs will be run synchronously
    var testingMode = false

    // Don't let other instances be created, we only want singleton access through the static `shared`
    // property
    private init() {}

    /// This function launches a background `Operation`
    ///
    /// - parameters:
    ///   * block: The block of code to execute, as a Closure that accepts no arugments and returns Void
    ///
    /// This function is used throughout Glean in order to launch tasks in the background, typically
    /// recording of metrics, uploading of pings, etc.  This function returns an `Operation` which can
    /// be used in conjunction with a completion handler to perform work after the task is complete.
    ///
    /// For example:
    /// ```
    /// Dispatchers.shared.launch {
    ///     // Do some work here
    /// }.completionBlock = {
    ///     print("Done!")
    /// }
    /// ```
    ///
    /// You could also use the returned `Operation` to await that specific task like this:
    /// ```
    /// let operation = Dispatchers.shared.launch {
    ///     // Do some work here
    /// }
    /// operation.waitUntilFinished()
    func launch(block: @escaping () -> Void) -> Operation {
        let operation = GleanOperation(block)

        if queueInitialTasks.value {
            // If we are queuing tasks, typically before Glean has been initialized, then we should
            // just add the created Operation to the preInitOperations array, provided there are
            // less than the max queued operations stored.
            if preInitOperations.count < Constants.maxQueueSize {
                preInitOperations.append(operation)

                if testingMode {
                    NSLog("\(Constants.logTag) : Task queued for execution in test mode")
                } else {
                    NSLog("\(Constants.logTag) : Task queued for execution and delayed until flushed")
                }
            } else {
                NSLog("\(Constants.logTag) : Exceeded maximum queue size, discarding task")
            }
        } else {
            // If we are not queuing initial tasks, we can go ahead and execute the Operation by
            // adding it to the `operationQueue`
            if testingMode {
                // If we are in testing mode, go ahead and execute the operation and wait until it
                // is finished before continuing to give us synchronous execution of the task.
                operation.start()
                operation.waitUntilFinished()
            } else {
                operationQueue.addOperation(operation)
            }
        }

        return operation
    }

    /// Stop queuing tasks and process any tasks in the queue.
    func flushQueuedInitialTasks() {
        // Add all of the queued operations to the `operationQueue` which will cause them to be
        // executed serially in the order they were collected.  We are passing `testingMode` to the
        // `waitUntilFinished` parameter since this is a serial queue and any newly queued tasks
        // should execute after the `preInitOperations` that are being added here. For tests, we
        // need to await all of the tasks to finish execution, so we set this to true.
        self.operationQueue.addOperations(preInitOperations, waitUntilFinished: testingMode)

        // Turn off queuing to allow for normal background execution mode
        queueInitialTasks.value = false

        // Clear the cached operations
        preInitOperations.removeAll()
    }

    /// Helper function to ensure the Glean SDK is being used in testing mode.
    ///
    /// This ensures that async jobs are being run synchronously. This should be called from every
    /// method in the testing API to make sure that the results of the main API can be tested as
    /// expected.
    func assertInTestingMode() {
        assert(
            testingMode,
            "To use the testing API, Glean must be in testing mode by calling Glean.enableTestingMode()"
        )
    }

    /// Enable/Disable testing mode.
    ///
    /// - parameters:
    ///   * enabled: `Bool` whether or not to enable the testing mode
    ///
    /// Enabling testing mode forces the public API functions to execute in a synchronous manner.
    public func setTestingMode(enabled: Bool) {
        testingMode = enabled
    }

    /// Enable queueing mode
    ///
    /// - parameters:
    ///   * enabled: `Bool` whether or not to enable the queuing mode
    ///
    /// When enabled, tasks are queued until launched by calling `flushQueuedInitialTasks()`
    func setTaskQueuing(enabled: Bool) {
        queueInitialTasks.value = enabled
    }

    /// This is an internal class that represents a Glean background task
    class GleanOperation: Operation {
        let backgroundTask: () -> Void

        init(_ block: @escaping () -> Void) {
            backgroundTask = block
        }

        override func main() {
            if isCancelled {
                return
            }

            backgroundTask()
        }
    }
}
