#[cfg(feature = "gungraun")]
use std::{env, fs};

#[cfg(feature = "gungraun")]
use gungraun::{binary_benchmark, binary_benchmark_group, main};

#[cfg(feature = "gungraun")]
fn setup_directory() {
    _ = fs::remove_dir_all("tmp");
}

#[cfg(feature = "gungraun")]
#[binary_benchmark(setup = setup_directory())]
fn bench_binary() -> gungraun::Command {
    // Arguments as defined in `samples/rapid-metrics/src/main.rs`
    gungraun::Command::new(env::var("RAPID_METRICS_EXE").expect("need path in RAPID_METRICS_EXE"))
        .args(["--maxn", "500"])
        .args(["--seed", "27230"])
        .args(["--threads", "4"])
        .arg("tmp")
        .build()
}

#[cfg(feature = "gungraun")]
binary_benchmark_group!(
    name = glean;
    benchmarks = bench_binary
);

#[cfg(feature = "gungraun")]
main!(binary_benchmark_groups = glean);

#[cfg(not(feature = "gungraun"))]
fn main() {
    eprintln!("--feature gungraun required. Linux only.");
}
