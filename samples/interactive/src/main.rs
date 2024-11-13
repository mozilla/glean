// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use tempfile::Builder;

use flate2::read::GzDecoder;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder};
use log::LevelFilter;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

pub mod metric_info {
    include!(concat!(env!("OUT_DIR"), "/metric_info.rs"));
}

#[derive(Debug)]
struct MovingUploader {
    out_path: String,
    recv_cnt: AtomicUsize,
}

impl MovingUploader {
    fn new(out_path: String) -> Self {
        let mut cnt = 0;

        let mut dir = PathBuf::from(&out_path);
        dir.push("sent_pings");
        if let Ok(entries) = dir.read_dir() {
            cnt = entries.filter_map(|entry| entry.ok()).count();
        }

        Self {
            out_path,
            recv_cnt: AtomicUsize::new(cnt),
        }
    }
}

impl net::PingUploader for MovingUploader {
    fn upload(&self, upload_request: net::PingUploadRequest) -> net::UploadResult {
        let cnt = self.recv_cnt.fetch_add(1, Ordering::Relaxed) + 1;
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
        out_path.push(format!("{cnt:0>3}-{doctype}-{docid}.json"));
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

        println!("\nReceived '{}' ping. Document ID: {}", doctype, docid);

        net::UploadResult::http_status(200)
    }
}

fn handle_command(cmd: &str) -> std::result::Result<(), &'static str> {
    let mut args = cmd.split(" ");

    match args.next().unwrap() {
        "metrics" => {
            println!("available metrics:");
            for m in metric_info::METRICS {
                println!("- {}", m);
            }
        }
        "pings" => {
            println!("available pings:");
            for m in metric_info::PINGS {
                println!("- {}", m);
            }
        }
        "add" => {
            let metric = args.next().ok_or("need metric name")?;
            let amount: i32 = args
                .next()
                .ok_or("need amount")?
                .parse()
                .map_err(|_| "need integer amount")?;
            metric_info::counter_add(metric, amount);
        }
        "set" => {
            let metric = args.next().ok_or("need metric name")?;
            let val: bool = args
                .next()
                .ok_or("need value")?
                .parse()
                .map_err(|_| "need bool")?;
            metric_info::boolean_set(metric, val);
        }
        "get" => {
            let metric = args.next().ok_or("need metric name")?;
            let ping = args.next().map(|s| s.to_string());
            metric_info::metric_get(metric, ping);
        }
        "record" => {
            let metric = args.next().ok_or("need metric name")?;
            metric_info::event_record(metric);
        }
        "submit" => {
            let ping = args.next().ok_or("need ping name")?;
            metric_info::ping_submit(ping);
        }
        "upload" => {
            let state = args.next().ok_or("need state: off or on")?;
            let state_b = match state {
                "on" | "true" => true,
                "off" | "false" => false,
                _ => return Err("unknown state")
            };
            glean::set_upload_enabled(state_b);
            println!("state changed to: {}", state);
        }
        "enable" => {
            let ping = args.next().ok_or("need ping")?;
            glean::set_ping_enabled(ping, true);
            println!("{} enabled", ping);
        }
        "disable" => {
            let ping = args.next().ok_or("need ping")?;
            glean::set_ping_enabled(ping, false);
            println!("{} disabled", ping);
        }
        "log" => {
            let state = args.next().ok_or("need state: off or on")?;
            let state_b = match state {
                "on" | "true" => true,
                "off" | "false" => false,
                _ => return Err("unknown state")
            };
            glean::set_log_pings(state_b);
            println!("log state changed to: {}", state);
        }
        "active" => {
            glean::handle_client_active();
        }
        "inactive" => {
            glean::handle_client_inactive();
        }
        other => {
            if !other.is_empty() {
                println!("E: Unknown command {other}");
            }
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter(Some("rustyline"), LevelFilter::Off)
        .init();

    let mut args = env::args().skip(1);

    let data_path = if let Some(path) = args.next() {
        PathBuf::from(path)
    } else {
        let root = Builder::new().prefix("simple-db").tempdir().unwrap();
        root.path().to_path_buf()
    };
    let history_path = data_path.join("history.txt");

    let ping_schedule = HashMap::from([
        ("baseline".to_string(), vec!["usage".to_string()])
    ]);
    let uploader = MovingUploader::new(data_path.display().to_string());
    let cfg = ConfigurationBuilder::new(true, data_path, "org.mozilla.glean_core.example")
        .with_server_endpoint("invalid-test-host")
        .with_use_core_mps(true)
        .with_uploader(uploader)
        .with_ping_schedule(ping_schedule)
        .build();

    let client_info = ClientInfoMetrics {
        app_build: env!("CARGO_PKG_VERSION").to_string(),
        app_display_version: env!("CARGO_PKG_VERSION").to_string(),
        channel: None,
        locale: None,
    };

    let mut rl = DefaultEditor::new()?;
    _ = rl.load_history(&history_path);

    metric_info::register_pings();
    glean::initialize(cfg, client_info);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if let Err(err) = handle_command(&line) {
                    println!("E: {}", err);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    glean::shutdown();

    rl.save_history(&history_path)?;
    Ok(())
}
