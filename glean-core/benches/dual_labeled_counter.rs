// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::common_metric_data::CommonMetricData;
use glean_core::metrics::DualLabeledCounterMetric; // Replace with actual crate path
use glean_core::Lifetime;
use std::hint::black_box;

fn bench_dual_labeled_get(c: &mut Criterion) {
    let metric = DualLabeledCounterMetric::new(
        CommonMetricData {
            name: "test_metric".into(),
            category: "test_category".into(),
            send_in_pings: vec!["test_ping".into()],
            disabled: false,
            lifetime: Lifetime::User,
            ..Default::default()
        },
        None,
        None,
    );

    const N: usize = 10_000;
    let keys: Vec<(String, String)> = (0..N)
        .map(|i| (format!("key{}", i), format!("cat{}", i)))
        .collect();

    c.bench_function("dual_labeled_counter_get", |b| {
        b.iter(|| {
            for (k, c) in &keys {
                let m = black_box(metric.get(k, c));
                m.add(1);
            }
        })
    });
}

criterion_group!(benches, bench_dual_labeled_get);
criterion_main!(benches);
