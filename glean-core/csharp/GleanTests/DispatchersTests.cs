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
        //public DispatchersTests()
        //{
        //    Dispatchers.CancelBackgroundTasks();
        //}

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
    }
}
