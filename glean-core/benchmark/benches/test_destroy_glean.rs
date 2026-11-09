// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Benchmark the impact of `delay_ping_lifetime_io` and automatic flushing on the overall performance.

use std::fs;

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::{ClientInfoMetrics, OnGleanEvents};

#[derive(Clone)]
struct GleanEventHandler;

#[cfg(feature = "sqlite")]
const DATA: &[u8] = include_bytes!("prefilled.sqlite");
#[cfg(feature = "sqlite")]
const DB_FILE: &str = "glean.sqlite";

#[cfg(not(feature = "sqlite"))]
const DATA: &[u8] = include_bytes!("prefilled.safe.bin");
#[cfg(not(feature = "sqlite"))]
const DB_FILE: &str = "data.safe.bin";

impl OnGleanEvents for GleanEventHandler {
    fn initialize_finished(&self) {}

    fn trigger_upload(&self) -> Result<(), glean_core::CallbackError> {
        Ok(())
    }

    fn start_metrics_ping_scheduler(&self) -> bool {
        false
    }

    fn cancel_uploads(&self) -> Result<(), glean_core::CallbackError> { Ok(()) }
}

fn bench<M: criterion::measurement::Measurement>(group: &mut criterion::BenchmarkGroup<M>, delay_ping_lifetime_io: bool, clear_stores: bool) {
    let dir = tempfile::tempdir().unwrap();
    let data_path = dir.path().display().to_string();
    let cfg = glean_core::InternalConfiguration {
        upload_enabled: true,
        data_path: data_path.clone(),
        application_id: "glean-bench".into(),
        language_binding_name: "rust".into(),
        max_events: None,
        delay_ping_lifetime_io,
        app_build: "1".into(),
        use_core_mps: true,
        trim_data_to_registered_pings: true,
        log_level: None,
        rate_limit: None,
        enable_event_timestamps: true,
        experimentation_id: None,
        enable_internal_pings: true,
        ping_schedule: Default::default(),
        ping_lifetime_threshold: 0,
        ping_lifetime_max_time: 0,
        max_pending_pings_count: None,
        max_pending_pings_directory_size: None,
        session_mode: glean_core::SessionMode::Auto,
        session_sample_rate: 1.0,
        session_inactivity_timeout_ms: 1_800_000,
        events_ping_acceleration_factor: None,
        enable_store_submitted_pings: false,
        flows: vec![],
    };
    let cbs = Box::new(GleanEventHandler);

    group.bench_function(format!("delay_ping_lifetime_io={delay_ping_lifetime_io}/clear_stores={clear_stores}"), move |b| {
        b.iter_batched(|| {
            let db_path = dir.path().join("db");
            fs::create_dir_all(&db_path).unwrap();

            let db_file = db_path.join(DB_FILE);
            if db_file.exists() {
                fs::remove_file(&db_file).unwrap();
            }
            fs::write(db_file, DATA).unwrap();

            glean_core::glean_initialize(cfg.clone(), ClientInfoMetrics::unknown(), cbs.clone());
        }, |_| {
            glean_core::glean_test_destroy_glean(clear_stores, Some(data_path.clone()));
        },
        criterion::BatchSize::SmallInput)
    });
}

pub fn test_destroy_glean_bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("test_destroy_glean");

    bench(&mut group, false, false);
    bench(&mut group, false, true);
    bench(&mut group, true, false);
    bench(&mut group, true, true);

    group.finish();
}

criterion_group!(benches, test_destroy_glean_bench);
criterion_main!(benches);
