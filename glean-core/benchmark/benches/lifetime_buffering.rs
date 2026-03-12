// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! Benchmark the impact of `delay_ping_lifetime_io` and automatic flushing on the overall performance.

use criterion::{Criterion, criterion_group, criterion_main};
use glean_core::{CommonMetricData, CounterMetric, Glean, Lifetime};

pub fn delay_io_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("IO delay");

    {
        let dir = tempfile::tempdir().unwrap();
        let data_path = dir.path().display().to_string();
        let cfg = glean_core::InternalConfiguration {
            upload_enabled: true,
            data_path,
            application_id: "glean-bench".into(),
            language_binding_name: "rust".into(),
            max_events: None,
            delay_ping_lifetime_io: false,
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
        };
        let glean = Glean::new(cfg).unwrap();

        let metric = CounterMetric::new(CommonMetricData {
            name: "counter".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["baseline".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
            ..Default::default()
        });

        group.bench_function("no delay", |b| {
            b.iter(|| {
                metric.add_sync(&glean, 1);
            })
        });

        assert!(metric.get_value(&glean, None).is_some());
    }

    {
        let dir = tempfile::tempdir().unwrap();
        let data_path = dir.path().display().to_string();
        let cfg = glean_core::InternalConfiguration {
            upload_enabled: true,
            data_path,
            application_id: "glean-bench".into(),
            language_binding_name: "rust".into(),
            max_events: None,
            delay_ping_lifetime_io: true,
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
        };
        let glean = Glean::new(cfg).unwrap();

        let metric = CounterMetric::new(CommonMetricData {
            name: "counter".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["baseline".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
            ..Default::default()
        });

        group.bench_function("delayed - no flush", |b| {
            b.iter(|| {
                metric.add_sync(&glean, 1);
            })
        });

        assert!(metric.get_value(&glean, None).is_some());
    }

    {
        let dir = tempfile::tempdir().unwrap();
        let data_path = dir.path().display().to_string();
        let cfg = glean_core::InternalConfiguration {
            upload_enabled: true,
            data_path,
            application_id: "glean-bench".into(),
            language_binding_name: "rust".into(),
            max_events: None,
            delay_ping_lifetime_io: true,
            app_build: "1".into(),
            use_core_mps: true,
            trim_data_to_registered_pings: true,
            log_level: None,
            rate_limit: None,
            enable_event_timestamps: true,
            experimentation_id: None,
            enable_internal_pings: true,
            ping_schedule: Default::default(),
            ping_lifetime_threshold: 1000,
            ping_lifetime_max_time: 0,
        };
        let glean = Glean::new(cfg).unwrap();

        let metric = CounterMetric::new(CommonMetricData {
            name: "counter".into(),
            category: "telemetry".into(),
            send_in_pings: vec!["baseline".into()],
            disabled: false,
            lifetime: Lifetime::Ping,
            ..Default::default()
        });

        group.bench_function("delayed - flushed after 1000", |b| {
            b.iter(|| {
                metric.add_sync(&glean, 1);
            })
        });

        assert!(metric.get_value(&glean, None).is_some());
    }

    group.finish();
}

criterion_group!(benches, delay_io_benchmark);
criterion_main!(benches);
