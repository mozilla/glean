// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[unsafe(no_mangle)]
unsafe extern "C" fn record(amount: i32) {
    env_logger::init();
    log::info!("Record invoked");

    log::info!("new LoginStore! Recording a metric");
    glean_metrics::dylib::counting.add(amount);
    log::info!("Metric recorded.");
}
