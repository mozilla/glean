// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Micro-benchmark to measure the number of instructions it takes to launch tasks on the dispatcher.
//! Explicitly does not measure the instructions it takes to _run_ these.

#[cfg(not(feature = "gungraun"))]
fn main() {
    eprintln!("--feature gungraun required. Linux only.");
}

#[cfg(feature = "gungraun")]
use imp::*;

#[cfg(feature = "gungraun")]
use gungraun::{library_benchmark_group, main};

#[cfg(feature = "gungraun")]
mod imp {
    use std::hint::black_box;

    use glean_core::dispatcher;
    use gungraun::{
        client_requests, library_benchmark, Cachegrind, LibraryBenchmarkConfig, ValgrindTool,
    };

    fn dispatch_empty_fn(value: u64) -> u64 {
        for _ in 0..value {
            dispatcher::launch(|| {
                // intentionally left empty
            });
        }
        value
    }

    fn dispatch_fn_1024(value: u64) -> u64 {
        for _ in 0..value {
            let c = [0; 1024];
            dispatcher::launch(move || {
                black_box(&c);
            });
        }
        value
    }

    fn dispatcher_flush_init(value: u64) -> u64 {
        _ = dispatcher::flush_init();
        value
    }

    fn dispatcher_reset(_value: u64) {
        // Drain the dispatcher.
        dispatcher::block_on_queue();

        // Ensure we can run more tests afterwards.
        dispatcher::reset_dispatcher();
    }

    #[library_benchmark(
        config = LibraryBenchmarkConfig::default()
        .default_tool(ValgrindTool::Cachegrind)
        .tool(Cachegrind::with_args(["--instr-at-start=no"]))
    )]
    #[bench::one(
        args = (1),
        setup = dispatcher_flush_init,
        teardown = dispatcher_reset,
    )]
    #[bench::many(
        args = (500),
        setup = dispatcher_flush_init,
        teardown = dispatcher_reset,
    )]
    fn bench_empty_fn(value: u64) -> u64 {
        client_requests::cachegrind::start_instrumentation();
        let r = black_box(dispatch_empty_fn(value));
        client_requests::cachegrind::stop_instrumentation();
        r
    }

    #[library_benchmark(
        config = LibraryBenchmarkConfig::default()
        .default_tool(ValgrindTool::Cachegrind)
        .tool(Cachegrind::with_args(["--instr-at-start=no"]))
    )]
    #[bench::one(
        args = (1),
        setup = dispatcher_flush_init,
        teardown = dispatcher_reset,
    )]
    #[bench::many(
        args = (500),
        setup = dispatcher_flush_init,
        teardown = dispatcher_reset,
    )]
    fn bench_fn_1024(value: u64) -> u64 {
        client_requests::cachegrind::start_instrumentation();
        let r = black_box(dispatch_fn_1024(value));
        client_requests::cachegrind::stop_instrumentation();
        r
    }
}

#[cfg(feature = "gungraun")]
library_benchmark_group!(name = benches; benchmarks = bench_empty_fn, bench_fn_1024);
#[cfg(feature = "gungraun")]
main!(library_benchmark_groups = benches);
