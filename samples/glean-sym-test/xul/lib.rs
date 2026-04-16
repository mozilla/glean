// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::ffi::{CStr, c_char};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

use flate2::read::GzDecoder;
use glean::{ClientInfoMetrics, ConfigurationBuilder, net};

#[allow(clippy::all)] // Don't lint generated code.
pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

#[derive(Debug)]
struct MovingUploader {
    out_path: String,
}

impl MovingUploader {
    fn new(out_path: String) -> Self {
        Self { out_path }
    }
}

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::CapablePingUploadRequest) -> net::UploadResult {
        let upload_request = upload_request.capable(|_| true).unwrap();
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

        let mut out_path = PathBuf::from(&self.out_path);
        out_path.push("sent_pings");
        std::fs::create_dir_all(&out_path).unwrap();

        let mut components = url.rsplit('/');
        let docid = components.next().unwrap();
        let _doc_version = components.next().unwrap();
        let doctype = components.next().unwrap();
        out_path.push(format!("{doctype}-{docid}.json"));
        let mut fp = File::create(out_path).unwrap();

        // pseudo-JSON, let's hope this works.
        writeln!(fp, "{{").unwrap();
        writeln!(fp, "  \"url\": {url:?},").unwrap();
        for (key, val) in headers {
            writeln!(fp, "  \"{key}\": \"{val}\",").unwrap();
        }
        writeln!(fp, "}}").unwrap();

        let data: serde_json::Value = serde_json::from_str(data).unwrap();
        let json = serde_json::to_string_pretty(&data).unwrap();
        writeln!(fp, "{json}").unwrap();

        net::UploadResult::http_status(200)
    }
}

#[unsafe(no_mangle)]
unsafe extern "C" fn startup(path: *const c_char) {
    env_logger::init();
    log::info!("Startup invoked");

    let data_path = unsafe {
        let path = CStr::from_ptr(path);
        PathBuf::from(path.to_str().unwrap())
    };
    log::info!("Path: {}", data_path.display());

    let uploader = MovingUploader::new(data_path.display().to_string());
    let cfg = ConfigurationBuilder::new(true, data_path, "glean.sym.sample")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .with_uploader(uploader)
        .with_internal_pings(false)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    _ = &*glean_metrics::prototype;
    glean::initialize(cfg, client_info);

    glean_metrics::test_metrics::sample_counter.add(1);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn submit() {
    log::info!("Submit invoked");

    glean_metrics::prototype.submit(None);
}

#[unsafe(no_mangle)]
unsafe extern "C" fn shutdown() {
    log::info!("Shutdown invoked");
    glean::shutdown();
}
