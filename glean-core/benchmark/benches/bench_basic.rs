use benchmark::glean_core::{metrics::*, CommonMetricData, Configuration, Glean};
use criterion::{criterion_group, criterion_main, Criterion};

/// Sets metrics and submits a custom ping.
///
/// Glean, the metrics and the custom ping are instantiated
/// before benchmarking the set/submit functionality.
pub fn criterion_benchmark(c: &mut Criterion) {
    let data_dir = tempfile::tempdir().unwrap();
    let tmpname = data_dir.path().display().to_string();
    let cfg = Configuration {
        upload_enabled: true,
        data_path: tmpname,
        application_id: "glean.bench".into(),
        language_binding_name: "Rust".into(),
        max_events: None,
        delay_ping_lifetime_io: false,
    };

    let mut glean = Glean::new(cfg).unwrap();

    let ping = PingType::new("sample", true, false, vec![]);
    glean.register_ping_type(&ping);

    let call_counter: CounterMetric = CounterMetric::new(CommonMetricData {
        name: "calls".into(),
        category: "local".into(),
        send_in_pings: vec!["sample".into()],
        ..Default::default()
    });

    let string: StringMetric = StringMetric::new(CommonMetricData {
        name: "string".into(),
        category: "local".into(),
        send_in_pings: vec!["sample".into()],
        ..Default::default()
    });

    c.bench_function("glean set and submit", |b| {
        b.iter(|| {
            call_counter.add(&glean, 1);
            string.set(&glean, "hello world");
            glean.submit_ping(&ping, None).unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
