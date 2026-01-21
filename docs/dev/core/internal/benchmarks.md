# Benchmarks

The Glean SDK currently has only a very limited set of benchmarks.
This will expand over time.

## Notes

Benchmark results cannot be directly compared across different machines.
Absolute numbers from the benchmarks are not useful.
The relative performance against a baseline is the interesting part.

We currently don't run these benchmarks automatically and don't track changes.

## Rust benchmarks

We use [criterion] for micro-benchmarking of the Rust code directly.

[criterion]: https://bheisler.github.io/criterion.rs/book/index.html

You can run all benchmarks using:

```
cargo benchmark
```

To run an individual benchmark pass its name after `--bench`:

```
cargo benchmark --bench dispatcher
```

To add a new benchmark create a file in `glean-core/benchmark/benches`, then add a new entry to the `glean-core/benchmark/Cargo.toml` file:

```toml
[[bench]]
name = "name-of-the-benchmark"
harness = false
```

We also provide a helper script to make comparison of changes against a previous baseline easy.

The following will first checkout the `main` branch, run all benchmarks on the `main` branch,
then go back to your current `HEAD` commit and run the benchmarks again, providing you with a comparison at the end.

```
bin/benchmark-compare.sh main HEAD
```

The output will be similar to this:

```
   Compiling glean-core v66.3.0 (/home/user/src/glean/glean-core)
   Compiling benchmark v0.1.0 (/home/user/jer/src/glean/glean-core/benchmark)
    Finished `bench` profile [optimized] target(s) in 18.62s
     Running benches/dispatcher.rs (glean-core/benchmark/target/release/deps/dispatcher-a44e0c186f62af70)
Gnuplot not found, using plotters backend
empty fn                time:   [61.181 ns 61.231 ns 61.285 ns]
                        change: [−23.764% −23.575% −23.390%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

fn 1024                 time:   [431.57 ns 432.26 ns 432.99 ns]
                        change: [−44.413% −34.742% −23.066%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
```

It will also provide you with a command to re-run the benchmark to compare against the `main` baseline.
