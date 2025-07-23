// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::metrics::LabeledTimingDistribution;
use glean_core::Lifetime;
use glean_core::{CommonMetricData, LabeledMetricData};
use std::hint::black_box;
use std::time::Duration;

fn bench_labeled_timing_distribution_get_and_accumulate_raw_duration(c: &mut Criterion) {
    let metric = LabeledTimingDistribution::new(
        LabeledMetricData::TimingDistribution {
            cmd: CommonMetricData {
                name: "test_metric".into(),
                category: "test_category".into(),
                send_in_pings: vec!["test_ping".into()],
                disabled: false,
                lifetime: Lifetime::User,
                ..Default::default()
            },
            unit: glean_core::metrics::TimeUnit::Millisecond,
        },
        None,
    );

    const N: usize = 5000;
    let keys: Vec<String> = (0..N).map(|i| format!("key{}", i)).collect();
    let raw_duration = Duration::from_millis(1);

    c.bench_function(
        "labeled_timing_distribution_get_and_accumulate_raw_duration",
        |b| {
            b.iter(|| {
                for key in &keys {
                    let m = black_box(metric.get(key));
                    m.accumulate_raw_duration(raw_duration);
                }
            })
        },
    );
}

fn bench_labeled_timing_distribution_get_and_accumulate_single_sample(c: &mut Criterion) {
    let metric = LabeledTimingDistribution::new(
        LabeledMetricData::TimingDistribution {
            cmd: CommonMetricData {
                name: "test_metric".into(),
                category: "test_category".into(),
                send_in_pings: vec!["test_ping".into()],
                disabled: false,
                lifetime: Lifetime::User,
                ..Default::default()
            },
            unit: glean_core::metrics::TimeUnit::Millisecond,
        },
        None,
    );

    const N: usize = 5000;
    let keys: Vec<String> = (0..N).map(|i| format!("key{}", i)).collect();

    c.bench_function(
        "labeled_timing_distribution_get_and_accumulate_single_sample",
        |b| {
            b.iter(|| {
                for key in &keys {
                    let m = black_box(metric.get(key));
                    m.accumulate_single_sample(1);
                }
            })
        },
    );
}

criterion_group!(
    benches,
    bench_labeled_timing_distribution_get_and_accumulate_raw_duration,
    bench_labeled_timing_distribution_get_and_accumulate_single_sample
);
criterion_main!(benches);
