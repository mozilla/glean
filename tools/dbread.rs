#!/usr/bin/env -S cargo +nightly -q -Zscript
---cargo
package.edition = "2024"
[dependencies]
rmp-serde = "1.3.1"
rusqlite = { version = "0.37.0", features = ["bundled"] }
serde_json = "1.0.44"
tabwriter = "1.1.0"
glean-core = { path = "../glean-core" }
---

// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

//! A tiny tool to read SQLite-based databases as produced by Glean.
//!
//! Displays the metrics for every lifetime.

use std::env;
use std::io::{self, Write};

use glean_core::metrics::Metric;
use rusqlite::{Connection, OpenFlags, named_params};
use serde_json::json;
use tabwriter::TabWriter;

struct MetricRow {
    id: String,
    ping: String,
    labels: String,
    value: Metric,
}

fn main() {
    let mut args = env::args().skip(1);
    let path = args.next().expect("REQUIRED: /path/to/db");
    let flags = OpenFlags::SQLITE_OPEN_NO_MUTEX
        | OpenFlags::SQLITE_OPEN_EXRESCODE
        | OpenFlags::SQLITE_OPEN_READ_ONLY;

    let conn = Connection::open_with_flags(path, flags).unwrap();

    let stdout = io::stdout();

    let mut stmt = conn.prepare("SELECT id, ping, labels, value FROM telemetry WHERE lifetime = :lifetime ORDER BY ping ASC, id ASC").unwrap();
    for lifetime in &["user", "app", "ping"] {
        let metric_iter = stmt
            .query_map(named_params! { ":lifetime": lifetime }, |row| {
                let id = row.get(0)?;
                let ping = row.get(1)?;
                let labels = row.get(2)?;
                let value: Vec<u8> = row.get(3)?;
                Ok(MetricRow {
                    id,
                    ping,
                    labels,
                    value: rmp_serde::from_slice(&value).unwrap(),
                })
            })
            .unwrap();

        let handle = stdout.lock();
        let mut tw = TabWriter::new(handle);
        writeln!(&mut tw, "Lifetime: {lifetime}").unwrap();
        writeln!(&mut tw, "Ping\tKey\tLabels\tValue").unwrap();
        writeln!(&mut tw, "====\t===\t======\t=====").unwrap();

        for row in metric_iter {
            let row = row.unwrap();
            let value = json!({ row.value.ping_section(): row.value.as_json() });
            let value = serde_json::to_string(&value).unwrap();
            writeln!(
                &mut tw,
                "{}\t{}\t{}\t{}",
                row.ping, row.id, row.labels, value
            )
            .ok();
        }

        writeln!(&mut tw, "").ok();
        tw.flush().unwrap();
    }
}
