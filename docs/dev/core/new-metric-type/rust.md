# Adding a new metric type - Rust

## Trait

To ensure the API is stable across Rust consumers and re-exporters (like FOG),
you must define a trait for the new metric in `glean-core/src/traits`.
First, add your metric type to `mod.rs`:

```Rust
mod counter;

pub use self::counter::Counter;
```

Then add the trait in e.g.
[`counter.rs`](https://github.com/mozilla/glean/blob/HEAD/glean-core/src/traits/counter.rs).

The trait includes `test_get_num_recorded_errors`
and any metric operation included in the metric type's API
(except `new`).
The idea here is to only include operations that make sense for Rust consumers.
If there are internal-only or language-specific APIs on the underlying metric,
feel free to not include them on the trait.

Spend some time on the comments.
These will be the dev-facing API docs for Rust consumers.

## Rust Language Binding (RLB) Type

The Rust Language Binding can simply re-export the core metric types.
You can find the RLB metric re-exports in `glean-core/rlb/src/private`.

Add your metric type to `mod.rs`:

```Rust
pub use glean_core::Counter;
```

## Documentation

Don't forget to document the new Rust API in the Book's page on the Metric Type
(e.g. [Counter](../../../book/user/metrics/counter.html)).
Add a tab for Rust, and mimic any other language's example.
