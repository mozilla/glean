use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::{
    ClientInfoMetrics, CommonMetricData, DualLabeledCounterMetric, LabeledCounter, LabeledMetricData, Lifetime, OnGleanEvents, glean_get_log_pings, glean_initialize, glean_shutdown, join_init
};

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

pub fn labeled_counter_benchmark(c: &mut Criterion) {
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

    c.bench_function("labeled_counter dynamic label", |b| {
        const N: usize = 5000;
        let keys: Vec<String> = (0..N).map(|i| format!("key{}", i)).collect();

        let metric = LabeledCounter::new(
            LabeledMetricData::Common {
                cmd: CommonMetricData {
                    name: "labeled_counter".into(),
                    category: "telemetry".into(),
                    send_in_pings: vec!["baseline".into()],
                    disabled: false,
                    lifetime: Lifetime::Ping,
                    ..Default::default()
                },
            },
            None,
        );

        b.iter(|| {
            for key in &keys {
                let m = metric.get(key);
                m.add(1);
            }

            // This ensures we drain the dispatcher.
            black_box(glean_get_log_pings());
        })
    });

    c.bench_function("dual_labeled_counter dynamic label", |b| {
        const N: usize = 5000;
        let keys: Vec<(String, String)> = (0..N)
            .map(|i| (format!("key{}", i), format!("cat{}", i)))
            .collect();

        let metric = DualLabeledCounterMetric::new(
            CommonMetricData {
                name: "dual_labeled_counter".into(),
                category: "telemetry".into(),
                send_in_pings: vec!["baseline".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            },
            None,
            None,
        );

        b.iter(|| {
            for (key, cat) in &keys {
                let m = metric.get(key, cat);
                m.add(1);
            }

            // This ensures we drain the dispatcher.
            black_box(glean_get_log_pings());
        })
    });

    glean_shutdown();
}

criterion_group!(benches, labeled_counter_benchmark);
criterion_main!(benches);
