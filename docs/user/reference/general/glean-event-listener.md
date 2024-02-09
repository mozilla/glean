# Glean Event Listener

> **Note:** This API is currently experimental and subject to change or elimination. Please reach out to the Glean Team if you are planning on using this API in its experimental state.

## Summary

Glean provides an API to register a callback by which a consumer can be notified of all event metrics being recorded.

## Usage

Consumers can register a callback through this API which will be called with the base identifier of each event metric when it is recorded. The base identifier of the event consists of the category name and the event name with a dot separator:  

`<event_category>.<event_name>`

Glean will execute the registered callbacks on a background thread independent of the thread in which the event is being recorded in order to not interfere with the collection of the event.

Glean will ensure that event recordings are reported to listeners in the same order that they are recorded by using the same dispatching mechanisms used to ensure events are recorded in the order they are received.  

## Examples

<div data-lang="Kotlin" class="tab">

```kotlin
class TestEventListener : GleanEventListener {
    val listenerTag: String = "TestEventListener"
    var lastSeenId: String = ""
    var count: Int = 0

    override fun onEventRecorded(id: String) {
        this.lastSeenId = id
        this.count += 1
    }
}

val listener = TestEventListener()
Glean.registerEventListener(listener.listenerTag, listener)

// If necessary to unregister the listener:
Glean.unregisterEventListener(listener.listenerTag)
```

</div>

<div data-lang="Swift" class="tab">

```Swift
class TestEventListener: GleanEventListener {
    let listenerTag: String = "TestEventListener"
    var lastSeenId: String = ""
    var count: Int64 = 0

    func onEventRecorded(_ id: String) {
        self.lastSeenId = id
        self.count += 1
    }
}

let listener = TestEventListener()
Glean.shared.registerEventListener(tag: listener.listenerTag, listener: listener)

// If necessary to unregister the listener:
Glean.shared.unregisterEventListener(listener.listenerTag)
```

</div>

{{#include ../../../shared/tab_footer.md}}
