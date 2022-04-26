/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

package mozilla.telemetry.glean

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.Dispatchers as KotlinDispatchers
import kotlinx.coroutines.Job
import kotlinx.coroutines.delay
import kotlinx.coroutines.isActive
import kotlinx.coroutines.joinAll
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.withTimeoutOrNull
import org.junit.Test
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotSame
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import java.util.concurrent.atomic.AtomicInteger

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
    fun `launch correctly adds tasks to queue if queueTasks is true`() {
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
        runBlocking {
            Dispatchers.API.flushQueuedInitialTasks()
        }

        assertEquals("Tasks have executed", 3, threadCanary)
        assertEquals("Task queue is cleared", 0, Dispatchers.API.taskQueue.size)
    }

    @Test
    fun `queued tasks are flushed off the main thread`() {
        val mainThread = Thread.currentThread()
        val threadCanary = AtomicInteger()

        // By setting testing mode to false, we make sure that things
        // are executed asynchronously.
        Dispatchers.API.setTestingMode(false)
        Dispatchers.API.setTaskQueueing(true)

        // Add 3 tasks to queue each one setting threadCanary to true
        // to indicate if any task has ran.
        repeat(3) {
            Dispatchers.API.launch {
                assertNotSame(
                    "Pre-init tasks must be flushed off the main thread",
                    mainThread,
                    Thread.currentThread()
                )
                threadCanary.incrementAndGet()
            }
        }

        assertEquals("Task queue contains the correct number of tasks",
            3, Dispatchers.API.taskQueue.size)
        assertEquals("Tasks have not run while in queue", 0, threadCanary.get())

        // Now trigger execution to ensure the tasks fired
        GlobalScope.launch {
            Dispatchers.API.flushQueuedInitialTasks()
        }

        // Wait for the flushed tasks to be executed.
        runBlocking {
            withTimeoutOrNull(2000) {
                while (isActive && (threadCanary.get() != 3 || Dispatchers.API.taskQueue.size > 0)) {
                    delay(1)
                }
            } ?: assertTrue("Timed out waiting for tasks to execute", false)
        }

        assertEquals("All tasks ran to completion", 3, threadCanary.get())
        assertEquals("Task queue is cleared", 0, Dispatchers.API.taskQueue.size)
    }

    @Test
    fun `queued tasks are executed in the order they are received`() {
        val orderedList = mutableListOf<Int>()
        val jobs = mutableListOf<Job>()

        Dispatchers.API.setTestingMode(false)
        Dispatchers.API.setTaskQueueing(true)

        val coroutineScope = CoroutineScope(KotlinDispatchers.Default)

        // This coroutine will monitor the taskQueue.count() to toggle the flushing of the queued
        // items when the queue is half full (50 elements).  This should give us 50 items in the
        // queue and then 50 items that are launched after the queue is flushed.
        val flushJob = coroutineScope.launch {
            while (Dispatchers.API.taskQueue.count() < 50) { Thread.yield() }
            Dispatchers.API.flushQueuedInitialTasks()
        }

        // This coroutine will add elements to the orderedList.  This will continue to
        // add elements to the queue until there are at least 50 elements in the queue. At that
        // point, the coroutine above will flush and disable the queuing and this coroutine will
        // continue launching tasks directly.
        // Note: we need a counter to make sure that all the jobs are added to the `jobs` array.
        // we can't simply check the array size as it might change while we're checking.
        val counter = AtomicInteger()
        val listJob = coroutineScope.launch(KotlinDispatchers.Default) {
            (0..99).forEach { num ->
                Dispatchers.API.launch {
                    orderedList.add(num)
                    counter.incrementAndGet()
                }?.let {
                    jobs.add(it)
                }
            }
        }

        // Wait for the numbers to be added to the list by waiting for the tasks to join.
        runBlocking {
            // Ensure that all the required jobs have been added to the list.
            withTimeoutOrNull(2000) {
                while (isActive && counter.get() < 100) {
                    delay(1)
                }
            } ?: assertEquals("Timed out waiting for tasks to execute", 100, counter.get())

            // Wait for them to execute.
            flushJob.join()
            listJob.join()
            jobs.joinAll()
        }

        // Ensure elements match in the correct order
        (0..99).forEach { num ->
            assertTrue(
                "Index [$num] is less than size of list [${orderedList.size}]",
                num < orderedList.size
            )
            assertEquals("This list is out of order $orderedList", num, orderedList[num])
        }
    }

    @Test
    fun `dispatched tasks throwing exceptions are correctly handled`() {
        val mainThread = Thread.currentThread()
        val threadCanary = AtomicInteger()

        // By setting testing mode to false, we make sure that things
        // are executed asynchronously.
        Dispatchers.API.setTestingMode(false)
        Dispatchers.API.setTaskQueueing(false)

        // Dispatch an initial tasks that throws an exception.
        Dispatchers.API.launch {
            assertNotSame(
                "Tasks must be executed off the main thread",
                mainThread,
                Thread.currentThread()
            )
            @Suppress("TooGenericExceptionThrown")
            throw Exception("Test exception for DispatchersTest")
        }

        // Add 3 tasks to queue each one increments threadCanary
        // to indicate if any task has ran.
        repeat(3) {
            Dispatchers.API.launch {
                assertNotSame(
                    "Tasks must be executed off the main thread",
                    mainThread,
                    Thread.currentThread()
                )
                threadCanary.incrementAndGet()
            }
        }

        // Wait for the flushed tasks to be executed.
        runBlocking {
            withTimeoutOrNull(2000) {
                while (isActive && (threadCanary.get() != 3)) {
                    delay(1)
                }
            } ?: assertTrue("Timed out waiting for tasks to execute", false)
        }

        assertEquals("All the dispatched actions should execute", 3, threadCanary.get())
    }
}
