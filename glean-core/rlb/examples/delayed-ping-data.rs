// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{env, process};

use once_cell::sync::Lazy;

use flate2::read::GzDecoder;
use glean::{net, private::PingType, ClientInfoMetrics, ConfigurationBuilder};

pub mod glean_metrics {
    use glean::{private::CounterMetric, CommonMetricData, Lifetime};

    #[allow(non_upper_case_globals)]
    pub static sample_counter: once_cell::sync::Lazy<CounterMetric> =
        once_cell::sync::Lazy::new(|| {
            CounterMetric::new(CommonMetricData {
                name: "sample_counter".into(),
                category: "test.metrics".into(),
                send_in_pings: vec!["prototype".into()],
                disabled: false,
                lifetime: Lifetime::Ping,
                ..Default::default()
            })
        });
}

#[derive(Debug)]
struct MovingUploader(PathBuf);

impl MovingUploader {
    fn new(mut path: PathBuf) -> Self {
        path.push("sent_pings");
        std::fs::create_dir_all(&path).unwrap();
        Self(path)
    }
}

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        let upload_request = upload_request.capable(|_| true).unwrap();
        // Filter out uninteristing pings.
        if upload_request.ping_name != "prototype" {
            return net::UploadResult::http_status(200);
        }
        let net::PingUploadRequest {
            body, url, headers, ..
        } = upload_request;
        let mut gzip_decoder = GzDecoder::new(&body[..]);
        let mut s = String::with_capacity(body.len());

        let data = gzip_decoder
            .read_to_string(&mut s)
            .ok()
            .map(|_| &s[..])
            .or_else(|| std::str::from_utf8(&body).ok())
            .unwrap();

        let docid = url.rsplit('/').next().unwrap();
        let out_path = self.0.join(format!("{docid}.json"));
        let mut fp = File::create(out_path).unwrap();

        // pseudo-JSON, let's hope this works.
        writeln!(fp, "{{").unwrap();
        writeln!(fp, "  \"url\": {url},").unwrap();
        for (key, val) in headers {
            writeln!(fp, "  \"{key}\": \"{val}\",").unwrap();
        }
        writeln!(fp, "}}").unwrap();
        writeln!(fp, "{data}").unwrap();

        net::UploadResult::http_status(200)
    }
}

#[allow(non_upper_case_globals)]
pub static PrototypePing: Lazy<PingType> = Lazy::new(|| {
    PingType::new(
        "prototype",
        true,
        true,
        false,
        true,
        true,
        vec![],
        vec![],
        true,
        vec![],
    )
});

fn main() {
    env_logger::init();

    let mut args = env::args().skip(1);

    let data_path = PathBuf::from(args.next().expect("need data path"));
    let state = args.next().unwrap_or_default();

    _ = &*PrototypePing;
    let uploader = MovingUploader::new(data_path.clone());
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.pingflush")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(false)
        .with_uploader(uploader)
        .with_delay_ping_lifetime_io(true)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    glean::initialize(cfg, client_info);

    match &*state {
        "accumulate_one_and_pretend_crash" => {
            log::debug!("incrementing by 1. exiting without shutdown.");
            glean_metrics::sample_counter.add(1)
        }
        "accumulate_ten_and_orderly_shutdown" => {
            log::debug!("incrementing by 10, waiting, shutdown. should trigger a flush.");
            glean_metrics::sample_counter.add(10);
            glean::shutdown();
        }
        "submit_ping" => {
            log::info!("submitting PrototypePing");
            PrototypePing.submit(None);
            // Wait just a bit to let the ping machinery kick in and
            // ensure the ping is uploaded before we exit.
            std::thread::sleep(std::time::Duration::from_millis(100));
            glean::shutdown();
        }
        _ => {
            eprintln!("unknown argument: {state}");
            process::exit(1);
        }
    }
}
