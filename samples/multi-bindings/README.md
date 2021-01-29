# Multi-binding example

This example showcases the interplay of using a [megazord] build of a Rust library, the Glean RLB and glean-ffi,
and another language that makes use of the exported Glean FFI.

This is conceptually similar to an Android application, where the Kotlin part wraps the Glean FFI coming from a `libmegazord`.
`libmegazord` is a single dynamic library consisting of multiple components, including `glean_ffi`.
Symbols from `glean_ffi` are forward-exported from `libmegazord`.
Internally the Rust code uses the Glean Rust language bindings to record metrics.

Data from both the Rust side and the other implementation land in the same single Glean instance.
Our "other implementation" here is a small C app instead of a full blown Kotlin application.

## Build

The example consists of 2 parts:

1. A Rust library `megazord`, compiled as a dynamic library (`cdylib`).
    * The Rust library depends on `glean_ffi` and re-exports those symbols.
    * It also depends on `glean` (RLB) and uses that internally.
2. A small C application that uses the Glean FFI to initialize Glean, record metrics and submit a custom ping.

To build in debug mode:

```
make debug
```

To build in release mode:

```
make release
```

## Run

The application will

1. Initialize Glean.
2. Record some metrics.
3. Submit a ping.
4. Will act as a ping uploader (without uploading any data)

It will exit with exit code `0` if all goes right.
Otherwise it will abort with another exit code. If an assertion failed a message will be printed.
The data and pending pings are put into `./tmp/`.
If the program fails you will need to clear that directory before the next run.

To run in debug mode:

```
make run
```

To run in release mode:

```
make run-release
```

[megazord]: https://github.com/mozilla/application-services/blob/043d8a317b86390113d10115d9dfee23de49e15c/docs/design/megazords.md
