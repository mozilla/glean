// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.Testing;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text.Json;
using System.Threading;
using Xunit;
using static Mozilla.Glean.Glean;

namespace Mozilla.Glean.Tests.Metrics
{
    enum clickKeys : int
    {
        objectId,
        other
    }

    enum testNameKeys : int
    {
        testName
    }

    enum SomeExtraKeys : int
    {
        SomeExtra
    }

    public class EventMetricTypeTest
    {
        private readonly MockUploader mockUploader = new MockUploader();

        // Define a convenience function to manually reset Glean.
        private void ResetGlean(string dataDir)
        {
            GleanInstance.TestDestroyGleanHandle();

            GleanInstance.Reset(
                applicationId: "org.mozilla.csharp.tests",
                applicationVersion: "1.0-test",
                uploadEnabled: true,
                configuration: new Configuration(httpClient: mockUploader),
                dataDir: dataDir
                );
        }

        public EventMetricTypeTest()
        {
            // Get a random test directory just for this single test.
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // Remove all the pending pings from the queue.
            mockUploader.Clear();

            ResetGlean(tempDataDir);
        }

        [Fact]
        public void APISavesToStorage()
        {
            Private.EventMetricType<clickKeys> click = new Private.EventMetricType<clickKeys>(
                category: "ui",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "click",
                sendInPings: new string[] { "store1" },
                allowedExtraKeys: new string[] { "object_id", "other" }
            );

            // Record two events of the same type, with a little delay.
            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonA" },
                { clickKeys.other, "foo" }
            });

