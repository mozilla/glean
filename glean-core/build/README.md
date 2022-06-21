# Glean SDK

The `Glean SDK` is a modern approach for a Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## `glean-build`

This library provides a code generator for use in `build.rs`.

## Documentation

The Glean SDK documentation is available online:

* [The Glean SDK Book][book]

The `glean-build` API is documented online:

* [API Documentation][apidocs]

[book]: https://mozilla.github.io/glean/
[apidocs]: https://docs.rs/glean-build/

## Requirements

* Python 3
* pip

## Usage

Add `glean-build` as a build dependency to your project in `Cargo.toml`:

```
[build-dependencies]
glean-build = "6.0.1"
```

Then add a `build.rs` file next to your `Cargo.toml` and call the builder:

```rust,no_run
use glean_build::Builder;

fn main() {
    Builder::default()
        .file("metrics.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");
}
```

In your code add the following to include the generated code:

```rust,ignore
mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}
```

You can then access your metrics and pings directly by name within the `metrics` module.

## License

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/
