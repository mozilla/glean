# Adding Glean to your Rust project

This page provides a step-by-step guide on how to integrate the Glean library into a Rust project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project.
Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.


## Setting up the dependency

The Glean Rust SDK is published on [crates.io](https://crates.io/crates/glean).
Add it to your dependencies in `Cargo.toml`:

```toml
[dependencies]
glean = "50.0.0"
```

## Setting up metrics and pings code generation

At build time you need to generate the metrics and ping API from your definition files.
Add the `glean-build` crate as a build dependency in your `Cargo.toml`:

```toml
[build-dependencies]
glean-build = "50.0.0"
```

Then add a `build.rs` file next to your `Cargo.toml` and call the builder:

```Rust
use glean_build::Builder;

fn main() {
    Builder::default()
        .metrics("metrics.yaml")
        .pings("pings.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");
}
```

Ensure your `metrics.yaml` and `pings.yaml` files are placed next to your `Cargo.toml` or adjust the path in the code above.
You can also leave out any of the files.

### Include the generated code

`glean-build` will generate a `glean_metrics.rs` file that needs to be included in your source code.
To do so add the following lines of code in your `src/lib.rs` file:

```Rust
mod metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}
```

Alternatively create `src/metrics.rs` (or a different name) with only the include line:

```Rust
include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
```

Then add `mod metrics;` to your `src/lib.rs` file.

### Use the metrics

In your code you can then access generated metrics nested within their category under the `metrics` module (or your chosen name):

```Rust
metrics::your_category::metric_name.set(true);
```

See [the metric API reference](../../reference/metrics/index.md) for details.
