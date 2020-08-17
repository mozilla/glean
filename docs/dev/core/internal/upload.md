# Upload mechanism

The `glean-core` Rust crate does not handle the ping upload directly.
Network stacks vastly differ between platforms, applications and operating systems.
The Glean SDK leverages the available platform capabilities to implement any network communication.

Glean core controls all upload and coordinates the platform side with its own internals.
All language bindings implement ping uploading around a common API and protocol.

## The upload module in the language bindings

```mermaid
classDiagram
    class UploadResult {
        ~ToFFI()* int
    }

    class HttpResponse {
        int statusCode
        ~ToFFI() int
    }

    class UnrecoverableFailure {
        ~ToFFI() int
    }

    class RecoverableFailure {
        ~ToFFI() int
    }

    class PingUploader {
        <<interface>>
        +Upload() UploadResult
    }

    class BaseUploader {
        +BaseUploader(PingUploader)
        ~TriggerUploads()
        ~CancelUploads()
    }

    class HttpUploader {
        +Upload() UploadResult
    }

    UploadResult <|-- HttpResponse
    UploadResult <|-- UnrecoverableFailure
    UploadResult <|-- RecoverableFailure
    PingUploader <|-- HttpUploader
    PingUploader o-- BaseUploader

```

### `PingUploader`
The `PingUploader` interface describes the contract between the `BaseUploader` and the SDK or user-provided upload modules.

### `BaseUploader`
The `BaseUploader` component is responsible for interfacing with the lower level `get_upload_task` calls and dealing with the logic in a platform-coherent way.

  * Every Glean instance will always have a single `BaseUploader` instance.
  * The `BaseUploader` is fed, at Glean initialization, with an instance of an implementation of the `PingUploader` interface.
  * Whenever `BaseUploader` thinks it should perform an upload, it will call the provided instance of the `PingUploader` interface and call `upload` with the data it's getting from the glean-core/FFI.
  * Any throttling happens at this layer: the core will orchestrate the throttling, while this layer will be responsible to abide to what the core is telling it to do.
  * Any logic for aborting uploads or triggering uploads is provided by this object.

### `HttpClientUploader`
The `HttpClientUploader` is the default SDK-provided HTTP uploader. It acts as an adapter between the platform-specific upload library and the Glean upload APIs.

> Note that most of the languages have now diverged, due to the many iterations, from this design. For example, in Kotlin, the [`BaseUploader` is mostly empty](https://searchfox.org/glean/source/glean-core/android/src/main/java/mozilla/telemetry/glean/net/BaseUploader.kt) and its functionalities are spread in the [`PingUploadWorker`](https://searchfox.org/glean/source/glean-core/android/src/main/java/mozilla/telemetry/glean/scheduler/PingUploadWorker.kt).

## Upload task API

The following diagram visualizes the communication between Glean core (the Rust crate),
a Glean language binding (e.g. the Kotlin or Swift implementation) and a Glean end point server.

```mermaid
sequenceDiagram
    participant Glean core
    participant Glean wrapper
    participant Server

    Glean wrapper->>Glean core: get_upload_task()
    Glean core->>Glean wrapper: Task::Upload(PingRequest)
    Glean wrapper-->>Server: POST /submit/{task.id}
    Server-->>Glean wrapper: 200 OK
    Glean wrapper->>Glean core: upload_response(200)
    Glean wrapper->>Glean core: get_upload_task()
    Glean core->>Glean wrapper: Task::Done
```

Glean core will take care of file management, cleanup, rescheduling and rate limiting[^1].

[^1]: Rate limiting is achieved by limiting the amount of times a language binding is allowed to get a `Task::Upload(PingRequest)` from `get_upload_task` in a given time interval. Currently, the default limit is for a maximum of 15 upload tasks every 60 seconds and there are no exposed methods that allow changing this default (follow [Bug 1647630](https://bugzilla.mozilla.org/show_bug.cgi?id=1647630) for updates). If the caller has reached the maximum tasks for the current interval, they will get a `Task::Wait` regardless if there are other `Task::Upload(PingRequest)`s queued.

## Available APIs

{{#include ../../../tab_header.md}}

<div data-lang="Rust" class="tab">

For direct Rust consumers the global `Glean` object provides these methods:

```rust
/// Gets the next task for an uploader.
fn get_upload_task(&self) -> PingUploadTask

/// Processes the response from an attempt to upload a ping.
fn process_ping_upload_response(&self, uuid: &str, status: UploadResult)
```

See the documentation for further usage and explanation of the additional types:

* [`get_upload_task`](../../../../docs/glean_core/struct.Glean.html#method.get_upload_task)
* [`process_ping_upload_response`](../../../../docs/glean_core/struct.Glean.html#method.process_ping_upload_response)
* [`PingUploadTask`](../../../../docs/glean_core/upload/enum.PingUploadTask.html)
* [`UploadResult`](../../../../docs/glean_core/upload/enum.UploadResult.html)

</div>

<div data-lang="FFI" class="tab">

For FFI consumers (e.g. Kotlin/Swift/Python implementations) these functions are available:

```rust
/// Gets the next task for an uploader. Which can be either:
extern "C" fn glean_get_upload_task(result: *mut FfiPingUploadTask)

/// Processes the response from an attempt to upload a ping.
extern "C" fn glean_process_ping_upload_response(task: *mut FfiPingUploadTask, status: u32)
```

See the documentation for additional information about the types:

* [`glean_ffi::upload`](../../../../docs/glean_ffi/upload/index.html)

</div>

{{#include ../../../tab_footer.md}}
