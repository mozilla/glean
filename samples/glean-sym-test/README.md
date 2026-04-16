# A minimal sample app using `glean-sym`

## Run the sample app

```
make run
```

This runs `app.py`.
`app.py` has some self-checks to ensure things are working correctly.
If the run succeeds it's all working as expected.

You can set `RUST_LOG=debug` to get log output.

## What it does

`app.py` acts as the surrounding application.
In the real world this would be Fenix, written in Kotlin.
It loads the `xul` lib and the `services` lib.

The `xul` crate ships a dynamic library.
Inside it initializes Glean, records some metrics and submits a ping.
This is similar to what `libxul` (Gecko) does in the real world.
Because it depends on the Glean Rust SDK ("RLB")
it also does expose all the necessary symbols used by the other foreign language implementations.
Just what `glean-sym` needs.

The `services` crate also ships a dynamic library.
It acts as the equivalent of `application-services` megazord library.
It does not depend on the Glean Rust SDK.
However it does depend on the `glean-sym` crate.
Under the hood this looks up the relevant symbols from a `xul` library,
which happens to be the one that initialized Glean properly before.
`services` then creates a metric and records a value into it.