            Thread.Sleep(37);

            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonB" },
                { clickKeys.other, "bar"  }
            });

            // Check that data was properly recorded.
            var snapshot = click.TestGetValue();
            Assert.True(click.TestHasValue());
            Assert.Equal(2, snapshot.Length);

            var firstEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonA");
            Assert.Equal("ui", firstEvent.Category);
            Assert.Equal("click", firstEvent.Name);
            Assert.Equal("foo", firstEvent.Extra["other"]);

            var secondEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonB");
            Assert.Equal("ui", secondEvent.Category);
            Assert.Equal("click", secondEvent.Name);
            Assert.Equal("bar", secondEvent.Extra["other"]);

            Assert.True(firstEvent.Timestamp < secondEvent.Timestamp, "The sequence of the events must be preserved");
        }

        [Fact]
        public void TheAPIRecordsToItsStorageEngineWhenCategoryIsEmpty()
        {
            Private.EventMetricType<clickKeys> click = new Private.EventMetricType<clickKeys>(
                category: "",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "click",
                sendInPings: new string[] { "store1" },
                allowedExtraKeys: new string[] { "object_id" }
            );

            // Record two events of the same type, with a little delay.
            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonA" }
            });

            Thread.Sleep(37);

            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonB" }
            });

            // Check that data was properly recorded.
            var snapshot = click.TestGetValue();
            Assert.True(click.TestHasValue());
            Assert.Equal(2, snapshot.Length);

            var firstEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonA");
            Assert.Equal("click", firstEvent.Name);

            var secondEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonB");
            Assert.Equal("click", secondEvent.Name);

            Assert.True(firstEvent.Timestamp < secondEvent.Timestamp, "The sequence of the events must be preserved");
        }

        [Fact]
        public void DisabledEventsMustNotRecordData()
        {
            Private.EventMetricType<clickKeys> click = new Private.EventMetricType<clickKeys>(
                category: "ui",
                disabled: true,
                lifetime: Private.Lifetime.Ping,
                name: "click",
                sendInPings: new string[] { "store1" }
            );

            // Attempt to store the event.
            click.Record();

            // Check that nothing was recorded.
            Assert.False(click.TestHasValue(), "Events must not be recorded if they are disabled");
        }

        [Fact]
        public void TestGetValueThrows()
        {
            Private.EventMetricType<clickKeys> testEvent = new Private.EventMetricType<clickKeys>(
                category: "ui",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "testEvent",
                sendInPings: new string[] { "store1" }
            );
            Assert.Throws<NullReferenceException>(() => testEvent.TestGetValue());
        }

        [Fact]
        public void TheAPIRecordsToSecondaryPings()
        {
            Private.EventMetricType<clickKeys> click = new Private.EventMetricType<clickKeys>(
                category: "ui",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "click",
                sendInPings: new string[] { "store1", "store2" },
                allowedExtraKeys: new string[] { "object_id" }
            );

            // Record two events of the same type, with a little delay.
            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonA" }
            });

            Thread.Sleep(37);


            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, "buttonB" }
            });

            // Check that data was properly recorded in the second ping.
            var snapshot = click.TestGetValue("store2");
            Assert.True(click.TestHasValue("store2"));
            Assert.Equal(2, snapshot.Length);

            var firstEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonA");
            Assert.Equal("ui", firstEvent.Category);
            Assert.Equal("click", firstEvent.Name);

            var secondEvent = snapshot.First(e => (e.Extra != null) && e.Extra["object_id"] == "buttonB");
            Assert.Equal("ui", secondEvent.Category);
            Assert.Equal("click", secondEvent.Name);

            Assert.True(firstEvent.Timestamp < secondEvent.Timestamp, "The sequence of the events must be preserved");
        }

        [Fact]
        public void EventsShouldNotRecordWhenUploadIsDisabled()
        {
            Private.EventMetricType<testNameKeys> eventMetric = new Private.EventMetricType<testNameKeys>(
                category: "ui",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "event_metric",
                sendInPings: new string[] { "store1" },
                allowedExtraKeys: new string[] { "test_name" }
            );

            GleanInstance.SetUploadEnabled(true);
            eventMetric.Record(extra: new Dictionary<testNameKeys, string> {
                { testNameKeys.testName, "event1" }
            });
            var snapshot1 = eventMetric.TestGetValue();
            Assert.Single(snapshot1);
            GleanInstance.SetUploadEnabled(false);
            eventMetric.Record(extra: new Dictionary<testNameKeys, string> {
                { testNameKeys.testName, "event2" }
            });

            try {
                eventMetric.TestGetValue();
                Assert.True(false, "Expected events to be empty");
            } catch (NullReferenceException) {
            }
            GleanInstance.SetUploadEnabled(true);
            eventMetric.Record(extra: new Dictionary<testNameKeys, string> {
                { testNameKeys.testName, "event3" }
            });
            var snapshot3 = eventMetric.TestGetValue();
            Assert.Single(snapshot3);
        }

        [Fact]
        public void FlushQueuedEventsOnStartup()
        {
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // Re-initialize, we need to point this to the data directory we know of.
            ResetGlean(tempDataDir);

            Private.EventMetricType<SomeExtraKeys> eventMetric = new Private.EventMetricType<SomeExtraKeys>(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "test_event",
                sendInPings: new string[] { "events" },
                allowedExtraKeys: new string[] { "someExtra" }
            );

            eventMetric.Record(extra: new Dictionary<SomeExtraKeys, string> {
                { SomeExtraKeys.SomeExtra, "bar" }
            });
            Assert.Single(eventMetric.TestGetValue());

            // Start a new Glean instance to trigger the sending of "stale" events
            ResetGlean(tempDataDir);

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("events", request.docType);
            Assert.Contains("/submit/org-mozilla-csharp-tests/events/", request.url);

            // Check the content of the events ping.
            JsonDocument data = JsonDocument.Parse(request.payload);
            JsonElement root = data.RootElement;

            // TODO: Check the ping schema.
            // checkPingSchema(data);

            JsonElement eventsProperty;
            Assert.True(root.TryGetProperty("events", out eventsProperty));
            Assert.Equal(1, eventsProperty.GetArrayLength());
            Assert.Equal("startup", root.GetProperty("ping_info").GetProperty("reason").GetString());
        }

        [Fact]
        public void FlushQueuedEventsOnStartupAndCorrectlyHandlePreInitEvents()
        {
            string tempDataDir = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());

            // Re-initialize, we need to point this to the data directory we know of.
            ResetGlean(tempDataDir);

            Private.EventMetricType<SomeExtraKeys> eventMetric = new Private.EventMetricType<SomeExtraKeys>(
                category: "telemetry",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "test_event",
                sendInPings: new string[] { "events" },
                allowedExtraKeys: new string[] { "someExtra" }
            );

            eventMetric.Record(extra: new Dictionary<SomeExtraKeys, string> {
                { SomeExtraKeys.SomeExtra, "run1" }
            });
            Assert.Single(eventMetric.TestGetValue());

            Dispatchers.QueueInitialTasks = true;
            eventMetric.Record(extra: new Dictionary<SomeExtraKeys, string> {
                { SomeExtraKeys.SomeExtra, "pre-init" }
            });

            ResetGlean(tempDataDir);

            eventMetric.Record(extra: new Dictionary<SomeExtraKeys, string> {
                { SomeExtraKeys.SomeExtra, "post-init" }
            });

            MockUploader.UploadRequest request = mockUploader.GetPendingUpload();
            Assert.Equal("events", request.docType);

            // Check the content of the events ping.
            JsonDocument data = JsonDocument.Parse(request.payload);
            JsonElement root = data.RootElement;

            // This event comes from disk from the prior "run"
            Assert.Equal("startup", root.GetProperty("ping_info").GetProperty("reason").GetString());

            JsonElement eventsProperty;
            Assert.True(root.TryGetProperty("events", out eventsProperty));
            Assert.Equal(1, eventsProperty.GetArrayLength());
            Assert.Equal("run1",
                eventsProperty.EnumerateArray().ElementAt(0).GetProperty("extra").GetProperty("someExtra").GetString()
            );

            GleanInstance.SubmitPingByName("events", "background");
            
            request = mockUploader.GetPendingUpload();
            Assert.Equal("events", request.docType);
            data = JsonDocument.Parse(request.payload);
            root = data.RootElement;

            // This event comes from the pre-initialization event
            Assert.Equal("background", root.GetProperty("ping_info").GetProperty("reason").GetString());
            Assert.True(root.TryGetProperty("events", out eventsProperty));
            Assert.Equal(2, eventsProperty.GetArrayLength());
            Assert.Equal("pre-init",
                eventsProperty.EnumerateArray().ElementAt(0).GetProperty("extra").GetProperty("someExtra").GetString()
            );
            Assert.Equal("post-init",
                eventsProperty.EnumerateArray().ElementAt(1).GetProperty("extra").GetProperty("someExtra").GetString()
            );
        }

        [Fact]
        public void LongExtraValuesRecordAnError()
        {
            Private.EventMetricType<clickKeys> click = new Private.EventMetricType<clickKeys>(
                category: "ui",
                disabled: false,
                lifetime: Private.Lifetime.Ping,
                name: "click",
                sendInPings: new string[] { "store1" },
                allowedExtraKeys: new string[] { "object_id", "other" }
            );

            string longString = new string('a', 110);

            click.Record(extra: new Dictionary<clickKeys, string> {
                { clickKeys.objectId, longString }
            });

            Assert.Equal(1, click.TestGetNumRecordedErrors(ErrorType.InvalidOverflow));
        }
    }
}
