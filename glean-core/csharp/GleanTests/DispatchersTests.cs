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
        public void TestTaskQueueing()
        {
            var threadCanary = 0;

            Dispatchers.TestingMode = true;
            Dispatchers.QueueInitialTasks = true;

            List<Task> tasks = new List<Task>();

            // Add 3 tasks to the queue, each one incrementing threadCanary to
            // indicate the task has executed.
            for (int i = 0; i < 3; ++i)
            {
                tasks.Add(Dispatchers.LaunchAPI(() =>
                {
                    threadCanary += 1;
                }));
            }

            Assert.Equal(3, Dispatchers.preInitActionQueue.Count);
            Assert.Equal(0, threadCanary);

            Dispatchers.FlushQueuedInitialTasks();

            Assert.Equal(3, threadCanary);
            Assert.Empty(Dispatchers.preInitActionQueue);
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
                    Interlocked.Add(ref threadCanary, 1);
                });
            }

            Assert.Equal(3, Dispatchers.preInitActionQueue.Count);
            Assert.Equal(0, Interlocked.Read(ref threadCanary));

            // Trigger execution to ensure tasks have fired
            Dispatchers.FlushQueuedInitialTasks();

            Assert.Equal(3, Interlocked.Read(ref threadCanary));
            Assert.Empty(Dispatchers.preInitActionQueue);
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
                while (!flushTasks) { Thread.Yield(); }
                Dispatchers.FlushQueuedInitialTasks();
            });

            List<Task> tasks = new List<Task>();
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
                    var t = Dispatchers.LaunchAPI(() =>
                    {
                        orderedList.Add(value);
                        counter += 1;
                    });

                    if (t != null) { tasks.Add(t); }
                }
            });

            task1.Wait();
            task2.Wait(5000);

            Task.WaitAll(tasks.ToArray());

            var counterTimeout = 0;
            while (counter < 99)
            {
                Thread.Sleep(1);

                Assert.True(++counterTimeout < 5000);
            }

            for (int num = 0; num < 99; num++)
            {
                Assert.Equal(num, orderedList[num]);
            }
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

            Task.WaitAll(tasks.ToArray());

            Assert.Equal(3, threadCanary);
        }

        [Fact]
        public void QueuedTasksExceptionsAreCaughtWhenFlushing()
        {
            var mainThread = Thread.CurrentThread;
            long threadCanary = 0;

            Dispatchers.TestingMode = false;
            Dispatchers.QueueInitialTasks = true;

            Assert.Empty(Dispatchers.preInitActionQueue);

            // Dispatch an initial task that throws an exception. Even if this
            // throws, the dispatcher should catch and keep on executing other
            // tasks while flushing.
            Dispatchers.LaunchAPI(() =>
            {
                throw new Exception("Test exception for DispatchersTest");
            });

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

            Assert.Equal(4, Dispatchers.preInitActionQueue.Count);
            Assert.Equal(0, threadCanary);

            // Trigger execution to ensure tasks have fired
            Dispatchers.FlushQueuedInitialTasks();

            Assert.Equal(3, threadCanary);
            Assert.Empty(Dispatchers.preInitActionQueue);
        }
    }
}
