// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::env;

use once_cell::sync::Lazy;
use tempfile::Builder;

use glean::{metrics::PingType, ClientInfoMetrics, Configuration, Error};

#[allow(non_upper_case_globals)]
pub static PrototypePing: Lazy<PingType> =
    Lazy::new(|| PingType::new("prototype", true, true, vec![]));

fn main() -> Result<(), Error> {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        path
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().display().to_string()
    };

    let cfg = Configuration {
        data_path,
        application_id: "org.mozilla.glean_core.example".into(),
        upload_enabled: true,
        max_events: None,
        delay_ping_lifetime_io: false,
        channel: None,
    };

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
    };

    glean::initialize(cfg, client_info)?;
    glean::register_ping_type(&PrototypePing);

    if glean::submit_ping_by_name("prototype", None) {
        log::info!("Successfully collected a prototype ping");
    } else {
        log::info!("Prototype ping failed");
    }

    Ok(())
}
