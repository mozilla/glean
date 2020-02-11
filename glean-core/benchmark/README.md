# Glean Benchmarks

The `Glean SDK` is a modern approach for a Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## Benchmarks

This crates provides simple benchmarks for Glean, based on the [criterion benchmark framework](https://bheisler.github.io/criterion.rs/book/criterion_rs.html).
The library itself does not contain additional code.

### Available benchmarks

* [`benches/bench_basic.rs`](benches/bench_basic.rs) - Setting metrics and submitting a custom ping

### How to run the benchmarks

From the top-level directory of the repository run:

```
cargo bench -p benchmark
```

This is also available as a `make` task:

```
make bench-rust
```

Any additional run compares results to the preceding run.

### Results

After running the benchmarks, an HTML-rendered report can be found in `target/criterion/report/index.html`.
We do not provide any results here as the absolute numbers are unreliable.
Benchmark timings across code changes are more important.

### Why an additional crate?

This way we don't add any new (dev) dependencies to the crates that get released.
