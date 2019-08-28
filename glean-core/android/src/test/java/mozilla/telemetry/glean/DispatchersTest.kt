/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.joinAll
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.junit.Test
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotSame
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue

@Suppress("EXPERIMENTAL_API_USAGE")
class DispatchersTest {

    @Test
    fun `API scope runs off the main thread`() {
        val mainThread = Thread.currentThread()
        var threadCanary = false
        Dispatchers.API.setTestingMode(false)
        Dispatchers.API.setTaskQueueing(false)

        runBlocking {
            Dispatchers.API.launch {
                assertNotSame(mainThread, Thread.currentThread())
                // Use the canary bool to make sure this is getting called before
                // the test completes.
                assertEquals(false, threadCanary)
                threadCanary = true
            }!!.join()
        }

        Dispatchers.API.setTestingMode(true)
        assertEquals(true, threadCanary)
        assertSame(mainThread, Thread.currentThread())
    }

    @Test
    fun `launch correctly adds tests to queue if queueTasks is true`() {
        var threadCanary = 0

        Dispatchers.API.setTestingMode(true)
        Dispatchers.API.setTaskQueueing(true)

        // Add 3 tasks to queue each one setting threadCanary to true to indicate if any task has ran
        repeat(3) {
            Dispatchers.API.launch {
                threadCanary += 1
            }
        }

        assertEquals("Task queue contains the correct number of tasks",
            3, Dispatchers.API.taskQueue.size)
        assertEquals("Tasks have not run while in queue", 0, threadCanary)

        // Now trigger execution to ensure the tasks fired
        Dispatchers.API.flushQueuedInitialTasks()

        assertEquals("Tasks have executed", 3, threadCanary)
        assertEquals("Task queue is cleared", 0, Dispatchers.API.taskQueue.size)
    }

    @Test
    fun `queued tasks are executed in the order they are received`() {
        val orderedList = mutableListOf<Int>()
        val jobs = mutableListOf<Job>()

        Dispatchers.API.setTestingMode(false)
        Dispatchers.API.setTaskQueueing(true)

        // This coroutine will monitor the taskQueue.count() to toggle the flushing of the queued
        // items when the queue is half full (50 elements).  This should give us 50 items in the
        // queue and then 50 items that are launched after the queue is flushed.
        jobs.add(GlobalScope.launch {
            while (Dispatchers.API.taskQueue.count() < 50) { Thread.yield() }
            Dispatchers.API.flushQueuedInitialTasks()
        })

        // This coroutine will add elements to the orderedList.  This will continue to
        // add elements to the queue until there are at least 50 elements in the queue. At that
        // point, the coroutine above will flush and disable the queuing and this coroutine will
        // continue launching tasks directly.
        jobs.add(GlobalScope.launch {
            (0..99).forEach { num ->
                Dispatchers.API.launch {
                    orderedList.add(num)
                }?.let {
                    jobs.add(it)
                }
            }
        })

        // Wait for the numbers to be added to the list by waiting for the tasks to join.
        runBlocking {
            jobs.joinAll()

            // This delay seems to be necessary to allow all of the coroutines time to finish.  This
            // is unexpected as I would think that the `joinAll()` above should accomplish this.
            delay(10)
        }

        // Ensure elements match in the correct order
        (0..99).forEach { num ->
            assertTrue(
                "Index [$num] is less than size of list [${orderedList.size}]",
                num < orderedList.size
            )
            assertEquals(num, orderedList[num])
        }
    }
}
