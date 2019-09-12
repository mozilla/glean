//
//  DispatchersTest.swift
//  GleanTests
//
//  Created by Travis Long on 9/12/19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

@testable import Glean
import XCTest

class DispatchersTest: XCTestCase {
    func testTaskQueuing() {
        var threadCanary = 0

        Dispatchers.shared.setTestingMode(enabled: true)
        Dispatchers.shared.setTaskQueuing(enabled: true)

        // Add 3 tasks to the queue, each one incrementing threadCanary to indicate the task has executed
        for _ in 0 ..< 3 {
            _ = Dispatchers.shared.launch {
                threadCanary += 1
            }
        }

        XCTAssertEqual(
            Dispatchers.shared.preInitOperations.count,
            3,
            "Task queue continas the correct number of tasks"
        )
        XCTAssertEqual(
            threadCanary,
            0,
            "Tasks have not executed while in queue"
        )

        // Now trigger the queue to fire the tasks
        Dispatchers.shared.flushQueuedInitialTasks()

        XCTAssertEqual(
            threadCanary,
            3,
            "Tasks have executed"
        )
        XCTAssertEqual(
            Dispatchers.shared.preInitOperations.count,
            0,
            "Task queue has cleared"
        )
    }

    func testQueuedTasksAreExecutedInOrder() {
        // Create a test operation queue to run our operations asynchronously
        let testQueue = OperationQueue()
        testQueue.name = "Glean test queue"
        // Set to 2 to allow both of our tasks to run concurrently
        testQueue.maxConcurrentOperationCount = 2

        var orderedList = [Int]()
        var flushTasks = AtomicBoolean(false)

        Dispatchers.shared.setTestingMode(enabled: false)
        Dispatchers.shared.setTaskQueuing(enabled: true)

        // This background task will monitor the size of the cached initial
        // operations and attempt to flush it when it reaches 50 elements.
        // This should give us 50 items in the queued items and 50 that are
        // executed in the background normally.
        let op1 = Dispatchers.GleanOperation {
            while !flushTasks.val { sleep(1) }
            Dispatchers.shared.flushQueuedInitialTasks()
        }
        testQueue.addOperation(op1)

        // This background task will add elements to the orderedList.  This will continue to
        // add elements to the queue until there are at least 50 elements in the queue. At that
        // point, the background task above will flush and disable the queuing and this task will
        // continue launching tasks directly.
        var counter = 0
        let op2 = Dispatchers.GleanOperation {
            for num in 0 ... 99 {
                if num == 50 {
                    flushTasks.val = true
                }
                _ = Dispatchers.shared.launch {
                    orderedList.append(num)
                    counter += 1
                }
            }
        }
        testQueue.addOperation(op2)

        // Wait for the numbers to be added to the list by waiting for the operations above to complete
        op1.waitUntilFinished()
        op2.waitUntilFinished()

        // Wait for all of the elements to be added to the list before we check the ordering
        while counter < 100 { sleep(1) }

        // Make sure the elements match in the correct order
        for num in 0 ... 99 {
            XCTAssertEqual(
                num,
                orderedList[num],
                "This list is out of order, \(num) != \(orderedList[num])"
            )
        }
    }
}
