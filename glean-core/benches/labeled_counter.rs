// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use criterion::{criterion_group, criterion_main, Criterion};
use glean_core::metrics::{LabeledCounter, LabeledMetricData};
use glean_core::CommonMetricData;
use glean_core::Lifetime;
use std::hint::black_box;

fn bench_labeled_counter_get_and_add(c: &mut Criterion) {
    let metric = LabeledCounter::new(
        LabeledMetricData::Common {
            cmd: CommonMetricData {
                name: "test_metric".into(),
                category: "test_category".into(),
                send_in_pings: vec!["test_ping".into()],
                disabled: false,
                lifetime: Lifetime::User,
                ..Default::default()
            },
        },
        None,
    );

    const N: usize = 5000;
    let keys: Vec<String> = (0..N).map(|i| format!("key{}", i)).collect();

    c.bench_function("labeled_counter_get_and_add", |b| {
        b.iter(|| {
            for key in &keys {
                let m = black_box(metric.get(key));
                m.add(1);
            }
        })
    });
}

criterion_group!(benches, bench_labeled_counter_get_and_add);
criterion_main!(benches);
