// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Micro-benchmark to measure the time it takes to launch tasks on the dispatcher.
//! Explicitly does not measure the time it takes to _run_ these.

use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::{
    dispatcher, glean_initialize, glean_shutdown, join_init, ClientInfoMetrics, CommonMetricData,
    CounterMetric, Lifetime, OnGleanEvents,
};

pub fn dispatcher_benchmark(c: &mut Criterion) {
    // Ensure dispatcher has been created and is usable.
    dispatcher::flush_init().unwrap();

    // An empty function has size 0. Putting that on the queue should be quick.
    c.bench_function("empty fn", |b| {
        b.iter(|| {
            dispatcher::launch(|| {
                // intentionally left empty
            });
        })
    });

    // Drain the dispatcher.
    dispatcher::block_on_queue();

    // Moving data into the function means we need to copy that data on to the queue too.
    c.bench_function("fn 1024", |b| {
        b.iter(|| {
            let c = [0; 1024];
            dispatcher::launch(move || {
                black_box(c);
            });
        })
    });

    // Ensure we can run more tests afterwards.
    dispatcher::reset_dispatcher();
}

struct Callbacks;

impl OnGleanEvents for Callbacks {
    fn initialize_finished(&self) {}

    fn trigger_upload(&self) -> glean_core::Result<(), glean_core::CallbackError> {
        Ok(())
    }

    fn start_metrics_ping_scheduler(&self) -> bool {
        false
    }

    fn cancel_uploads(&self) -> glean_core::Result<(), glean_core::CallbackError> {
        Ok(())
    }
}

pub fn metric_dispatcher_benchmark(c: &mut Criterion) {
    let dir = tempfile::tempdir().unwrap();
    let data_path = dir.path().display().to_string();

    let cfg = glean_core::InternalConfiguration {
        upload_enabled: true,
        data_path,
        application_id: String::from("glean-bench"),
        language_binding_name: String::from("rust"),
        max_events: None,
        delay_ping_lifetime_io: false,
        app_build: String::from("1"),
        use_core_mps: true,
        trim_data_to_registered_pings: true,
        log_level: None,
        rate_limit: None,
        enable_event_timestamps: true,
        experimentation_id: None,
        enable_internal_pings: false,
        ping_schedule: Default::default(),
        ping_lifetime_threshold: 0,
        ping_lifetime_max_time: 0,
    };
    let client_info = ClientInfoMetrics::unknown();

    glean_initialize(cfg, client_info, Box::new(Callbacks));
    join_init();

    c.bench_function("counter.add", |b| {
        let metric = CounterMetric::new(CommonMetricData {
            name: "counter".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["baseline".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
            ..Default::default()
        });

        b.iter(|| {
            metric.add(1);
        })
    });

    glean_shutdown();
}

criterion_group!(benches, dispatcher_benchmark, metric_dispatcher_benchmark);
criterion_main!(benches);
