# Architecture

This document describes the architecture of the Glean SDK, covering the `glean-core` crate and its interaction with a foreign-language SDK.
Some of the listed things are not yet implemented as is, but desired.

## UniFFI definition

The API available to foreign-language SDKs is defined in `glean-core/src/glean.udl`.


### `namespace glean`

The top-level `glean` namespace describes the API that is used by foreign-language SDKs to configure & initialize the Glean object and change its state while running.
To avoid name clashes all functions are prefixed with `glean_`.
Some functions MUST NOT be used outside of test mode. These have a `glean_test_` prefix.

Note: This DOES NOT correspond to the public [Glean General API](https://mozilla.github.io/glean/book/reference/general/index.html).
The foreign language SDK needs to implement the public API by deferring to `glean_*` functions.

All methods should include appropriate documentation.

### Metric types

Metric types are defined as a UniFFI `interface`.


```udl
interface CounterMetric {
	...
};
```

This should describe the API exposed to users directly.  
_Implementation note:_
When migrating a data type try to keep the API stable, e.g. use the same integer types.
Breaking changes (e.g. changing the integer type) can be changed in a followup.

(All example code below uses the `Counter` metric)

#### Constructor

Every metric type needs a constructor, taking a `CommonMetricData` object and any additional configuration.

```udl
constructor(CommonMetricData meta);
```

### Recording methods

Every metric type can export one or more recording methods.

```udl
void add(optional i32 amount = 1);
```

### Test methods

Every metric should export a `test_get_value` method.

```udl
i32? test_get_value(optional string? ping_name = null);
```

Additional methods can be exposed as well, e.g. to get errors.  
_TODO: Add example to the counter metric._

## Core implementation (Rust)

### `namespace glean`

Everything exposed in the top-level `glean` namespace is implemented in `glean-core/src/lib.rs` (or publicly exported there).
Most methods there probably need to dispatch their task to be sure they run after Glean has been initialized.
If that's not the case they should get proper documentation why it's not necessary.
Test methods can use `block_on_dispatcher` and run synchronously.

This part holds additional state (`global_state()`), such as the client info and callbacks.

_Note: Always lock the `Glean` object first, then acquire a handle to the global state, to avoid lock-order-inversion._

_Implementation note:_ We might eventually merge that back into the `Glean` object to reduce the parts where we hold state.

#### Core

The actual implementation of the `Glean` object is in `glean-core/src/core/mod.rs`.
This object holds all state of Glean, such as the database handle, upload state, etc.
Methods on the Glean object MUST run synchronously and not use the dispatcher.

_Note: This code is (nearly) unmodified from the old `glean-core` implementation._

The core object defers to other objects for further work.
Other objects, such as the database, ping maker, MUST run synchronously and not use the dispatcher.

_Note: These other objects are also the unmodified code from the old `glean-core` implementation._


#### Dispatcher

The dispatcher is based on the initial RLB implementation.
Tasks will run on a separate thread.
The dispatcher adds a test mode to run tasks asynchronously.
This MUST NOT be used outside of foreign-language SDK tests.

#### Metric types

Metric type implementations live in `glean-core/src/metrics`.
They should be based on the initial `glean-core` implementations,
but modified to expose the API defined for the RLB (see `glean-core/rlb/src/private`).
Recording methods should wrap everything into tasks launched on the dispatcher.
Actual recording should be a separate synchronous method with a `_sync` suffix.
That will make testing easier.

_TODO: Sync methods should only be crate-public, but that requires us moving tests._

Test methods should block on the dispatcher, then run synchronously to return data.

### Tests

Currently metric type tests are implemented as integration tests in `glean-core/tests`.
It's best to test the synchronous API of the metric,
as that way they can run in parallel with their own instance of a `Glean` object.
However that requires for synchronous methods to be exposed publicly.

Eventually we might migrate these tests back into unit tests within the crate, so we can turn synchronous methods to crate-public only.

## Foreign-language SDK: Kotlin

The Glean Kotlin SDK lives in `glean-core/android`.

### General API

The general API is implemented in `Glean.kt`.
It imports the UniFFI-created module `mozilla.telemetry.glean.internal.*` and essentially wraps the exposed functions to provide the user-facing general API.
It holds a bit of state, including

* the HTTP uploader
* the metrics ping scheduler
* the lifecycle observer
* an `initialized` flag

It can directly call `glean-core` exposed methods and MUST NOT use a dispatcher.
It can avoid calling `glean-core` methods if its not `initialized`.

The most complex part is the test mode, where it _can be_ reset.
Resetting the Glean object MUST NOT be possible in non-test usage.

### Ping upload worker

The ping upload worker is responsible for getting ping upload requests,
invoking the actual HTTP uploader
and communicating back the status of the upload.
It runs on the Android work manager.
It SHOULD NOT use another dispatcher thread.

It uses the `glean_*_upload_*` methods from the global namespace.
That interface is described in [Upload mechanism](https://mozilla.github.io/glean/dev/core/internal/upload.html)

_TODO: Update the upload mechanism docs with the new UniFFI types_

### Metric types
 
Most metric types SHOULD NOT need additional implementation in Kotlin.
As they are exposed in a different namespace we can re-export them, eventually from a share file.

```kotlin
package mozilla.telemetry.glean.private

typealias CounterMetricType = mozilla.telemetry.glean.internal.CounterMetric
```

Some more complex type might need additional work.
These should wrap the `internal` metric and add the additional state.

### Tests

Most tests that only use the (test-)public API should conceptually continue to work as is.
They will require some changes to adopt the new syntax (e.g. passing the metric info as `CommonMetricData` to constructors).

If they don't pass we first need to make sure we didn't modify behavior of Glean itself.
That's more likely than the test being wrong.
However some previously test-public methods might be gone,
so the test needs an update.

At this early stage the majority of tests does not work.
Metric type tests will only work when that metric is converted to use UniFFI.
