# glean.rs

![Glean logo](docs/glean.jpeg)

_Modern Firefox Telemetry for mobile platforms_

### Overview

This repository is used to build the client-side cross-platform Telemetry library called `glean`.

The code is organized as follows:

* [./glean-core/](glean-core) contains the source for the low-level Rust library
* [./glean-core/ffi](glean-core/ffi) contains the mapping into a C FFI.
* [./glean-core/android](glean-core/android) contains the Kotlin bindings for use by Android applications.
* [./glean-core/ios](glean-core/ios) contains the Swift bindings for use by iOS applications.

This repository also hosts the [documentation](docs) for `glean.rs`
Development documenation can be found in [./docs/dev](docs/dev).
User-facing documentation can be found in [./docs/user](docs/user).
[Everything is available online](https://badboy.github.com/glean.rs).

The Rust documentation is available [online as well](https://badboy.github.com/glean.rs/docs).
