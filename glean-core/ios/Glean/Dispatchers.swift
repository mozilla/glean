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

    private let logger = Logger(tag: Constants.logTag)

    // This is a task queue for all Glean background operations that are required to be executed in order.
    // It is currently set to be a serial queue by setting the `maxConcurrentOperationsCount` to 1
    lazy var serialOperationQueue: OperationQueue = {
        var queue = OperationQueue()
        queue.name = "Glean serial dispatch queue"
        queue.maxConcurrentOperationCount = 1
        return queue
    }()

    // This is a task queue for all Glean background operations that can be executed concurrently.
    lazy var concurrentOperationsQueue: OperationQueue = {
        var queue = OperationQueue()
        queue.name = "Glean concurrent dispatch queue"
        return queue
    }()

    // When true, jobs will be queued and not ran until triggered by calling `flushQueuedInitialTasks()`
    private var queueInitialTasks = AtomicBoolean(true)

    // This array will hold the queued initial tasks that are launched before Glean is initialized
    lazy var preInitOperations = [Operation]()

    // When true, jobs will be run synchronously
    var testingMode = false

    // Don't let other instances be created, we only want singleton access through the static `shared`
    // property
    private init() {}

    /// This function launches an `Operation` on a serially executed queue.
    ///
    /// - parameters:
    ///   * block: The block of code to execute
    ///
    /// This function is used throughout Glean in order to launch tasks in the background, typically
    /// recording of metrics and things that need to execute in order. Since this executes the tasks on
    /// a non-concurrent (serial) queue, the tasks are executed in the order that they are launched.
    ///
    /// If `queueInitialTasks` is enabled, then the operation will be created and added to the
    /// `preInitOperations` array but not executed until flushed.
    ///
    /// If `testingMode` is enabled, then `launchAPI` will await the execution of the task (unless queuing is
    /// enabled)
    func launchAPI(block: @escaping () -> Void) {
        if queueInitialTasks.value {
            // If we are queuing tasks, typically before Glean has been initialized, then we should
            // just add the created Operation to the preInitOperations array, provided there are
            // less than the max queued operations stored.
            if preInitOperations.count < Constants.maxQueueSize {
                preInitOperations.append(BlockOperation(block: block))

                if testingMode {
                    logger.info("Task queued for execution in test mode")
                } else {
                    logger.info("Task queued for execution and delayed until flushed")
                }
            } else {
                logger.error("Exceeded maximum queue size, discarding task")
            }
        } else {
            // If we are not queuing initial tasks, we can go ahead and execute the Operation by
            // adding it to the `operationQueue`
            serialOperationQueue.addOperation(block)

            // If we are in testing mode, go ahead and wait until it is finished before continuing
            // to ensure synchronous execution of the task.
            if testingMode {
                serialOperationQueue.waitUntilAllOperationsAreFinished()
            }
        }
    }

    /// This function launches an `Operation` on a concurrently executed queue.
    ///
    /// - parameters:
    ///   * operation: The `Operation` to execute
    ///
    /// This function is used to execute tasks in an asynchrounous manner and still give us the ability
    /// to cancel the tasks by creating them as `Operation`s rather than using GCD.
    ///
    /// This function specifically ignores the `queueInitialTasks` flag because the only tasks that
    /// should be launched by this are the ping upload schedulers and those should run regardless of
    /// the initialized state.
    ///
    /// If `testingMode` is enabled, then `launchConcurrent` will just execute the task rather than
    /// adding it to the concurrent queue to avoid asynchronous issues while testing
    func launchConcurrent(operation: Operation) {
        if testingMode {
            operation.start()
            operation.waitUntilFinished()
        } else {
            concurrentOperationsQueue.addOperation(operation)
        }
    }

    /// This function launches an `Operation` on a concurrently executed queue.
    ///
    /// - parameters:
    ///   * block: The block of code to execute, as a Closure that accepts no arugments and returns Void
    ///
    /// This function is used to execute tasks in an asynchrounous manner and still give us the ability
    /// to cancel the tasks by creating them as `Operation`s rather than using GCD.
    ///
    /// This function specifically ignores the `queueInitialTasks` flag because the only tasks that
    /// should be launched by this are the ping upload schedulers and those should run regardless of
    /// the initialized state.
    ///
    /// If `testingMode` is enabled, then `launchConcurrent will just execute the task rather than
    /// adding it to the concurrent queue to avoid asynchronous issues while testing
    func launchConcurrent(block: @escaping () -> Void) {
        launchConcurrent(operation: BlockOperation(block: block))
    }

    /// Cancel any pending background tasks
    func cancelBackgroundTasks() {
        // This will remove all queued operations prior to Glean.initialize() so that they won't be
        // executed if flushQueuedInitialTasks() is called
        preInitOperations.removeAll()

        // This will cancel operations in the serially executed queue. This includes most of the
        // metrics recording and things that need to execute in order. This would not stop the currently
        // executing operation, but would prevent all remaining operations from executing.
        serialOperationQueue.cancelAllOperations()

        // This will cancel operations that are executing concurrently. This doesn't abort running
        // operations immediately and it is up to the operation to handle the cancel request,
        // otherwise it will just run to completion, so we shouldn't have to worry about cancellation
        // causing undefined behavior.
        concurrentOperationsQueue.cancelAllOperations()
    }

    /// Stop queuing tasks and process any tasks in the queue.
    func flushQueuedInitialTasks() {
        // Add all of the queued operations to the `operationQueue` which will cause them to be
        // executed serially in the order they were collected.  We are passing `testingMode` to the
        // `waitUntilFinished` parameter since this is a serial queue and any newly queued tasks
        // should execute after the `preInitOperations` that are being added here. For tests, we
        // need to await all of the tasks to finish execution, so we set this to true.

        // TODO(bug 1591094) Android has a timeout of 5 seconds for running preinit tasks.

        self.serialOperationQueue.addOperations(preInitOperations, waitUntilFinished: testingMode)

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
            "To use the testing API, Glean must be in testing mode by calling Glean.shared.enableTestingMode()"
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
}
