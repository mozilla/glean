// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use client_id::get_profile_id;
use tempfile::Builder;

use flate2::read::GzDecoder;
use glean::{net, ClientInfoMetrics, ConfigurationBuilder};
use log::LevelFilter;
use rustyline::error::ReadlineError;
use rustyline::Result;
use rustyline::completion::{extract_word, Completer, Pair};
use rustyline::{CompletionType, Config, Context, Editor};
use rustyline::{Helper, Highlighter, Hinter, Validator};

pub mod glean_metrics {
    include!(concat!(env!("OUT_DIR"), "/glean_metrics.rs"));
}

pub mod metric_info {
    include!(concat!(env!("OUT_DIR"), "/metric_info.rs"));
}
pub mod client_id {
    use std::{fs, path::Path};
    use uuid::Uuid;

    const CANARY_USAGE_PROFILE_ID: &str = "beefbeef-beef-beef-beef-beeefbeefbee";

    pub fn get_profile_id(path: &Path) -> Uuid {
        let uuid = if let Ok(s) = fs::read_to_string(path.join("profile_id.txt")) {
            Uuid::parse_str(&s).unwrap()
        } else {
            let uuid = Uuid::new_v4();
            write_profile_id(path, uuid);
            uuid
        };
        super::glean_metrics::metrics::profile_id.set(uuid.to_string());
        uuid
    }

    pub fn reset_profile_id(path: &Path) -> Uuid {
        let uuid = Uuid::new_v4();
        write_profile_id(path, uuid);
        super::glean_metrics::metrics::profile_id.set(uuid.to_string());
        uuid
    }

    pub fn set_canary_id(path: &Path) {
        let uuid = Uuid::parse_str(&CANARY_USAGE_PROFILE_ID).unwrap();
        write_profile_id(path, uuid);
    }

    pub fn write_profile_id(path: &Path, uuid: Uuid) {
        fs::create_dir_all(path).unwrap();
        fs::write(path.join("profile_id.txt"), uuid.to_string()).unwrap();
    }
}

#[derive(Helper, Hinter, Validator, Highlighter)]
struct CommandHelper {
    cmd_completer: CommandCompleter,
}

const DEFAULT_BREAK_CHARS: [char; 3] = [' ', '\t', '\n'];

#[derive(Hash, Debug, PartialEq, Eq)]
struct Command {
    cmd: String,
    pre_cmd: String,
}

impl Command {
    fn new(cmd: &str, pre_cmd: &str) -> Self {
        Self {
            cmd: cmd.into(),
            pre_cmd: pre_cmd.into(),
        }
    }
}
struct CommandCompleter {
    cmds: HashSet<Command>,
}

impl CommandCompleter {
    pub fn find_matches(&self, line: &str, pos: usize) -> rustyline::Result<(usize, Vec<Pair>)> {
        let (start, word) = extract_word(line, pos, None, |c| DEFAULT_BREAK_CHARS.contains(&c));
        let pre_cmd = line[..start].trim();

        let matches = self
            .cmds
            .iter()
            .filter_map(|hint| {
                if hint.cmd.starts_with(word) && pre_cmd == &hint.pre_cmd {
                    let mut replacement = hint.cmd.clone();
                    replacement += " ";
                    Some(Pair {
                        display: hint.cmd.to_string(),
                        replacement: replacement.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();
        Ok((start, matches))
    }
}

impl Completer for CommandHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> Result<(usize, Vec<Pair>)> {
        match self.cmd_completer.find_matches(line, pos) {
            Ok((start, matches)) => {
                if matches.is_empty() {
                    Ok((0, vec![]))
                } else {
                    Ok((start, matches))
                }
            }
            Err(e) => Err(e),
        }
    }
}

fn cmd_sets() -> HashSet<Command> {
    let mut set = HashSet::new();
    let toplevel_commands = &[
        "metrics",
        "pings",
        "add",
        "set",
        "get",
        "record",
        "submit",
        "upload",
        "enable",
        "disable",
        "log",
        "active",
        "inactive",
    ];
    for cmd in  toplevel_commands {
        set.insert(Command::new(cmd, ""));
    }

    for ping in metric_info::PINGS {
        set.insert(Command::new(ping, "enable"));
        set.insert(Command::new(ping, "disable"));
        set.insert(Command::new(ping, "submit"));
    }

    for metric in metric_info::METRICS {
        set.insert(Command::new(metric, "add"));
        set.insert(Command::new(metric, "set"));
        set.insert(Command::new(metric, "get"));
        set.insert(Command::new(metric, "record"));

        for ping in metric_info::PINGS {
            let precmd = format!("get {}", metric);
            set.insert(Command::new(ping, &precmd));
        }
    }

    set.insert(Command::new("off", "upload"));
    set.insert(Command::new("on", "upload"));
    set.insert(Command::new("off", "log"));
    set.insert(Command::new("on", "log"));
    set
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

fn handle_command(cmd: &str, data_path: &Path) -> std::result::Result<(), &'static str> {
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
                _ => return Err("unknown state"),
            };
            glean::set_upload_enabled(state_b);
            println!("state changed to: {}", state);
        }
        "enable" => {
            let ping = args.next().ok_or("need ping")?;
            metric_info::ping_set_enabled(ping, true);
            if ping == "usage" {
                glean_metrics::usage_deletion_request.set_enabled(true);
                client_id::reset_profile_id(data_path);
            }
        }
        "disable" => {
            let ping = args.next().ok_or("need ping")?;
            if ping == "usage" {
                glean_metrics::usage_deletion_request.submit(None);
                client_id::set_canary_id(data_path);
            }
            metric_info::ping_set_enabled(ping, false);
        }
        "log" => {
            let state = args.next().ok_or("need state: off or on")?;
            let state_b = match state {
                "on" | "true" => true,
                "off" | "false" => false,
                _ => return Err("unknown state"),
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
    get_profile_id(&data_path);

    let ping_schedule = HashMap::from([("baseline".to_string(), vec!["usage".to_string()])]);
    let uploader = MovingUploader::new(data_path.display().to_string());
    let cfg = ConfigurationBuilder::new(true, data_path.clone(), "org.mozilla.glean_core.example")
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

    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();
    let h = CommandHelper {
        cmd_completer: CommandCompleter { cmds: cmd_sets() },
    };
    let mut rl = Editor::with_config(config)?;
    rl.set_helper(Some(h));
    _ = rl.load_history(&history_path);

    metric_info::register_pings();
    glean::initialize(cfg, client_info);

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if let Err(err) = handle_command(&line, &data_path) {
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
