// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

pub use glean_ffi;

mod test {
    use glean::{private::CounterMetric, CommonMetricData};
    use once_cell::sync::Lazy;

    #[allow(non_upper_case_globals)]
    pub static runs: Lazy<CounterMetric> = Lazy::new(|| {
        CounterMetric::new(CommonMetricData {
            category: "test".into(),
            name: "runs".into(),
            lifetime: glean::Lifetime::Ping,
            send_in_pings: vec!["store1".into()],
            disabled: false,
            ..Default::default()
        })
    });
}

/// Increment the internal counter by `amount`.
#[no_mangle]
pub extern "C" fn increment_native_metric(amount: i32) {
    log::info!("Incrementing native counter from the Rust side");
    test::runs.add(amount);
}
