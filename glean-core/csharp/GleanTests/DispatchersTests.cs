// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Xunit;

namespace Mozilla.Glean.Tests
{
    public class DispatchersTests
    {
        [Fact]
        public void LaunchAPIDoesNotRunOnMainThread()
        {
            var mainThread = Thread.CurrentThread;
            var threadCanary = false;

            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = false;

            var task = Dispatchers.LaunchAPI(() =>
            {
                Assert.NotSame(mainThread, Thread.CurrentThread);
                threadCanary = true;
            });

            // Wait for the task to complete
            task.Wait();

            Dispatchers.TestingMode = true;
            Assert.True(threadCanary);
            Assert.Same(mainThread, Thread.CurrentThread);
        }

        [Fact]
        public void LaunchAConcurrentDoesNotRunOnMainThread()
        {
            var mainThread = Thread.CurrentThread;
            var threadCanary = false;

            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = false;

            var task = Dispatchers.LaunchConcurrent(() =>
            {
                Assert.NotSame(mainThread, Thread.CurrentThread);
                threadCanary = true;
            });

            // Wait for the task to complete
            task.Wait();

            Dispatchers.TestingMode = true;
            Assert.True(threadCanary);
            Assert.Same(mainThread, Thread.CurrentThread);
        }

        [Fact]
        public void TestTaskQueueing()
        {
            var threadCanary = 0;

            Dispatchers.TestingMode = true;
            Dispatchers.QueueInitialTasks = true;

            // Add 3 tasks to the queue, each one incrementing threadCanary to
            // indicate the task has executed.
            for (int i = 0; i < 3; ++i)
            {
                Dispatchers.LaunchAPI(() =>
                {
                    threadCanary += 1;
                });
            }

            Assert.Equal(3, Dispatchers.taskQueue.Count);
            Assert.Equal(0, threadCanary);

            Dispatchers.FlushQueuedInitialTasks();

            Assert.Equal(3, threadCanary);
            Assert.Empty(Dispatchers.taskQueue);
        }

        [Fact]
        public void QueuedTasksAreFlushedOffTheMainThread()
        {
            var mainThread = Thread.CurrentThread;
            long threadCanary = 0;

            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = true;

            // Add 3 tasks to queue each one setting threadCanary to true
            // to indicate if any task has ran.
            for (int i = 0; i < 3; i++)
            {
                Dispatchers.LaunchAPI(() =>
                {
                    Assert.NotSame(mainThread, Thread.CurrentThread);
                    threadCanary += 1;
                });
            }

            Assert.Equal(3, Dispatchers.taskQueue.Count);
            Assert.Equal(0, Interlocked.Read(ref threadCanary));

            // Trigger execution to ensure tasks have fired
            Dispatchers.FlushQueuedInitialTasks();

            Assert.Equal(3, threadCanary);
            Assert.Empty(Dispatchers.taskQueue);
        }

        [Fact]
        public void TestQueuedTasksAreExecutedInOrderAsync()
        {
            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = true;

            var flushTasks = false;
            List<int> orderedList = new List<int>();

            var task1 = Task.Factory.StartNew(() =>
            {
                while (!flushTasks) { Thread.Sleep(1); }
                Dispatchers.FlushQueuedInitialTasks();
            });

            var counter = 0;
            var task2 = Task.Factory.StartNew(() =>
            {
                for (int num = 0; num < 99; num++)
                {
                    if (num == 50)
                    {
                        flushTasks = true;
                    }

                    // Need to "capture" the value here with a local variable.
                    // Just using `num` was insufficient as it appeared to be
                    // not getting captured by the lambda below.
                    var value = num;
                    Dispatchers.LaunchAPI(() =>
                    {
                        orderedList.Add(value);
                        counter += 1;
                    });
                }
            });

            task1.Wait();
            task2.Wait();

            while (counter < 99) { Thread.Sleep(1); }

            for (int num = 0; num < 99; num++)
            {
                Assert.Equal(num, orderedList[num]);
            }
        }

        [Fact]
        public void TestCancellingBackgroundTasksClearsQueue()
        {
            // Set testing mode to false to allow for background execution.
            Dispatchers.TestingMode = false;

            // Set task queuing to true to ensure that we queue tasks when we
            // launch them.
            Dispatchers.QueueInitialTasks = true;

            // Assert the queue is empty
            Assert.Empty(Dispatchers.taskQueue);

            // Add a task to the pre-init queue
            Dispatchers.LaunchAPI(() =>
            {
                Console.WriteLine("A queued task");
            });

            // Assert the task was queued
            Assert.NotEmpty(Dispatchers.taskQueue);

            Dispatchers.CancelBackgroundTasks();

            // Assert the queue is empty
            Assert.Empty(Dispatchers.taskQueue);
        }

        [Fact]
        public void DispatchedTasksThrowingExceptionsAreCorrectlyHandled()
        {
            List<Task> tasks = new List<Task>();
            var mainThread = Thread.CurrentThread;
            long threadCanary = 0;

            // Ensure tasks are executed asynchronously
            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = false;

            // Dispatch an initial task that throws an exception
            tasks.Add(Dispatchers.LaunchAPI(() =>
            {
                Assert.NotSame(mainThread, Thread.CurrentThread);
                throw new Exception("Test exception for DispatchersTest");
            }));

            // Add 3 tasks to queue each one setting threadCanary to true
            // to indicate if any task has ran.
            for (int i = 0; i < 3; i++)
            {
                tasks.Add(Dispatchers.LaunchAPI(() =>
                {
                    Assert.NotSame(mainThread, Thread.CurrentThread);
                    threadCanary += 1;
                }));
            }

            // Wait for the tasks to complete
            tasks.ForEach(task => task.Wait());

            Assert.Equal(3, threadCanary);
        }
    }
}
